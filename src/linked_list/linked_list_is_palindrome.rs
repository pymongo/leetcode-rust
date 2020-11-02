use super::ListNode;

struct Solution;

impl Solution {
    // 仅仅适用于node.val在0~9之间
    #[cfg(FALSE)]
    fn is_palindrome_only_one_digit(head: Option<Box<ListNode>>) -> bool {
        let mut asc = 0;
        let mut desc = 0;
        let mut desc_base = 1;

        let mut curr = head.as_ref();
        while let Some(curr_node) = curr {
            asc = 10 * asc + curr_node.val;
            desc += curr_node.val * desc_base;
            desc_base *= 10;
            curr = curr_node.next.as_ref();
        }
        asc == desc
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut nums = Vec::new();
        let mut curr = &head;
        while let Some(curr_inner) = curr {
            nums.push(curr_inner.val);
            curr = &curr_inner.next;
        }
        if nums.is_empty() {
            // 空链表也算回文
            return true;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] != nums[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
}

#[cfg(test)]
const TESTCASES: [(&[i32], bool); 1] = [(&[0, 0], true)];

#[test]
fn test_linked_list_is_palindrome() {
    for &(ln, expected) in TESTCASES.iter() {
        let head = super::arr_to_linked_list(ln);
        assert_eq!(Solution::is_palindrome(head), expected);
    }
}
