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


fn main() {
    let mut a = Forest::new();
}
