use std::collections::VecDeque;

fn main() {}

pub struct MyQueue {
    a: Vec<i32>,
    b: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        // Initialize the queue with two empty stacks
        MyQueue {
            a: vec![],
            b: vec![],
        }
    }

    pub fn push(&mut self, value: i32) {
        // Push the element to the back of the queue
        self.a.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Remove and return the front element of the queue
        if self.b.is_empty() {
            while let Some(value) = self.a.pop() {
                self.b.push(value);
            }
        }
        self.b.pop()
    }

    pub fn peek(&mut self) -> Option<i32> {
        // Return the front element of the queue without removing it
        if self.b.is_empty() {
            while let Some(value) = self.a.pop() {
                self.b.push(value);
            }
        }
        self.b.last().cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.a.is_empty() && self.b.is_empty()
    }
}

pub struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        let n = self.queue.len();
        self.queue.push_back(value);

        for _ in 0..n {
            if let Some(front) = self.queue.pop_front() {
                self.queue.push_back(front);
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.queue.pop_front()
    }

    pub fn top(&self) -> Option<i32> {
        self.queue.front().cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), Some(1));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_my_stack() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), Some(2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.is_empty(), true);
    }
}
