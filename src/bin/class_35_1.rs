use std::{
    collections::HashMap,
    ptr::{self, NonNull},
};

fn main() {}

type Link = Option<NonNull<DoubleNode>>;

struct DoubleNode {
    key: i32,
    val: i32,
    last: Link,
    next: Link,
}

struct LRUCache {
    map: HashMap<i32, NonNull<DoubleNode>>,
    capacity: i32,
    size: i32,
    head: Link,
    tail: Link,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            capacity,
            size: 0,
            head: None,
            tail: None,
        }
    }

    fn add_node(self: &mut Self, node: NonNull<DoubleNode>) {
        unsafe {
            // let new = NonNull::new_unchecked(Box::into_raw(node));
            let new = node;
            if let Some(old) = self.tail {
                (*old.as_ptr()).next = Some(new);
                (*new.as_ptr()).last = Some(old);
            } else {
                self.head = Some(new);
            }
            self.tail = Some(new);
        }
    }

    fn remove_head(self: &mut Self) -> Option<Box<DoubleNode>> {
        unsafe {
            self.head.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                self.head = boxed_node.next;
                if let Some(new) = self.head {
                    (*new.as_ptr()).last = None;
                } else {
                    self.tail = None;
                }
                boxed_node
            })
        }
    }

    fn move_node_to_tail(self: &mut Self, node: NonNull<DoubleNode>) {
        unsafe {
            if let (Some(old_tail), Some(old_head)) = (self.tail, self.head) {
                // let change_node = Box::from_raw(node.as_ptr());

                if ptr::eq(old_tail.as_ptr(), node.as_ptr()) {
                    return;
                }

                if ptr::eq(old_head.as_ptr(), node.as_ptr()) {
                    self.head = (*node.as_ptr()).next;
                    if let Some(new) = self.head {
                        (*new.as_ptr()).last = None;
                    }
                } else {
                    if let (Some(last), Some(next)) = ((*node.as_ptr()).last, (*node.as_ptr()).next) {
                        (*last.as_ptr()).next = Some(next);
                        (*next.as_ptr()).last = Some(last);
                    }
                }
                (*node.as_ptr()).last = self.tail;
                (*node.as_ptr()).next = None;
                (*old_tail.as_ptr()).next = Some(node);
                self.tail = Some(node);
            }
        }
    }

    fn get(self: &mut Self, key: i32) -> i32 {
        let mut node = None;

        if let Some(value) = self.map.get(&key) {
            node = Some(value.clone());
        }

        if let Some(node) = node {
            self.move_node_to_tail(node);

            unsafe {
                let x = (*node.as_ptr()).val;
                return x;
            }
        }
        return -1;
    }

    fn put(&mut self, key: i32, value: i32) {
        unsafe {
            let mut node = None;

            if let Some(value) = self.map.get(&key) {
                node = Some(value.clone());
            }

            if let Some(node) = node {
                (*node.as_ptr()).val = value;
                self.move_node_to_tail(node);
            } else {
                if self.size == self.capacity {
                    let over = self.remove_head().unwrap().key;
                    self.map.remove(&over);
                    self.size -= 1;
                }

                let new = NonNull::new_unchecked(Box::into_raw(Box::new(DoubleNode {
                    key,
                    val: value,
                    last: None,
                    next: None,
                })));

                self.map.insert(key, new);

                self.add_node(new);
                self.size += 1;
            }
        }
    }
}

// impl Drop for LRUCache {
//     fn drop(&mut self) {
//         while let Some(_) = self.remove_head() {}
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
    }
}
