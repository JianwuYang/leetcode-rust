use leetcode_rust::ListNode;

fn main() {}

// 2. 两数相加
// https://leetcode.cn/problems/add-two-numbers/description/
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut cur = &mut dummy;

    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        if let Some(node) = l1 {
            carry += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            carry += node.val;
            l2 = node.next;
        }

        cur.next = Some(Box::new(ListNode::new(carry % 10)));
        carry = carry / 10;

        cur = cur.next.as_mut()?;
    }

    dummy.next
}
