use super::ListNode;

/// https://leetcode-cn.com/problems/kth-node-from-end-of-list-lcci/
/// 找出单向链表中倒数第 k 个节点，返回该节点的值，容易想到用滑动窗口的方法，宽度为k窗口往右划底时，左边界就是倒数第k个节点
fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
    fn helper(head: Option<Box<ListNode>>, k: i32) -> Option<i32> {
        let dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut left = dummy.as_ref();
        let mut right = dummy.as_ref();
        // 扩宽右窗口，使得窗口宽度等于k
        for _ in 0..k {
            right = right?.next.as_ref();
        }
        while right?.next.is_some() {
            right = right?.next.as_ref();
            left = left?.next.as_ref();
        }
        Some(left?.next.as_ref()?.val)
    }
    helper(head, k).unwrap_or_default()
}

#[test]
fn test_kth_to_last() {
    const TEST_CASES: [(&[i32], i32, i32); 2] = [(&[1, 2, 3, 4, 5], 2, 4), (&[1], 1, 1)];
    for &(nums, k, output) in TEST_CASES.iter() {
        assert_eq!(kth_to_last(super::arr_to_linked_list(nums), k), output);
    }
}

/** https://leetcode.com/problems/remove-nth-node-from-end-of-list/
## Rust挪动链表节点的要点
1. 双指针(快慢双指针 or 滑动窗口)
如果类似获取链表中点值这样的只读操作，用两个不可变引用没问题
但是一旦像这题左指针需要可变，右指针不可变，就会违反所有权机制——只能存在一个可变或多个不可变，类似RwLock或Go的RwMutex
当写锁存在时不能有其它读锁，需要unsafe绕开所有权检查
2. 使用?不方便找到哪一行因None而unwrap
我的经验是首先全用unwrap，等代码调试对了后再改成?
*/
fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut left = &mut dummy as *mut Option<Box<ListNode>>;
    let mut right = dummy.as_ref();
    for _ in 0..n {
        right = right?.next.as_ref();
    }
    while right?.next.is_some() {
        right = right?.next.as_ref();
        left = unsafe { &mut (*left).as_mut()?.next } as *mut _;
    }
    unsafe {
        // or: (*left).as_mut()?.next = (*left).as_mut()?.next.take()?.next;
        (*left).as_mut()?.next = (*left).as_mut()?.next.as_mut()?.next.take();
    }
    dummy?.next
}

#[test]
fn test_remove_nth_from_end() {
    use super::arr_to_linked_list;
    const TEST_CASES: [(&[i32], i32, &[i32]); 2] =
        [(&[1, 2, 3, 4, 5], 2, &[1, 2, 3, 5]), (&[1], 1, &[])];
    for &(nums, k, output) in TEST_CASES.iter() {
        let (nums, output) = (arr_to_linked_list(nums), arr_to_linked_list(output));
        assert_eq!(remove_nth_from_end(nums, k), output);
    }
}
