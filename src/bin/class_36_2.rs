fn main() {}

struct Solution {}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode_rust::TreeNode;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        if root.is_none() {
            return ans;
        }

        let mut queue = VecDeque::new();

        queue.push_back(root.unwrap());

        let mut left_to_right = true;

        while !queue.is_empty() {
            let level_size = queue.len();

            let mut level = Vec::with_capacity(level_size);

            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();

                if left_to_right {
                    level.push(node_ref.val);
                } else {
                    level.insert(0, node_ref.val);
                }

                if let Some(left) = &node_ref.left {
                    queue.push_back(left.clone());
                }

                if let Some(right) = &node_ref.right {
                    queue.push_back(right.clone());
                }
            }

            ans.push(level);
            left_to_right = !left_to_right;
        }

        ans
    }
}
