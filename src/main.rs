use std::fs;
use std::time::Instant;

use arjer::parser;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.json").unwrap();
    let json = parser::parse(input).unwrap();
    let end = Instant::now();
    let dur = end.duration_since(start);
    dbg!(json);
    println!("It took {:?} to core", dur);
}
