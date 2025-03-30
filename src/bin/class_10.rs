use leetcode_rust::ListNode;

fn main() {}

// 21. 合并两个有序链表
// https://leetcode.cn/problems/merge-two-sorted-lists/description/
pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut cur = &mut dummy;

    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        let lst: &mut Option<Box<ListNode>> = if node1.val < node2.val {
            &mut list1
        } else {
            &mut list2
        };
        cur.next = lst.take();
        cur = cur.next.as_mut()?;
        *lst = cur.next.take();
    }
    cur.next = list1.or(list2);
    dummy.next
}
