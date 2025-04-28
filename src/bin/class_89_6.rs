use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    todo!();
}

pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();

    for stick in sticks {
        heap.push(Reverse(stick));
    }

    let mut ans = 0;

    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        let new = a + b;
        ans += new;
        heap.push(Reverse(new));
    }

    ans
}
