use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input.json").unwrap();
    let json = arjer::parse(input).unwrap();
    let end = Instant::now();
    let dur = end.duration_since(start);
    dbg!(&json);
    println!("It took {:?} to core", dur);
    println!("----------");
    let job_title = json.cursor().obj("job").string("title").unwrap();
    println!("job title is {}", job_title);
    let self_employed = json.cursor().obj("job").boolean("self_employed").unwrap();
    println!("is self employed: {}", self_employed);
    let no_such_boolean = json.cursor().obj("job").boolean("self_employed_non");
    println!("no such boolean error: {}", no_such_boolean.err().unwrap())
}
