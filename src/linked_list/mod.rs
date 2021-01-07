/*!
# 链表

## 侵入式和非侵入式

Rust标准库的双向链表和C++ STL的链表一样都是较为简单的非侵入式链表(non-intrusive)，侵入式链表好处的内存利用率高，缺点是指针生命周期管理太复杂

## 非随机访问数据结构

array is a 'random access data structure'

数组可以通过下标乘元素长度计算出内存偏移地址去任意下标的访问数据，但是链表访问第N个节点的数据前必须遍历前N-1个元素

所以数组是随机访问数据结构，而链表是非随机访问数据结构

## Rust链表可以用dummyHead去遍历也可以不用
*/
mod add_two_linked_list;
mod insertion_sort_linked_list;
mod linked_list_is_palindrome;
mod merge_two_sorted_linked_list;
mod middle_of_linked_list;
mod nth_node_from_end;
mod partition_list;
mod reverse_linked_list;
mod reverse_linked_list_2;
mod swap_nodes_in_pairs;

/// non-interest single_linked_list Node
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", linked_list_to_vec(&Some(Box::new(self.clone()))))
    }
}

// Option<T>的设计优点之一: 链表题可以不使用dummy_head也能生成链表后返回头部
fn arr_to_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut curr = &mut head;
    for num in nums {
        *curr = Some(Box::new(ListNode::new(*num)));
        curr = &mut curr.as_mut()?.next;
    }
    head
}

/**
```java
ListNode dummy = new ListNode();
ListNode cur = dummyHead;
for (int num: nums) {
    cur.next = new ListNode(num);
    cur = cur.next;
}
return dummy.next;
```
*/
#[cfg(FALSE)]
fn arr_to_linked_list_with_dummy(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut dummy;
    for num in nums {
        let curr_node = curr.as_mut()?;
        curr_node.next = Some(Box::new(ListNode::new(*num)));
        curr = &mut curr_node.next;
    }
    dummy?.next
}

/**
```java
static int[] listNodeToArray(ListNode head) {
    List<Integer> nums = new ArrayList<>();
    ListNode curr = head;
    while (curr != null) {
        nums.add(curr.val);
        curr = curr.next;
    }
    return nums.stream().mapToInt(i -> i).toArray();
}
```
*/
fn linked_list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    // 由于链表转数组只需要读链表不需要修改链表各节点，所以curr=head即可而不是curr=&mut head，而且代码也简洁多了
    // 但是一旦用了dummyHead，可能只需要修改head一个节点，但是出于遍历原因可变引用需要往后传染，所以后面的节点被迫也用可变引用
    let mut curr = head;
    while let Some(curr_node) = curr {
        nums.push(curr_node.val);
        curr = &curr_node.next;
    }
    nums
}

#[test]
fn test_arr_to_linked_list() {
    let head = arr_to_linked_list(&[1, 2, 3, 4, 5]);
    let nums_vec = linked_list_to_vec(&head);
    assert_eq!(nums_vec, vec![1, 2, 3, 4, 5])
}
