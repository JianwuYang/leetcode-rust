fn main() {}

pub trait MyQueue {
    fn is_empty(&self) -> bool;
    fn offer(&mut self, value: i32);
    fn poll(&mut self) -> Option<i32>;
    fn peek(&self) -> Option<i32>;
    fn size(&self) -> usize;
}

pub struct MyQueueImpl {
    queue: Vec<i32>,
    size: usize,
    head: usize,
    tail: usize,
}

impl MyQueueImpl {
    pub fn new(n: usize) -> Self {
        MyQueueImpl {
            queue: vec![0; n],
            size: 0,
            head: 0,
            tail: 0,
        }
    }
}

impl MyQueue for MyQueueImpl {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn offer(&mut self, value: i32) {
        if self.size == self.queue.len() {
            panic!("Queue is full");
        }
        self.queue[self.tail] = value;
        self.tail = if self.tail + 1 == self.queue.len() {
            0
        } else {
            self.tail + 1
        };
        self.size += 1;
    }

    fn poll(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let res = self.queue[self.head];
            self.head = if self.head + 1 == self.queue.len() {
                0
            } else {
                self.head + 1
            };
            self.size -= 1;
            Some(res)
        }
    }

    fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.queue[self.head])
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_my_queue() {
        let mut queue = MyQueueImpl::new(5);
        assert!(queue.is_empty());
        queue.offer(1);
        queue.offer(2);
        queue.offer(3);
        queue.offer(4);
        queue.offer(5);
        assert_eq!(queue.size(), 5);
        assert_eq!(queue.peek(), Some(1));
        assert_eq!(queue.poll(), Some(1));
        assert_eq!(queue.size(), 4);
        assert_eq!(queue.peek(), Some(2));
    }
}
