use std::collections::HashMap;

use rand::{Rng, rngs::ThreadRng};

fn main() {
    let mut random_set = RandomizedSet::new();
    random_set.remove(0);
    random_set.remove(0);
    random_set.insert(0);
    random_set.get_random();
    random_set.remove(0);
    let ans = random_set.insert(0);
    println!("{ans}");
}

struct RandomizedSet {
    map: HashMap<i32, usize>,
    arr: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            arr: Vec::new(),
            rng: rand::rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.map.insert(val, self.arr.len());
            self.arr.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(val_idx) = self.map.remove(&val) {
            if val_idx == self.arr.len() - 1 {
                self.arr.pop();
            } else {
                let end_value = self.arr.pop().unwrap();
                self.map.insert(end_value, val_idx);
                self.arr.insert(val_idx, end_value);
            }
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        let idx = self.rng.random_range(0..self.arr.len());
        self.arr[idx]
    }
}
