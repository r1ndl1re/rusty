use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Searches for patterns
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// regex pattern
    #[clap(short, long)]
    pattern: String,

    /// input file
    #[clap(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;
    let re = Regex::new(&pattern).unwrap();

    let input = args.input;
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
