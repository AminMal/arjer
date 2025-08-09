use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.json").unwrap();
    let start = Instant::now();
    let json = arjer::parse(input.as_str()).unwrap();
    let end = Instant::now();
    let dur = end.duration_since(start);
    println!("{}", json.pretty_print());
    println!("{}", json.indent("\t"));
    println!("It took {:?} to parse json file", dur);
    // println!("----------");
    // let job_title = json.cursor().obj("job").string("title").unwrap();
    // println!("job title is {}", job_title);
    // let self_employed = json.cursor().obj("job").boolean("self_employed").unwrap();
    // println!("is self employed: {}", self_employed);
    // let no_such_boolean = json.cursor().obj("job").boolean("self_employed_non");
    // println!("no such boolean error: {}", no_such_boolean.err().unwrap())
}
