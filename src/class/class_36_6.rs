use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::common::TreeNode;
struct Codec {
    queue: VecDeque<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::new();

        if let Some(node) = root {
            let node_rc = node.borrow();
            ans.push_str(&format!("{},", node_rc.val));
            self.queue.push_back(node.clone());
            while !self.queue.is_empty() {
                let cur = self.queue.pop_front().unwrap();
                let cur_ref = cur.borrow();
                if let Some(left) = &cur_ref.left {



                }
                else {
                    ans.push_str("#,");
                }

            }
        }

        ans
    }

    // fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {}
}
