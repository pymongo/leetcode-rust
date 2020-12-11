//! https://leetcode.com/problems/insertion-sort-list/
use super::ListNode;

fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut last_sorted = &mut dummy.as_mut()?.next as *mut Option<Box<ListNode>>;
    unsafe {
        // TODO 想一边遍历一边修改就不能while let Some()遍历吗？
        while (*last_sorted).as_ref()?.next.is_some() {
            if (*last_sorted).as_ref()?.val <= (*last_sorted).as_ref()?.next.as_ref()?.val {
                last_sorted = &mut (*last_sorted).as_mut()?.next as *mut _;
            } else {
                let mut unsorted = (*last_sorted).as_mut()?.next.take();
                (*last_sorted).as_mut()?.next = unsorted.as_mut()?.next.take();
                let mut ptr = &mut dummy as *mut Option<Box<ListNode>>;
                while (*ptr).as_ref()?.next.as_ref()?.val <= unsorted.as_ref()?.val {
                    ptr = &mut (*ptr).as_mut()?.next as *mut _;
                }
                // 将新节点插入到ptr的后一个位置
                unsorted.as_mut()?.next = (*ptr).as_mut()?.next.take();
                (*ptr).as_mut()?.next = unsorted;
            }
        }
    }
    dummy?.next
}

fn insertion_sort_list_dirty_solution(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut nums: Vec<i32> = Vec::new();
    let mut curr = head;
    while let Some(curr_node) = curr {
        nums.push(curr_node.val);
        curr = curr_node.next;
    }
    nums.sort_unstable();
    let mut head = None;
    let mut curr = &mut head;
    for num in nums.into_iter() {
        *curr = Some(Box::new(ListNode::new(num)));
        curr = &mut curr.as_mut()?.next;
    }
    head
}
