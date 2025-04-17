use std::collections::HashMap;

fn main() {}

struct FreqStack {
    top_times: usize,
    cnt_values: HashMap<usize, Vec<i32>>,
    value_times: HashMap<i32, usize>,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            top_times: 0,
            cnt_values: HashMap::new(),
            value_times: HashMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let count = self.value_times.entry(val).or_insert(0);
        *count += 1;
        let cur_top_times = *count;
        self.cnt_values
            .entry(cur_top_times)
            .or_insert(Vec::new())
            .push(val);
        self.top_times = self.top_times.max(cur_top_times);
    }

    fn pop(&mut self) -> i32 {
        let top_time_values = self.cnt_values.get_mut(&self.top_times).unwrap();
        let ans = top_time_values.pop().unwrap();

        if top_time_values.is_empty() {
            self.cnt_values.remove(&self.top_times);
            self.top_times -= 1;
        }

        let times = self.value_times.get_mut(&ans).unwrap();
        if *times == 1 {
            self.value_times.remove(&ans);
        }
        else {
            *times -= 1;
        }
        ans
    }
}
