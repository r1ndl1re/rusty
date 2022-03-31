use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

/// Searches for patterns
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// regex pattern
    #[clap(short, long)]
    pattern: String,

    /// input file
    #[clap(short, long, default_value = "-")]
    input: String,
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

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;
    let re = Regex::new(&pattern).unwrap();

    let input = args.input;

    if input == "-" {
        let stdin = stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
