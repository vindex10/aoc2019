use std::fs::File;
use std::io::Result;
use std::io::Read;

fn main() -> Result<()> {
    let mut file = File::open("input")?;
    let mut line = String::new();
    file.read_to_string(&mut line)?;

    let mut res = vec![2;25*6];

    let mut i: usize = 0;
    for ch in line.chars() {
        if let Some(val) = ch.to_digit(10) {
            if val != 2 && res[i] == 2 {
                res[i] = val;
            }
        }

        i = (i + 1) % (25*6);
    }

    i = 0;
    for ch in res {
        print!("{} ", ch);
        if i == 24 {
            println!("");
        }
        i = (i + 1)%25;
    }

    Ok(())
}
