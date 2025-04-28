use std::{cell::RefCell, rc::Rc};

fn main() {
    todo!();
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, parent_val: i32, cur_len: i32, max_len: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            let new_len = if n.val == parent_val + 1 {
                cur_len + 1
            } else {
                1
            };
            *max_len = (*max_len).max(new_len);
            dfs(n.left.clone(), n.val, new_len, max_len);
            dfs(n.right.clone(), n.val, new_len, max_len);
        }
    }

    let mut max_len = 0;
    if let Some(n) = root {
        dfs(Some(n.clone()), n.borrow().val - 1, 0, &mut max_len);
    }
    max_len
}
