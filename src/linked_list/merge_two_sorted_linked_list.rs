use super::ListNode;
use std::boxed::Box;
use crate::binary_tree::TreeNode;

struct Solution;

impl Solution {
    fn merge_two_lists_without_match(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut curr = &mut head;
        while let (Some(n1), Some(n2)) = (&mut l1, &mut l2) {
            if n1.val < n2.val {
                let next = n1.next.take();
                *curr = l1.take();
                l1 = next;
            } else {
                let next = n2.next.take();
                *curr = l2.take();
                l2 = next;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }
        *curr = l1.or(l2);
        head
    }

    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut curr = &mut head;
        loop {
            // 由于add_two_numbers一题的4个模式识别分支要么l1,l2全改，要么全不改，所以不需要&mut l1
            // 由于第一个分支改l1或l2，二选一的话就l1和l2都需要&mut了
            let val;
            match (&mut l1, &mut l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        val = n1.val;
                        l1 = n1.next.take();
                    } else {
                        val = n2.val;
                        l2 = n2.next.take();
                    };
                    *curr = Some(Box::new(ListNode::new(val)));
                    curr = &mut curr.as_mut().unwrap().next;
                },
                (Some(n1), None) => {
                    *curr = Some(n1.clone());
                    break;
                },
                (None, Some(n2)) => {
                    *curr = Some(n2.clone());
                    break;
                },
                (None, None) => {
                    break;
                }
            }
        }
        head
    }
}