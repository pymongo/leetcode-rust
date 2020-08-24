#![allow(dead_code, unused_imports)]
mod add_two_linked_list;
mod reverse_linked_list;
mod reverse_linked_list_2;
mod merge_two_sorted_linked_list;
mod middle_of_linked_list;

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

// Option<T>的设计优点之一: 链表题可以不使用dummy_head也能生成链表后返回头部
fn arr_to_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut curr = &mut head;
    for num in nums {
        *curr = Some(Box::new(ListNode::new(*num)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    head
}

/* Java版数组转链表
public static ListNode arrayToListNode(int []nums) {
    ListNode dummy = new ListNode();
    ListNode cur = dummyHead;
    for (int num: nums) {
        cur.next = new ListNode(num);
        cur = cur.next;
    }
    return dummy.next;
}
*/
#[cfg(not)]
pub fn arr_to_linked_list_with_dummy(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut dummy;
    for num in nums {
        let curr_node = curr.as_mut().unwrap();
        curr_node.next = Some(Box::new(ListNode::new(*num)));
        curr = &mut curr_node.next;
    }
    dummy.unwrap().next
}

/* Java版链表转数组
public static int[] listNodeToArray(ListNode head) {
    List<Integer> nums = new ArrayList<>();
    ListNode curr = head;
    while (curr != null) {
        nums.add(curr.val);
        curr = curr.next;
    }
    return nums.stream().mapToInt(i -> i).toArray();
}
*/
pub fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    // 由于链表转数组只需要读链表不需要修改链表各节点，所以curr=head而不是curr=&mut head，而且代码也简洁多了
    let mut curr = head;
    while let Some(curr_node) = curr {
        nums.push(curr_node.val);
        curr = curr_node.next;
    }
    // println!("{:?}", nums);
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
