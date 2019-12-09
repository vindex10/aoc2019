use std::fs::File;
use std::io::Result;
use std::io::Read;

fn main() -> Result<()> {
    let mut file = File::open("input")?;
    let mut line = String::new();
    file.read_to_string(&mut line)?;

    let mut count_zeros = 0;
    let mut res = 0;
    let mut min_zeros = 25*6+1;
    let mut count_ones = 0;
    let mut count_twos = 0;

    let mut i = 0;
    for ch in line.chars() {
        match ch.to_digit(10) {
            Some(0) => {count_zeros += 1},
            Some(1) => {count_ones += 1},
            Some(2) => {count_twos += 1},
            Some(_) => {},
            None => {}
        }

        if i == 25*6 - 1 {
            if count_zeros < min_zeros {
                res = count_ones*count_twos;
                min_zeros = count_zeros;
            }
            count_ones = 0;
            count_twos = 0;
            count_zeros = 0;
        }

        i = (i + 1) % (25*6);
    }

    println!("{}", res);

    Ok(())
}
