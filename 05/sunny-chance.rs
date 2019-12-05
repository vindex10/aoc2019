use std::fs::File;
use std::io::{prelude::*, Result};
use std::io;

fn parse_args(line:&Vec::<i32>, instruction_i:usize, num_args:i8) -> Vec::<i32> {
    let mut res = Vec::<i32>::new();
    let mut arg_counter = 0;

    let mut buf = line[instruction_i]/100;
    let mut idx;
    while buf != 0 {
        arg_counter += 1;
        idx = line[(instruction_i + arg_counter) as usize] as usize;
        if buf % 10 == 0 {
            res.push(line[idx] as i32);
        } else {
            res.push(idx as i32);
        }
        buf = buf / 10;
    }
    while (arg_counter as i8) < num_args {
        arg_counter += 1;
        idx = line[(instruction_i + arg_counter) as usize] as usize;
        res.push(line[idx] as i32);
    }
    return res;
}

fn main() -> Result<()> {
    let mut file = File::open("input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut line: Vec::<i32> = Vec::new();
    for i in input.trim().split(",") {
        let parsed = i.parse::<i32>();
        match parsed {
            Ok(num) => line.push(num),
            Err(_) => continue
        }
    }
    line.resize(10000, 99);

    let max = line.len();
    let mut i = 0;
    let mut cmd;
    let mut args;
    let mut val = String::new();
    let mut to;
    while i < max {
        cmd = line[i] % 100;
        println!("{}, {}", i, cmd);
        if cmd == 1 {
            args = parse_args(&line, i, 2);
            to = line[i+3] as usize;
            println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] = args[1] + args[0];
            println!("{}: {}", to, line[to]);
            i += 4;
        } else if cmd == 2 {
            args = parse_args(&line, i, 2);
            to = line[i+3] as usize;
            println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] = args[1] * args[0];
            println!("{}: {}", to, line[to]);
            i += 4;
        } else if cmd == 3 {
            to = line[i+1] as usize;
            println!("{}, {}", cmd, to);
            println!("Enter input value: ");
            io::stdin().read_line(&mut val).unwrap();
            line[to] = val.trim().parse::<i32>().unwrap();
            println!("{}: {}", to, line[to]);
            i += 2;
        } else if cmd == 4 {
            args = parse_args(&line, i, 1);
            println!("{}", args[0]);
            i += 2;
            println!("{}, {}", cmd, args[0]);
        } else if cmd == 5 {
            args = parse_args(&line, i, 2);
            println!("{}, {}, {}", cmd, args[0], args[1]);
            if args[0] != 0 {
                i = args[1] as usize;
            } else {
                i += 3;
            }
        } else if cmd == 6 {
            args = parse_args(&line, i, 2);
            println!("{}, {}, {}", cmd, args[0], args[1]);
            if args[0] == 0 {
                i = args[1] as usize;
            } else {
                i += 3;
            }
        } else if cmd == 7 {
            to = line[i+3] as usize;
            args = parse_args(&line, i, 2);
            println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] =  if args[0] < args[1] { 1 } else { 0 }; 
            i += 4;
            println!("{}: {}", to, line[to]);
        } else if cmd == 8 {
            to = line[i+3] as usize;
            args = parse_args(&line, i, 2);
            println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] =  if args[0] == args[1] { 1 } else { 0 }; 
            i += 4;
            println!("{}: {}", to, line[to]);
        } else if cmd == 99 {
            println!("halt!");
            break;
        } else {
            println!("Errorr!!! Command not matched: {}", cmd);
            break;
        }
        //println!("next cmd!");
        //io::stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}
