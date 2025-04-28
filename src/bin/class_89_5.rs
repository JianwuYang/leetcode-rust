use std::collections::BinaryHeap;

fn main() {
    todo!();
}

pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
    courses.sort_by_key(|c| c[1]);

    let mut heap = BinaryHeap::new();
    let mut time = 0;

    for course in courses {
        let duration = course[0];
        let last_day = course[1];

        time += duration;
        heap.push(duration);

        if time > last_day {
            if let Some(longest) = heap.pop() {
                time -= longest;
            }
        }
    }

    heap.len() as i32
}
