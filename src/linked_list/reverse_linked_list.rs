use super::ListNode;

struct Solution;

impl Solution {
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut has_rev = None;
        let mut not_rev = head;
        while let Some(mut not_rev_node) = not_rev {
            /*
            H: has_rev
            N: not_rev
            M: mut not_rev_node
            Before:
            None 1->2->3
               ^ ^
               H N/M
            After:
            None 1->2->3
               ^ ^  ^
               H M  N
            */
            not_rev = not_rev_node.next;
            /*
            Before:
            None 1->2->3
               ^ ^  ^
               H M  N
            After:
            None<-1  2->3
               ^  ^  ^
               H  M  N
            */
            not_rev_node.next = has_rev;
            /*
            Before:
            None<-1  2->3
               ^  ^  ^
               H  M  N
            After:
            None<-1 2->3
                  ^ ^
                  H N
            */
            has_rev = Some(not_rev_node);
        }
        has_rev
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], &[i32]); 1] = [(&[1, 2, 3], &[3, 2, 1])];

#[test]
fn test_traverse_two_list_node() {
    use crate::linked_list::{arr_to_linked_list, linked_list_to_vec};
    for &(input, output) in &TEST_CASES {
        let head = arr_to_linked_list(input);
        assert_eq!(
            linked_list_to_vec(&Solution::reverse_list(head)),
            output.to_vec()
        );
    }
}
