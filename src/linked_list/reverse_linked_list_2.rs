use super::{arr_to_linked_list, linked_list_to_vec, ListNode};
use std::boxed::Box;

struct Solution;

type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn reverse_range_inplace(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        unsafe {
            let mut dummy = Some(Box::new(ListNode::new(0)));
            dummy.as_mut().unwrap().next = head;
            let mut node_m_prev = &mut dummy as *mut Node;
            for _ in 0..m-1 {
                node_m_prev = &mut (*node_m_prev).as_mut().unwrap().next as *mut Node;
            }
            // rev_head会一直往前挪，但是节点的值和内存地址不变
            let rev_head = &mut (*node_m_prev).as_mut().unwrap().next as *mut Node;
            for _ in m..n {
                // 备份rev_head.next
                let rev_head_next = &mut (*rev_head).as_mut().unwrap().next as *mut Node;
                // rev_head的next指针越过rev_head_next
                (*rev_head).as_mut().unwrap().next = (*rev_head_next).as_mut().unwrap().next.take();
                // 将rev_head_next插入到node_m_prev和rev_head之间
                (*rev_head_next).as_mut().unwrap().next = (*node_m_prev).as_mut().unwrap().next.take();
                (*node_m_prev).as_mut().unwrap().next = (*rev_head_next).take();
                // FIXME [1,4,3,2,5]经过一轮后会变成[1,4,2]，丢失节点的原因可能是take导致的，换成mem::swap也一样
            }
            return dummy.unwrap().next;
        }
    }

    fn vec_to_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut curr = &mut head;
        for num in nums {
            *curr = Some(Box::new(ListNode::new(num)));
            curr = &mut curr.as_mut().unwrap().next;
        }
        head
    }

    fn linked_list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut nums: Vec<i32> = Vec::new();
        let mut curr = head;
        while let Some(curr_node) = curr {
            nums.push(curr_node.val);
            curr = &curr_node.next;
        }
        nums
    }

    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut nums = Self::linked_list_to_vec(&head);
        // 因为入参m和n是从1开始编号的，所以这里要减1
        let mut left = (m-1) as usize;
        let mut right = (n-1) as usize;
        while left < right {
            let temp = nums[left];
            nums[left] = nums[right];
            nums[right] = temp;
            left += 1;
            right -= 1;
        }
        Self::vec_to_linked_list(nums)
    }

}

#[cfg(test)]
const TEST_CASES: [(&[i32], i32, i32, &[i32]); 1] = [(&[1, 2, 3, 4, 5], 2, 4, &[1, 4, 3, 2, 5])];

#[test]
fn test_traverse_two_list_node() {
    for &(input, m, n, output) in &TEST_CASES {
        let head = arr_to_linked_list(input);
        let output_head = Solution::reverse_between(head, m, n);
        assert_eq!(linked_list_to_vec(&output_head), output.to_vec());
    }
}

#[test]
#[ignore]
fn test_reverse_range_inplace() {
    for &(input, m, n, output) in &TEST_CASES {
        let head = arr_to_linked_list(input);
        let output_head = Solution::reverse_range_inplace(head, m, n);
        assert_eq!(linked_list_to_vec(&output_head), output.to_vec());
    }
}
