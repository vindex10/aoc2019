use std::fs::File;
use std::io::{prelude::*, Result};

fn test(noun:i32, verb:i32) -> Result<i32> {
    let mut file = File::open("input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut line: Vec<i32> = Vec::new();
    for i in input.trim().split(",") {
        let parsed = i.parse::<i32>();
        match parsed {
            Ok(num) => line.push(num),
            Err(_) => continue
        }
    }
    line[1] = noun;
    line[2] = verb;

    let max = line.len();
    let mut i = 0;
    while i < max {
        if line[i] == 1 {
            let idx1 = line[i+3] as usize;
            let idx2 = line[i+2] as usize;
            let idx3 = line[i+1] as usize;
            line[idx1] = line[idx2] + line[idx3];
            i += 4;
        } else if line[i] == 2 {
            let idx1 = line[i+3] as usize;
            let idx2 = line[i+2] as usize;
            let idx3 = line[i+1] as usize;
            line[idx1] = line[idx2] * line[idx3];
            i += 4;
        } else if line[i] == 99 {
            break;
        }
    }

    return Ok(line[0]);
}

fn main() -> Result<()> {
    for i in 0..99 {
        for j in 0..99 {
            let test_val = match test(i, j) {
                Ok(num) => num,
                Err(_) => continue
            };
            if test_val == 19690720 {
                println!("{}", 100*i + j);
                return Ok(());
            }
        }
    }
    
    return Ok(());
}
