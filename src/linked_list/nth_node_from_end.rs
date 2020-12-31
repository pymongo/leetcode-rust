use super::ListNode;

/// https://leetcode-cn.com/problems/kth-node-from-end-of-list-lcci/
/// 找出单向链表中倒数第 k 个节点，返回该节点的值，容易想到用滑动窗口的方法，宽度为k窗口往右划底时，左边界就是倒数第k个节点
fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
    fn helper(head: Option<Box<ListNode>>, k: i32) -> Option<i32> {
        let mut dummy = ListNode { val: 0, next: head };
        let mut left = dummy.next.as_ref();
        let mut right = dummy.next.as_ref();
        for _ in 0..k {
            right = right?.next.as_ref();
        }

        // while fast?.next.is_some() && fast?.next?.next.is_some() {
        //     fast = fast?.next?.next.as_ref();
        //     slow = slow?.next.as_ref();
        // }
        Some(0i32)
    }
    helper(head, k).unwrap_or_default()
}
