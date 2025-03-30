use leetcode_rust::ListNode;

fn main() {}

// 86. 分隔链表
// https://leetcode.cn/problems/partition-list/description/
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut small = Box::new(ListNode::new(0));
    let mut p1 = &mut small;
    let mut large = Box::new(ListNode::new(0));
    let mut p2 = &mut large;

    let mut p = head;

    while let Some(node) = &p {
        if node.val < x {
            p1.next = p;
            p1 = p1.next.as_mut()?;
            p = p1.next.take();
        } else {
            p2.next = p;
            p2 = p2.next.as_mut()?;
            p = p2.next.take();
        }
    }

    p1.next = large.next;
    small.next
}
