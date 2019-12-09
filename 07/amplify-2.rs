extern crate itertools;

use std::fs::File;
use std::io::{prelude::*, Result};
use std::io;
use itertools::Itertools;

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

struct Machine {
    program: Vec<i32>,
    cursor: usize,
    halted: bool
}

impl Machine {
    fn new(program:&Vec<i32>) -> Machine {
        let prog = (*program).iter().map(|s| *s).collect();
        let halted = false;
        return Machine {
            program: prog,
            cursor: 0,
            halted: halted
        }
    }

    fn run(&mut self, inputs:&Vec<i32>) -> Vec<i32> {
        let mut line = &mut self.program;
        let max = line.len();
        let mut i = self.cursor;
        let mut cmd;
        let mut args;
        let mut to;
        let mut inputs: Vec<i32> = (*inputs).iter().map(|s| *s).collect();
        let mut output = Vec::<i32>::new();
        loop {
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
                if let Some(inp) = inputs.pop() {
                    line[to] = inp;
                } else {
                    self.cursor = i;
                    break;
                }
                println!("{}: {}", to, line[to]);
                i += 2;
            } else if cmd == 4 {
                args = parse_args(&line, i, 1);
                println!("Output: {}", args[0]);
                output.push(args[0] as i32);
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
                self.halted = true;
                break;
            } else {
                println!("Errorr!!! Command not matched: {}", cmd);
                break;
            }
            //println!("next cmd!");
            //io::stdin().read_line(&mut String::new()).unwrap();
        }
        return output;
    }
}

fn main() -> Result<()> {
    let mut file = File::open("input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut line: Vec::<i32> = Vec::new();
    for i in input.trim().split(",") {
        let parsed = i.parse::<i32>();
        match parsed {
            Ok(x) => line.push(x),
            Err(_) => continue
        }
    }
    line.resize(10000, 99);

    const num: i32 = 5;
    let perms = (5..(5+num) as i32).permutations(num as usize);
    let mut resout = -1;
    let mut best;
    for perm in perms {
        let mut machines = Vec::<Machine>::new();
        for i in perm.iter() {
            let mut mach = Machine::new(&line);
            let input = vec![*i];
            mach.run(&input);
            machines.push(mach)
        }
        let mut out = 0;
        let mut c: usize = 0;
        loop {
            let input = vec![out];
            let res = machines[c].run(&input);
            out = res[0];
            if machines[4].halted {
                break
            }
            c = (c+1) % (num as usize);
        }
        if out > resout {
            resout = out;
            best = perm;
        }
    }
    println!("{}", resout);

    Ok(())
}
