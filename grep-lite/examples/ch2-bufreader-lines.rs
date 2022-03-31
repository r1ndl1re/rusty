use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("Cargo.toml").unwrap();
    let reader = BufReader::new(f);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
