use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut total = 0;

    let file = File::open("input").expect("Can't read file");

    let reader = BufReader::new(file);
    let mut current;
    for line in reader.lines() {
        current = (line.expect("error reading line").trim().parse::<f64>().expect("can't parse")/3.).floor() - 2.;
        while current > 0. {
            total += current as i64;
            current = current/3. - 2.
        }
    }
    println!("Total: {}", total);
}
