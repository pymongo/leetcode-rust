use super::ListNode;

struct Solution;

impl Solution {
    fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut slow = head.as_ref();
        while let Some(fast_inner) = fast {
            fast = fast_inner.next.as_ref();
            if fast.is_none() {
                break;
            }
            fast = fast?.next.as_ref();
            slow = slow?.next.as_ref();
        }
        // slow.map(|node| node.clone())
        slow.cloned()
    }

    fn clean_solution(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some() && fast?.next.is_some() {
            slow = slow?.next.as_ref();
            fast = fast?.next.as_ref()?.next.as_ref();
        }

        slow.cloned()
    }
}
