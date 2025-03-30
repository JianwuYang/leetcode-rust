// 单双链表及其反转

use leetcode_rust::ListNode;

fn main() {}

// 206. 反转链表
// https://leetcode.cn/problems/reverse-linked-list/description/
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut cur = head;
    while let Some(mut node) = cur {
        let next = node.next;
        node.next = pre;
        pre = Some(node);
        cur = next;
    }

    pre
}
