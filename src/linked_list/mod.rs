mod add_two_linked_list;

use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn arr_to_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut dummy;
    for num in nums {
        if let Some(curr_node) = curr {
            curr_node.next = Some(Box::new(ListNode::new(*num)));
            curr = &mut curr_node.next;
        }
    }
    dummy.unwrap().next
}


pub fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    let mut curr = head;
    while let Some(node) = curr {
        nums.push(node.val);
        curr = node.next;
    }
    nums
}

#[cfg(test)]
const TEST_CASES: [&[i32]; 1] = [&[1, 2, 3, 4, 5]];

#[test]
fn test_arr_to_linked_list() {
    for nums in &TEST_CASES {
        let head = arr_to_linked_list(nums);
        let nums_vec = linked_list_to_vec(head);
        println!("{:?}", nums_vec);
    }
}
