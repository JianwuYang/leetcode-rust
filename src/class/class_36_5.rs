use crate::common::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn f(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
            match root {
                None => ans.push_str("#,"),
                Some(node) => {
                    let node = node.borrow();
                    ans.push_str(&format!("{},", node.val));
                    f(node.left.clone(), ans);
                    f(node.right.clone(), ans);
                }
            }
        }

        let mut ans = String::new();
        f(root, &mut ans);
        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn g(vals: &Vec<&str>, cnt: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            let cur = vals[*cnt];
            *cnt += 1;
            if cur == "#" {
                return None;
            } else {
                let head: Rc<RefCell<TreeNode>> =
                    Rc::new(RefCell::new(TreeNode::new(cur.parse().unwrap())));
                head.borrow_mut().left = g(vals, cnt);
                head.borrow_mut().right = g(vals, cnt);
                Some(head)
            }
        }
        let vals: Vec<&str> = data.split(",").collect();
        let mut cnt = 0;
        g(&vals, &mut cnt)
    }
}
