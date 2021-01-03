use super::ListNode;

/// https://leetcode.com/problems/partition-list/
fn partition_list(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    // linked_list less than x
    let mut less_than_x_head = None;
    let mut less_than_x = &mut less_than_x_head;
    // linked_list greater or equal than x
    let mut greater_than_x_head = None;
    let mut greater_than_x = &mut greater_than_x_head;
    while let Some(mut box_node) = head {
        head = box_node.next.take();
        if box_node.val < x {
            *less_than_x = Some(box_node);
            less_than_x = &mut less_than_x.as_mut()?.next;
        } else {
            *greater_than_x = Some(box_node);
            greater_than_x = &mut greater_than_x.as_mut()?.next;
        }
    }
    *less_than_x = greater_than_x_head;
    less_than_x_head
}

#[test]
fn test_partition_list() {
    use super::arr_to_linked_list;
    const TEST_CASES: [(&[i32], i32, &[i32]); 1] = [(&[1, 4, 3, 2, 5, 2], 3, &[1, 2, 2, 4, 3, 5])];
    for &(head, x, output) in TEST_CASES.iter() {
        let (head, output) = (arr_to_linked_list(head), arr_to_linked_list(output));
        assert_eq!(partition_list(head, x), output);
    }
}
