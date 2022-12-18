#![warn(clippy::all, clippy::pedantic)]

use std::{env, io, process, thread};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
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
        static ref SUBST:HashMap<String, String> = if args.len() == 3 {
            let whitelist_filename = &args[1];
                read_whitelist(whitelist_filename).unwrap()
            } else {
                read_std_whitelist().unwrap()
        };
        static ref WHITELIST:HashMap<&'static str, &'static str> = SUBST.iter().map(|(s,t)| (s.as_str(),t.as_str())).collect::<HashMap<&'static str, &'static str>>();
    }
    let mut clean_buffer = vec![];
    for line in buffer {
        let mut clean_line = String::new();
        line.graphemes(true).map(|g|*(SUBST.get(g)).or_else(||Some(g.to_owned()))).for_each(|c| clean_line.push_str(c.unwrap()));
        clean_buffer.push(clean_line);
    }

    sender.send(Some(clean_buffer)).unwrap();
}

fn read_whitelist(filename: &String) -> Result<HashMap<String, String>, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut substitutions = HashMap::new();

    for line in reader.lines().flatten() {
        let key_value = line.split(":").map(|e|e.to_owned()).collect::<Vec<String>>();
        let source = key_value[0].clone();
        let dest = key_value[1].clone();
        source.graphemes(true).for_each(|token| { substitutions.insert(token.to_owned(), dest.clone()); });
    }

    Ok(substitutions)
}

fn read_std_whitelist() -> Result<HashMap<String, String>, io::Error> {
    let substitute_dat = include_str!("../../dat/default_substitute.dat");

    let mut substitutions = HashMap::new();

    for line in substitute_dat.split("\n") {
        let key_value = line.split(":").map(|e|e.to_owned()).collect::<Vec<String>>();
        let source = key_value[0].clone();
        let dest = key_value[1].clone();
        source.graphemes(true).for_each(|token| { substitutions.insert(token.to_owned(), dest.clone()); });
    }

    Ok(substitutions)
}