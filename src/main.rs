#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for pattern")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("file to search")
            .takes_value(true)
            .required(false))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");
    let mut filename = String::new(); 
    if input == "-" {
        let stdin = io::stdin();
        let _ = stdin.read_line(&mut filename);
        filename.remove(filename.len() - 1);
    }

    if filename.is_empty() {
        filename = input.to_string();
    }
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    process_lines(reader, re);
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn grep_command_line() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for pattern")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("file to search")
            .takes_value(true)
            .required(true))
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let filename = args.value_of("input").unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn regex_search() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let quote = "\
Every face, every shop,
bedroom, window, public-house, and 
dark square is a picture 
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";
    
    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn find_with_context() {
    let ctx_lines = 2;
    let needle = "oo";
    
    let haystack = "\
Every face, every shop,
bedroom, window, public-house, and 
dark square is a picture 
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
        }
        let v = Vec::with_capacity(2 * ctx_lines + 1);
        ctx.push(v);
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
// Pg 99: Reading from files
