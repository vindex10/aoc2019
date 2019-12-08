use std::io::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::option::Option;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

fn print_offset(offset: i32) {
    for _ in 1..offset {
        print!(" ");
    }
}

struct Node {
    id: String,
    children: Option<Vec<Rc<RefCell<Node>>>>
}

impl Node {
    fn add(&mut self, new_child: Rc<RefCell<Node>>) {
        match &mut self.children {
            Some(ch) => ch.push(Rc::clone(&new_child)),
            None => {
                let new_children = vec![Rc::clone(&new_child)];
                self.children = Some(new_children);
            }
        }
    }

    fn print(&self, offset: i32) {
        print_offset(offset);
        println!("{}", self.id);
        if let Some(children) = &self.children {
            for node in children {
                (&node).borrow().print(offset+4);
            }
        }
    }
}

struct Forest {
    registry: HashMap<String, Rc<RefCell<Node>>>,
    roots: HashMap<String, Rc<RefCell<Node>>>
}

impl Forest {
    fn new() -> Forest {
        return Forest {
            registry: HashMap::<String, Rc<RefCell<Node>>>::new(),
            roots: HashMap::<String, Rc<RefCell<Node>>>::new()
        }
    }

    // a)b
    fn add(&mut self, a:&str, b:&str) {
        let planet = match self.roots.get(b) {
            Some(p) => {
                let res = Rc::clone(&p);
                self.roots.remove(b);
                res
            }
            None => {
                let new_p = Node {id: String::from(b), children: None};
                let new_p_ref = Rc::new(RefCell::new(new_p));
                self.registry.insert(String::from(b), Rc::clone(&new_p_ref));
                new_p_ref
            }
        };
        match self.registry.get(a) {
            Some(c) => {
                c.borrow_mut().add(planet)
            },
            None => {
                let new_c = Node {id: String::from(a), children: Some(vec![Rc::clone(&planet)])};
                let c_ref = Rc::new(RefCell::new(new_c));
                self.registry.insert(String::from(a), Rc::clone(&c_ref));
                self.roots.insert(String::from(a), Rc::clone(&c_ref));
            }
        };
    }

    fn print(&self, offset: i32) {
        for root_node in self.roots.values() {
            (&root_node).borrow().print(offset);
        }
    }
}

fn count_orbits(root:&RefCell<Node>, steps:i32) -> i32 {
    let mut orbits = 0;
    if let Some(children) = &((*root).borrow().children) {
        for node in children.iter() {
            orbits += steps + 1 + count_orbits(&node, steps+1);
        }
    }
    return orbits;
}

fn main() -> Result<()> {
    let mut forest = Forest::new();

    let f = File::open("input")?;
    let reader = BufReader::new(f);

    let mut a;
    let mut b;
    let mut pair_iter:Vec<String>;
    for line in reader.lines() {
        let pair = line.unwrap();
        pair_iter = pair.trim().split(")").map(|s| String::from(s)).collect();
        a = String::from(&pair_iter[0]);
        b = String::from(&pair_iter[1]);
        forest.add(&a, &b);
    }

    println!("{}", count_orbits(&*forest.roots["COM"], 0));

    Ok(())
}
