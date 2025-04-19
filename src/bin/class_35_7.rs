fn main() {}

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

struct Node {
    count: i32,
    keys: HashSet<String>,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(count: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            count,
            keys: HashSet::new(),
            prev: None,
            next: None,
        }))
    }
}

pub struct AllOne {
    key_map: HashMap<String, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl AllOne {
    pub fn new() -> Self {
        AllOne {
            key_map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn insert_after(&mut self, prev: Option<Rc<RefCell<Node>>>, node: Rc<RefCell<Node>>) {
        

    }


    fn remove_node(&mut self, node: Rc<RefCell<Node>>) {
        let prev_weak = node.borrow().prev.clone();
        let next_rc = node.borrow().next.clone();

        if let Some(prev_rc) = prev_weak.as_ref().and_then(|w| w.upgrade()) {
            prev_rc.borrow_mut().next = next_rc.clone();
        } else {
            self.head = next_rc.clone();
        }

        if let Some(next_rc) = next_rc {
            next_rc.borrow_mut().prev = prev_weak;
        } else {
            self.tail = prev_weak.and_then(|w| w.upgrade());
        }
    }
}
