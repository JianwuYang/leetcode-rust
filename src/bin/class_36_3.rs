fn main() {

}

struct Solution {

}

use leetcode_rust::TreeNode;


use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        if root.is_none() {
            return 0;
        }

        let mut max_width = 0;
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();

        queue.push_back((root.unwrap(), 0));

        while !queue.is_empty() {
            let level_size = queue.len();
            let first_index = queue.front().unwrap().1;
            let mut last_index = 0;

            for _ in 0..level_size {
                let (node, index) = queue.pop_front().unwrap();
                let node_ref = node.borrow();

                let normalized_index = index - first_index;

                if let Some(left) = &node_ref.left {
                    queue.push_back((left.clone(), 2 * normalized_index));
                }
                if let Some(right) = &node_ref.right {
                    queue.push_back((right.clone(), 2 * normalized_index + 1));
                }

                last_index = normalized_index;
            }

            let width = last_index + 1;
            max_width = max_width.max(width);
        }
        
        max_width as i32
    }
}