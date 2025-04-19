use crate::common::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = Self::min_depth(node.left.clone());
                let right = Self::min_depth(node.right.clone());

                if left == 0 || right == 0 {
                    return left + right + 1;
                }

                left.min(right) + 1
            }
        }
    }
}
