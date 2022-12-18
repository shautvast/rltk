#![warn(clippy::all, clippy::pedantic)]

use std::{env, io, process, thread};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use chrono::{DateTime, Local};
use crossbeam_channel::bounded;
use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;

use crate::worker::ThreadPool;

const BUFFER_SIZE: usize = 10_000;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: clean [whitelist-filename] input-filename");
        process::exit(-1);
    }

    let input_filename = if args.len() == 3 {
        &args[2]
    } else {
        &args[1]
    };

    let start: DateTime<Local> = Local::now();

    let file = File::open(input_filename)?;
    let reader = io::BufReader::new(file);

    let mut worker_pool = ThreadPool::new(4); // TODO turn into cmdline argument
    let (merge_sender, merge_receiver) = bounded::<Option<Vec<String>>>(8);

    {
        thread::spawn(move || {
            loop {
                // handle Error case silently
                let maybe_buffer = merge_receiver.recv().unwrap_or(None);
                if let Some(clean_buffer) = maybe_buffer {
                    for line in clean_buffer {
                        println!("{}", line);
                    }
                } else {
                    // None is the break signal (may be made nicer using enum)
                    break;
                }
            }
        });
    }

    let mut line_count = 0usize;
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

    worker_pool.wait();

    eprintln!("took {} seconds", Local::now().signed_duration_since(start));
    Ok(())
}

fn handle_line_buffer(buffer: Vec<String>, sender: &crossbeam_channel::Sender<Option<Vec<String>>>) {
    lazy_static! {
        static ref args: Vec<String> = env::args().collect();
        static ref w:Vec<String> = if args.len() == 3 {
            let whitelist_filename = &args[1];
                read_whitelist(whitelist_filename).unwrap()
            } else {
                read_std_whitelist().unwrap()
        };
        static ref WHITELIST:Vec<&'static str> = w.iter().map(|s| s.as_str()).collect::<Vec<&'static str>>();
    }
    let mut clean_buffer = vec![];
    for line in buffer {
        let mut clean_line = String::new();
        line.graphemes(true).filter(|g| WHITELIST.contains(g)).for_each(|c| clean_line.push_str(c));
        clean_buffer.push(clean_line);
    }
    sender.send(Some(clean_buffer)).unwrap();
}

fn read_whitelist(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut whitelist = vec![];
    for line in reader.lines().flatten() {
        line.graphemes(true).for_each(|token| whitelist.push(token.to_owned()));
    }

    Ok(whitelist)
}

fn read_std_whitelist() -> Result<Vec<String>, io::Error> {
    let whitelist_dat = include_str!("../../dat/default_whitelist.dat");
    let mut whitelist = vec![];

    whitelist_dat.graphemes(true).for_each(|token| whitelist.push(token.to_owned()));

    Ok(whitelist)
}