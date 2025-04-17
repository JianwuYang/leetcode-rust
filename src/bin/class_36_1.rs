use leetcode_rust::TreeNode;
fn main() {}
struct Solution {}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        if root.is_none() {
            return ans;
        }

        let mut queue = VecDeque::new();

        queue.push_back(root.unwrap());

        while !queue.is_empty() {

            let size = queue.len();

            let mut list = Vec::with_capacity(size);

            for _ in 0..size {

                let cur = queue.pop_front().unwrap();
                let node = cur.borrow();

                list.push(node.val);

                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }
            ans.push(list);

        }

        ans
    }
}
