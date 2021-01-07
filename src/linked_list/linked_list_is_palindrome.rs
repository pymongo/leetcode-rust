use super::ListNode;

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut nums = Vec::new();
    let mut curr = &head;
    while let Some(curr_inner) = curr {
        nums.push(curr_inner.val);
        curr = &curr_inner.next;
    }
    let nums_before_reverse = nums.clone();
    nums.reverse();
    nums_before_reverse == nums
}

#[cfg(test)]
const TEST_CASES: [(&[i32], bool); 1] = [(&[0, 0], true)];

#[test]
fn test_linked_list_is_palindrome() {
    for &(ln, expected) in TEST_CASES.iter() {
        let head = super::arr_to_linked_list(ln);
        assert_eq!(is_palindrome(head), expected);
    }
}

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
