use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {}

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || self.max_heap.peek().unwrap() >= &num {
            self.max_heap.push(num);
        }
        else {
            self.min_heap.push(Reverse(num));
        }
        self.balance();
    }

    fn find_median(&self) -> f64 {
        if self.min_heap.len() == self.max_heap.len() {
            (*self.max_heap.peek().unwrap() + self.min_heap.peek().unwrap().0) as f64 / 2.0
        } else if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as f64
        } else {
            self.min_heap.peek().unwrap().0 as f64
        }
    }

    fn balance(&mut self) {
        if self.max_heap.len().abs_diff(self.min_heap.len()) == 2 {
            if self.max_heap.len() > self.min_heap.len() {
                self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
            } else {
                self.max_heap.push(self.min_heap.pop().unwrap().0);
            }
        }
    }
}
