#[cfg(test)]
use super::{arr_to_linked_list, linked_list_to_vec, ListNode};
#[cfg(test)]
use std::boxed::Box;

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn reverse_range_inplace(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_node = ListNode::new(0);
        // head is taken to dummy_node.next
        dummy_node.next = head;
        let mut dummy = Some(Box::new(dummy_node));
        let mut node_m_prev = &mut dummy;
        // use raw pointer
        let mut ptr = 0 as *mut Box<ListNode>;
        for _ in 0..m-1 {
            node_m_prev = &mut node_m_prev.as_mut().unwrap().next;
        }
        let mut rev_head = &mut node_m_prev.as_mut().unwrap().next;
        for _ in m..n {

            // 备份rev_head.next
            // if let Some(rev_head_node) = rev_head {
            //     let rev_head_node_next = rev_head_node.next;
            //     if let Some(rev_head_node_next_node) = rev_head_node_next {
            //         rev_head_node.next = rev_head_node_next_node.next;
            //     }
            // }
            // let mut rev_head_next_node = rev_head_node.next.as_mut().unwrap();
            // // rev_head的next指针越过rev_head_next
            // rev_head.as_mut().unwrap().next = rev_head_next_node.next;
            // // 将rev_head_next插入到node_m_prev和rev_head之间
            // rev_head_next_node.next = node_m_prev_node.next;
            // node_m_prev_node.next = rev_head_node.next;
        }
        return dummy.unwrap().next;
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

    fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut nums: Vec<i32> = Vec::new();
        let mut curr = head;
        while let Some(curr_node) = curr {
            nums.push(curr_node.val);
            curr = curr_node.next;
        }
        nums
    }

    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut nums = Self::linked_list_to_vec(head);
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
        assert_eq!(linked_list_to_vec(output_head), output.to_vec());
    }
}
