mod json;
mod lex;

use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.json").unwrap();
    let tokens = lex::tokenize(String::from(input)).unwrap();
    let json = lex::parse_tokens(tokens).unwrap();
    let end = Instant::now();
    let dur = end.duration_since(start);
    dbg!(json);
    println!("It took {:?} to parse", dur);
}
