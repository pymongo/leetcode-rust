#[cfg(test)]
use super::{arr_to_linked_list, linked_list_to_vec, ListNode};
#[cfg(test)]
use std::boxed::Box;

#[cfg(test)]
struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut has_rev = None;
        let mut not_rev = head;
        while let Some(mut not_rev_node) = not_rev {
            not_rev = not_rev_node.next;

            not_rev_node.next = has_rev;
            has_rev = Some(not_rev_node);
        }
        return has_rev;
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], &[i32]); 1] = [(&[1, 2, 3], &[3, 2, 1])];

#[test]
fn test_traverse_two_list_node() {
    for &(input, output) in &TEST_CASES {
        let head = arr_to_linked_list(input);
        assert_eq!(
            linked_list_to_vec(Solution::reverse_list(head)),
            output.to_vec()
        );
    }
}
