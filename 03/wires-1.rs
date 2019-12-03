use std::io::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Coord = i64;

struct Ray {
    coord: Coord,
    from: i64,
    to: i64
}

type Bundle = Vec<Ray>;

fn build_bundle(line:&String) -> (Bundle, Bundle) {
    let mut bundle_x = Bundle::new();
    let mut bundle_y = Bundle::new();

    let (mut x, mut y) = (0, 0);
    let mut cmd;
    let mut param;
    for word in line.trim().split(",") {
        cmd = word.chars().nth(0).unwrap().to_string();
        param = word[1..].parse::<i64>().unwrap() as Coord;

        if cmd == "U" {
            bundle_x.push(Ray {coord: x, from: y, to: y+param});
            y += param;
        } else if cmd == "D" {
            bundle_x.push(Ray {coord: x, from: y-param, to: y});
            y -= param;
        } else if cmd == "L" {
            bundle_y.push(Ray {coord: y, from: x-param, to: x});
            x -= param;
        } else if cmd == "R" {
            bundle_y.push(Ray {coord: y, from: x, to: x+param});
            x += param;
        }
    }

    return (bundle_x, bundle_y);
}

fn intersect_bundles(a:Bundle, b:Bundle) -> Vec<(Coord, Coord)> {
    let mut res = Vec::<(Coord, Coord)>::new();
    for ray1 in a.iter() {
        for ray2 in b.iter() {
            if ray2.from <= ray1.coord && ray2.to >= ray1.coord && ray2.coord >= ray1.from && ray2.coord <= ray1.to {
                res.push((ray1.coord, ray2.coord));
            }
        }
    }
    return res;
}

fn main() -> Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f).lines();

    let mut line;

    line = reader.next().unwrap().unwrap();
    let (bundle1_x, bundle1_y) = build_bundle(&line);

    line = reader.next().unwrap().unwrap();
    let (bundle2_x, bundle2_y) = build_bundle(&line);

    let mut dist = i64::max_value();
    let mut tmp;

    for (x, y) in intersect_bundles(bundle1_x, bundle2_y) {
        tmp = x.abs() + y.abs();
        if tmp < dist {
            dist = tmp
        }
    }
    for (y, x) in intersect_bundles(bundle1_y, bundle2_x) {
        tmp = x.abs() + y.abs();
        if tmp < dist {
            dist = tmp
        }
    }

    println!("{}", dist);
    
    return Ok(());
}
