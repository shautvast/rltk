#![warn(clippy::all, clippy::pedantic)]

use std::{env, process};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::prelude::*;
use crossbeam_channel::bounded;

use crate::worker::ThreadPool;

const BUFFER_SIZE: usize = 10_000;

/// Creates counts for single words in a text (file). These can be used as preprocessing for (sub)word tokenization
/// Writes the result to a new file.
///
/// removes all interpunction and special characters
///
/// Uses a fork-join pattern using 8 threads (can be changed in the code).
///
pub fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: tokenize [filename]");
        process::exit(-1);
    }

    let input_filename = &args[1];
    let start: DateTime<Local> = Local::now();

    eprintln!("started at {:?}", start);
    let file = File::open(input_filename)?;
    let reader = io::BufReader::new(file);

    // counter for determining if the buffer is full (and ready to be passed to a worker thread)
    let mut line_count = 0usize;

    let mut worker_pool = ThreadPool::new(4); // TODO turn into cmdline argument

    // this is the end result before writing to file
    let merge_counter = Arc::new(Mutex::new(HashMap::new()));

    // bounded channel for sending intermediate results (counts for a single buffer) to the merger
    // let (merge_sender, merge_receiver) = crossbeam_channel::bounded::<Option<HashMap<String, usize>>>(8); //#threads
    let (merge_sender, merge_receiver) = bounded::<Option<HashMap<String, usize>>>(8);
    // crossbeam_channel::bounded::<Option<HashMap<String, usize>>>(8); //#threads

    // create new scope for the merge_counter clone
    {
        let merge_counter = Arc::clone(&merge_counter);
        // create a single thread that waits for intermediate counts to process them in the overall result
        thread::spawn(move || {
            loop {
                // handle Error case silently
                let maybe_buffer = merge_receiver.recv().unwrap_or(None);
                if let Some(counter) = maybe_buffer {
                    // create a lock on the merge_counter (this is the only thread while processing). The main thread will read later
                    let mut merge_counter = merge_counter.lock().unwrap();

                    // update counts and discard intermediate result
                    for (word, count) in counter {
                        let entry = merge_counter.entry(word).or_insert(0);
                        *entry += count;
                    }
                } else {
                    // None is the break signal (may be made nicer using enum)
                    break;
                }
            }
        });
    }

    // read a file of text with line separators
    // when the buffer is full, pass it to a handle_line_buffer function
    let mut buffer = Vec::with_capacity(BUFFER_SIZE);
    for line in reader.lines().flatten() {
        if line_count < BUFFER_SIZE {
            buffer.push(line);
            line_count += 1;
        } else {
            // multi producer, single consumer. So multiple senders, using clone
            let merge_sender = merge_sender.clone();
            worker_pool.execute(move || handle_line_buffer(buffer, &merge_sender));
            buffer = Vec::with_capacity(BUFFER_SIZE);
            line_count = 0;
        }
    }
    // last, partially filled buffer
    worker_pool.execute(move || handle_line_buffer(buffer, &merge_sender));

    // wait for processing to finish
    worker_pool.wait();

    for (word, count) in merge_counter.lock().unwrap().iter() {
        // cutoff is 5 (inclusive min word count)
        // TODO parametrize
        if *count > 4 {
            println!("{}: {}", word, count);
        }
    }

    eprintln!("took {} seconds", Local::now().signed_duration_since(start));
    Ok(())
}

fn handle_line_buffer(buffer: Vec<String>, sender: &crossbeam_channel::Sender<Option<HashMap<String, usize>>>) {
    let mut counter = HashMap::new();

    for line in buffer {
        for word in line.split(&[' ', ',', '.', ';']) {
            let count = counter.entry(word.to_lowercase().to_owned()).or_insert(0);
            *count += 1;
        }
    }

    sender.send(Some(counter)).unwrap();
}

