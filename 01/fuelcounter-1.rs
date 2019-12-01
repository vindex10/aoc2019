use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut total = 0;

    let file = File::open("input").expect("Can't read file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        total += ((line.expect("error reading line").trim().parse::<f64>().expect("can't parse")/3.).trunc() as i64) - 2;
    }
    println!("Total: {}", total);
}
