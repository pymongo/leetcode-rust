use super::ListNode;

/// https://leetcode.com/problems/merge-two-sorted-lists/
fn merge_two_sorted_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
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
        curr = &mut curr.as_mut()?.next;
    }
    *curr = l1.or(l2);
    head
}

/** https://leetcode.com/problems/sort-list/
Rust暂且就先放O(logn)空间复杂度自上而下的递归归并排序把，即便上unsafe也很难挪动链表节点
用unsafe的原因，需要多个可变借用(head和slow)或链表某个节点的所有权要被slow和mid同时拥有，还需要take()切断

用unsafe的根本原因，快慢双指针遍历链表时，慢指针遍历到链表中点后要切断链表，所以慢指针要用可变借用
但是Rust所有权限制当链表存在一个可变借用时，就不能有其它的可变或不可变借用，所以快指针将无法定义。
此时的慢指针是个可变引用就类似RwLock或RWMutex的write_lock，当拥有一个WLock时不能拥有其他RLock/WLock
用unsafe将慢指针定义成原始指针，绕开Rust所有权机制的限制

```text
let mut slow = head.as_mut();
               ---- mutable borrow occurs here
let mut fast = head.as_ref()?.next.as_ref();
               ^^^^ immutable borrow occurs here
```
*/
fn sort_linked_list_top_down_recursive(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.as_ref()?.next == None {
        return head;
    }
    let mut slow = &mut head as *mut Option<Box<ListNode>>;
    // 因为right_part取的是慢指针的next，所以快指针初始值要比慢指针多一步
    let mut fast = head.as_ref()?.next.as_ref();
    let mid = unsafe {
        while fast.is_some() && fast.as_ref()?.next.is_some() {
            slow = &mut (*slow).as_mut()?.next as *mut _;
            fast = fast?.next.as_ref()?.next.as_ref();
        }
        let mid = (*slow).as_mut()?.next.take();
        // cut left_part_list and right_part_list
        (*slow).as_mut()?.next = None;
        mid
    };
    let left_part = sort_linked_list_top_down_recursive(head);
    let right_part = sort_linked_list_top_down_recursive(mid);
    merge_two_sorted_lists(left_part, right_part)
}

fn merge_two_lists_match_solution(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
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
                curr = &mut curr.as_mut()?.next;
            }
            (Some(n1), None) => {
                *curr = Some(n1.clone());
                break;
            }
            (None, Some(n2)) => {
                *curr = Some(n2.clone());
                break;
            }
            (None, None) => {
                break;
            }
        }
    }
    head
}
