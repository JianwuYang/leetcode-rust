/**
 * 使用循环数组实现双端队列
 */
fn main() {}
pub struct MyCircularDeque {
    values: Vec<i32>,
    capacity: usize,
    l: usize,
    r: usize,
    size: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        MyCircularDeque {
            values: vec![0; 1000],
            capacity: k as usize,
            l: 0,
            r: 0,
            size: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.l = (self.l + self.capacity - 1) % self.capacity;
        self.values[self.l] = value;
        self.size += 1;
        true
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.values[self.r] = value;
        self.r = (self.r + self.capacity + 1) % self.capacity;
        self.size += 1;
        true
    }

    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.l = (self.l + self.capacity + 1) % self.capacity;
        self.size -= 1;
        true
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.r = (self.r + self.capacity - 1) % self.capacity;
        self.size -= 1;
        true
    }

    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[self.l]
        }
    }

    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[(self.r + self.capacity - 1) % self.capacity]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}
