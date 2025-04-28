use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    todo!();
}

pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut heap = BinaryHeap::new();

    let mut ans = 0;

    for interval in intervals {
        while let Some(&Reverse(end)) = heap.peek() {
            if end <= interval[0] {
                heap.pop();
            } else {
                break;
            }
        }
        heap.push(Reverse(interval[1]));
        ans = ans.max(heap.len());
    }

    ans as i32
}
