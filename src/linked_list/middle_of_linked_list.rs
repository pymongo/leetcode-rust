use super::ListNode;

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        // TODO 注意head.as_ref()得到的是Option<&T>，而&head会得到不同的(不好用)的&Option<T>
        // TODO 链表/二叉树题建议用as_ref()或as_mut()
        let mut slow: Option<&Box<ListNode>> = head.as_ref();
        let mut fast: Option<&Box<ListNode>> = head.as_ref();

        while fast.is_some() && fast?.next.is_some() {
            slow = slow?.next.as_ref();
            fast = fast?.next.as_ref()?.next.as_ref();
        }

        // slow.map(|node| node.clone())
        slow.cloned()
    }
}
