use super::ListNode;

fn bottom_up_get_carry(node: &mut Option<Box<ListNode>>) -> i32 {
    match node {
        Some(ref mut node) => {
            let add_result = node.val + bottom_up_get_carry(&mut node.next);
            node.val = add_result % 10;
            add_result / 10
        }
        // 自底向上，这是链表的尾部
        None => 1,
    }
}

/// https://leetcode.com/problems/plus-one-linked-list/submissions/
fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let carry = bottom_up_get_carry(&mut head);
    if carry == 1 {
        Some(Box::new(ListNode { val: 1, next: head }))
    } else {
        head
    }
}

#[test]
fn test_plus_one() {
    use super::{arr_to_linked_list, linked_list_to_vec};
    const TEST_CASES: [(&[i32], &[i32]); 2] = [(&[1, 2, 3], &[1, 2, 4]), (&[9, 9], &[1, 0, 0])];
    for &(input, output) in &TEST_CASES {
        assert_eq!(
            linked_list_to_vec(&plus_one(arr_to_linked_list(input))),
            output
        );
    }
}
