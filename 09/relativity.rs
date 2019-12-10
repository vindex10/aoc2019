use std::fs::File;
use std::io::{prelude::*, Result};
use std::io;
use std::ops::{Index, IndexMut};
use std::cmp::{min};

struct Pages {
    pages: Vec<Vec<i64>>,
    indices: Vec<usize>
}

impl Pages {
    fn new() -> Pages {
        Pages {pages: vec![vec![0;10000]],
               indices: vec![0]}
    }
}

impl Index<usize> for Pages {
    type Output = i64;

    fn index(&self, idx: usize) -> &Self::Output {
        let index = match self.indices.binary_search(&idx) {
            Ok(i) => {i},
            Err(i) => {
                i-1
            }
        };

        if idx - self.indices[index] < 10000 {
            return &self.pages[index][idx - self.indices[index]];
        } else {
            return &0;
        }
    }
}

impl IndexMut<usize> for Pages {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        let index = match self.indices.binary_search(&idx) {
            Ok(i) => {i},
            Err(i) => {
                let newpage_size;
                if i < self.indices.len() {
                    let next_num = self.indices[i];
                    newpage_size = min(next_num - idx, 10000);
                } else {
                    newpage_size = 10000;
                }
                let newpage = vec![0; newpage_size];
                self.indices.insert(i, idx);
                self.pages.insert(i, newpage);
                i
            }
        };
        &mut self.pages[index][idx - self.indices[index]]
    }
}

struct ArgParser {
    relative_base: i64
}

type Prg = Pages;

impl ArgParser {
    fn new() -> ArgParser {
        return ArgParser { relative_base: 0 };
    }

    fn parse_args(&self, line:&Prg, instruction_i:usize, num_args:i8) -> Vec::<i64> {
        let mut res = Vec::<i64>::new();
        let mut arg_counter = 0;

        let mut buf = line[instruction_i]/100;
        let mut idx;
        while buf != 0 && arg_counter < num_args-1 {
            arg_counter += 1;
            idx = line[(instruction_i as i64 + arg_counter as i64) as usize];
            if buf % 10 == 0 {
                res.push(line[idx as usize]);
            } else if buf % 10 == 2 {
                res.push(line[(idx + self.relative_base) as usize]);
            }
            else {
                res.push(idx);
            }
            buf = buf / 10;
        }

        if buf != 0 {
            arg_counter += 1;
            idx = line[(instruction_i as i64 + arg_counter as i64) as usize];
            if buf % 10 == 0 {
                res.push(idx as i64);
            } else if buf % 10 == 2 {
                res.push((idx + self.relative_base) as i64);
            }
            else {
                res.push(idx);
            }
            return res;
        }

        while (arg_counter as i8) < num_args-1 {
            arg_counter += 1;
            idx = line[((instruction_i as i64) + arg_counter as i64) as usize];
            res.push(line[idx as usize] as i64);
        }

        arg_counter += 1;
        idx = line[(instruction_i as i64 + arg_counter as i64) as usize];
        res.push(idx);
        return res;
    }
}

fn main() -> Result<()> {
    let mut file = File::open("input")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut line: Prg = Prg::new();

    let mut i = 0;
    for ch in input.trim().split(",") {
        let parsed = ch.parse::<i64>();
        match parsed {
            Ok(num) => { line[i] = num },
            Err(_) => continue
        }
        i += 1;
    }

    i = 0;
    let mut cmd;
    let mut args;
    let mut val = String::new();
    let mut to;
    let mut argparser = ArgParser::new();
    loop {
        cmd = line[i] % 100;
        println!("cmd. {}, {}", i, line[i]);
        if cmd == 1 {
            args = argparser.parse_args(&line, i, 3);
            to = args[2] as usize;
            //println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] = args[1] + args[0];
            println!("{}: {}", to, line[to]);
            i += 4;
        } else if cmd == 2 {
            args = argparser.parse_args(&line, i, 3);
            to = args[2] as usize;
            //println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] = args[1] * args[0];
            println!("{}: {}", to, line[to]);
            i += 4;
        } else if cmd == 3 {
            args = argparser.parse_args(&line, i, 1);
            to = args[0] as usize;
            //println!("{}, {}", cmd, to);
            println!("Enter input value: ");
            io::stdin().read_line(&mut val).unwrap();
            line[to] = val.trim().parse::<i64>().unwrap();
            println!("{}: {}", to, line[to]);
            i += 2;
        } else if cmd == 4 {
            args = argparser.parse_args(&line, i, 1);
            println!("{}", line[args[0] as usize]);
            i += 2;
            //println!("{}, {}", cmd, args[0]);
        } else if cmd == 5 {
            args = argparser.parse_args(&line, i, 3);
            //println!("{}, {}, {}", cmd, args[0], args[1]);
            if args[0] != 0 {
                i = args[1] as usize;
            } else {
                i += 3;
            }
        } else if cmd == 6 {
            args = argparser.parse_args(&line, i, 3);
            //println!("{}, {}, {}", cmd, args[0], args[1]);
            if args[0] == 0 {
                i = args[1] as usize;
            } else {
                i += 3;
            }
        } else if cmd == 7 {
            args = argparser.parse_args(&line, i, 3);
            to = args[2] as usize;
            //println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] = if args[0] < args[1] { 1 } else { 0 }; 
            i += 4;
            println!("{}: {}", to, line[to]);
        } else if cmd == 8 {
            args = argparser.parse_args(&line, i, 3);
            to = args[2] as usize;
            //println!("{}, {}, {}, {}", cmd, args[0], args[1], to);
            line[to] = if args[0] == args[1] { 1 } else { 0 }; 
            i += 4;
            println!("{}: {}", to, line[to]);
        } else if cmd == 9 {
            args = argparser.parse_args(&line, i, 2);
            //println!("{}, {}", cmd, args[0]);
            argparser.relative_base += args[0];
            println!("rb: {}", argparser.relative_base);
            i += 2;
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
