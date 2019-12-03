use std::io::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp::min;
use std::cmp::max;

type Coord = i64;

struct Ray {
    coord: Coord,
    from: i64,
    to: i64,
    steps_to: i64
}

type Bundle = Vec<Ray>;

fn build_bundle(line:&String) -> (Bundle, Bundle) {
    let mut bundle_x = Bundle::new();
    let mut bundle_y = Bundle::new();

    let (mut x, mut y) = (0, 0);
    let mut cmd;
    let mut param;
    let mut steps = 0;
    for word in line.trim().split(",") {
        cmd = word.chars().nth(0).unwrap().to_string();
        param = word[1..].parse::<i64>().unwrap() as Coord;

        if cmd == "U" {
            bundle_x.push(Ray {coord: x, from: y, to: y+param, steps_to: steps});
            y += param;
            steps += param;
        } else if cmd == "D" {
            bundle_x.push(Ray {coord: x, from: y, to: y-param, steps_to: steps});
            y -= param;
            steps += param;
        } else if cmd == "L" {
            bundle_y.push(Ray {coord: y, from: x, to: x-param, steps_to: steps});
            x -= param;
            steps += param;
        } else if cmd == "R" {
            bundle_y.push(Ray {coord: y, from: x, to: x+param, steps_to: steps});
            x += param;
            steps += param;
        }
    }

    return (bundle_x, bundle_y);
}

fn intersect_bundles(a:Bundle, b:Bundle) -> Vec<(Coord, Coord, Coord)> {
    let mut res = Vec::<(Coord, Coord, Coord)>::new();
    let mut ray1_min;
    let mut ray1_max;
    for ray1 in a.iter() {
        ray1_min = min(ray1.from, ray1.to);
        ray1_max = max(ray1.from, ray1.to);
        let mut ray2_min;
        let mut ray2_max;
        for ray2 in b.iter() {
            ray2_min = min(ray2.from, ray2.to);
            ray2_max = max(ray2.from, ray2.to);
            if ray2_min <= ray1.coord && ray2_max >= ray1.coord && ray2.coord >= ray1_min && ray2.coord <= ray1_max {
                res.push((ray1.coord, ray2.coord, ray1.steps_to + ray2.steps_to + (ray1.coord - ray2.from).abs() + (ray2.coord - ray1.from).abs()));
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

    for (_x, _y, cur_dist) in intersect_bundles(bundle1_x, bundle2_y) {
        if _x == 0 && _y == 0 {
            continue;
        }
        if cur_dist < dist {
            dist = cur_dist
        }
    }

    for (_x, _y, cur_dist) in intersect_bundles(bundle2_x, bundle1_y) {
        if _x == 0 && _y == 0 {
            continue;
        }
        if cur_dist < dist {
            dist = cur_dist
        }
    }

    println!("{}", dist);
    
    return Ok(());
}
