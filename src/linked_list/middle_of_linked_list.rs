//! https://leetcode.com/problems/middle-of-the-linked-list/
use super::ListNode;

/// 链表题型的快慢双指针算法
/// 快指针每次两步，慢指针每次一步
/// 快指针到尾的时候，慢指针就是中点(左中位节点)
fn middle_of_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();
    while fast.is_some() && fast?.next.is_some() {
        slow = slow?.next.as_ref();
        fast = fast?.next.as_ref()?.next.as_ref();
    }
    slow.cloned()
}

#[test]
fn test_middle_of_linked_list() {
    use super::{arr_to_linked_list, linked_list_to_vec};
    const TEST_CASES: [(&[i32], &[i32]); 2] = [
        (&[1, 2, 3, 4, 5], &[3, 4, 5]),
        (&[1, 2, 3, 4, 5, 6], &[4, 5, 6]),
    ];
    for (input, output) in TEST_CASES {
        let input = arr_to_linked_list(input);
        assert_eq!(linked_list_to_vec(&middle_of_linked_list(input)), output);
    }
}
