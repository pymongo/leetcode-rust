// use super::{arr_to_linked_list, linked_list_to_vec, ListNode};
use super::ListNode;
struct Solution;

type Node = Option<Box<ListNode>>;

impl Solution {
    // pub fn reverse_range_inplace(
    //     head: Option<Box<ListNode>>,
    //     m: i32,
    //     _n: i32,
    // ) -> Option<Box<ListNode>> {
    //     unsafe {
    //         let mut dummy = Some(Box::new(ListNode::new(0)));
    //         dummy.as_mut()?.next = head;
    //         let mut node_m_prev = &mut dummy as *mut Node;
    //         for _ in 0..m - 1 {
    //             node_m_prev = &mut (*node_m_prev).as_mut()?.next as *mut Node;
    //         }
    //         // rev_head会一直往前挪，但是节点的值和内存地址不变
    //         let rev_head = &mut (*node_m_prev).as_mut()?.next as *mut Node;
    //         // for _ in m..n {
    //         for _ in 0..1 {
    //             // 备份rev_head.next
    //             let rev_head_next = &mut (*rev_head).as_mut()?.next as *mut Node;
    //
    //             // rev_head的next指针越过rev_head_next
    //             (*rev_head).as_mut()?.next = (*rev_head_next).as_mut()?.next.take();
    //
    //             // 将rev_head_next插入到node_m_prev和rev_head之间
    //             (*rev_head_next).as_mut()?.next = (*node_m_prev).as_mut()?.next.take();
    //             (*node_m_prev).as_mut()?.next = (*rev_head_next).take();
    //             // FIXME [1,4,3,2,5]经过一轮后会变成[1,4,2]，丢失节点的原因可能是take导致的，换成mem::swap也一样
    //         }
    //         return dummy?.next;
    //     }
    // }

    // reverse mth..nth node in linked_list
    #[cfg(FALSE)]
    fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut()?.next = head;

        /* m=2, n=4
        Before: dummy->1->2->3->4->5,
                    ^
        After:  dummy->1->[2->3->4]->5,
                       ^
        其实指针前移的过程根本就没有任何修改，不应该用mut，但是为了后面的代码rev_head能是可变的，所以要从源头开始将mut传播下去
        */
        let mut node_m_prev = dummy.as_mut()?;
        for _ in 0..m - 1 {
            node_m_prev = node_m_prev.next.as_mut()?;
        }

        // 需要反转的第一个节点
        let mut rev_head = node_m_prev.next.as_mut()?;
        for _ in m..n {
            // 备份rev_head.next
            // let mut rev_head_next = rev_head.next.as_mut()?;
            // rev_head的next指针越过rev_head_next
            std::mem::swap(rev_head.next, &mut rev_head.next.as_mut()?.next);
            // rev_head.next = rev_head_next.next;

            // (*rev_head).as_mut().unwrap().next = (*rev_head_next).as_mut().unwrap().next.take();
            // // 将rev_head_next插入到node_m_prev和rev_head之间
            // (*rev_head_next).as_mut().unwrap().next = (*node_m_prev).as_mut().unwrap().next.take();
            // (*node_m_prev).as_mut().unwrap().next = (*rev_head_next).take();
        }
        return dummy?.next;
    }

    /* 请无视这种链表转数组，数组操作完后再转为链表的解法
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
        let mut left = (m - 1) as usize;
        let mut right = (n - 1) as usize;
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
        Self::vec_to_linked_list(nums)
    }
    */
}

#[cfg(test)]
const TEST_CASES: [(&[i32], i32, i32, &[i32]); 1] = [(&[1, 2, 3, 4, 5], 2, 4, &[1, 4, 3, 2, 5])];

// #[test]
// fn test_traverse_two_list_node() {
//     for &(input, m, n, output) in &TEST_CASES {
//         let head = arr_to_linked_list(input);
//         let output_head = Solution::reverse_between(head, m, n);
//         assert_eq!(linked_list_to_vec(&output_head), output.to_vec());
//     }
// }
//
// #[test]
// #[ignore]
// fn test_reverse_range_inplace() {
//     for &(input, m, n, output) in &TEST_CASES {
//         let head = arr_to_linked_list(input);
//         let output_head = Solution::reverse_range_inplace(head, m, n);
//         assert_eq!(linked_list_to_vec(&output_head), output.to_vec());
//     }
// }
