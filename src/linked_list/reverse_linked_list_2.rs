use super::{ListNode, linked_list_to_vec};

#[test]
fn test_reverse_range_inplace() {
    const TEST_CASES: [(&[i32], i32, i32, &[i32]); 1] = [(&[1, 2, 3, 4, 5], 2, 4, &[1, 4, 3, 2, 5])];

    for &(input, m, n, output) in &TEST_CASES {
        let head = super::arr_to_linked_list(input);
        let output_head = unsafe { reverse_range_inplace(head, m, n) };
        assert_eq!(linked_list_to_vec(&output_head), output.to_vec());
    }
}

unsafe fn reverse_range_inplace(head: Option<Box<ListNode>>, m: i32, n: i32, ) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut node_m_prev = &mut dummy as *mut Option<Box<ListNode>>;
    for _ in 0..m - 1 {
        node_m_prev = &mut (*node_m_prev).as_mut()?.next as *mut Option<Box<ListNode>>;
    }
    let mut rev_head = &mut (*node_m_prev).as_mut()?.next as *mut Option<Box<ListNode>>;
    for i in m..n {
        println!("{}: rev_head: {:?}", line!(), linked_list_to_vec(&(*rev_head)));

        let mut rev_head_next = (*rev_head).as_mut()?.next.take();
        println!("{}: dummy: {:?}", line!(), linked_list_to_vec(&dummy));
        // println!("{}: node_m_prev: {:?}", line!(), linked_list_to_vec(&(*node_m_prev)));
        println!("{}: rev_head_next: {:?}", line!(), linked_list_to_vec(&rev_head_next));
        println!();
        // rev_head now is single node

        (*rev_head).as_mut()?.next = rev_head_next.as_mut()?.next.take();
        println!("{}: dummy: {:?}", line!(), linked_list_to_vec(&dummy));
        // println!("{}: node_m_prev: {:?}", line!(), linked_list_to_vec(&(*node_m_prev)));
        // println!("{}: rev_head: {:?}", line!(), linked_list_to_vec(&(*rev_head)));
        println!("{}: rev_head_next: {:?}", line!(), linked_list_to_vec(&rev_head_next));
        println!();
        // rev_head_next now is single node

        // 将rev_head_next插入到node_m_prev和rev_head中间时一定要先连右边(rev_head_next右边接上rev_head)
        // 先写`(*node_m_prev).as_mut()?.next = rev_head_next;`会导致上一步中rev_head的2->4->5中的4->5丢失掉
        rev_head_next.as_mut()?.next = (*node_m_prev).as_mut()?.next.take();
        println!("{}: dummy: {:?}", line!(), linked_list_to_vec(&dummy));
        // println!("{}: node_m_prev: {:?}", line!(), linked_list_to_vec(&(*node_m_prev)));
        // println!("{}: rev_head: {:?}", line!(), linked_list_to_vec(&(*rev_head)));
        println!("{}: rev_head_next: {:?}", line!(), linked_list_to_vec(&rev_head_next));
        println!();

        (*node_m_prev).as_mut()?.next = rev_head_next;
        println!("{}: dummy: {:?}", line!(), linked_list_to_vec(&dummy));
        // println!("{}: node_m_prev: {:?}", line!(), linked_list_to_vec(&(*node_m_prev)));
        // println!("{}: rev_head: {:?}", line!(), linked_list_to_vec(&(*rev_head)));
        println!("{}: rev_head_next: moved to node_m_prev.next", line!());
        println!();

        // node_m_prev = &mut (*node_m_prev).as_mut()?.next as *mut _;
        // rev_head = &mut (*node_m_prev).as_mut()?.next as *mut _;
        // dbg!(&(*rev_head));
        // dbg!(&(*node_m_prev));
        rev_head = &mut (*rev_head).as_mut()?.next as *mut _;
        println!("{}: dummy: {:?}", line!(), linked_list_to_vec(&dummy));
        // println!("{}: node_m_prev: {:?}", line!(), linked_list_to_vec(&(*node_m_prev)));
        println!("{}: rev_head: {:?}", line!(), linked_list_to_vec(&(*rev_head)));
        println!("{}: rev_head_next: moved to node_m_prev.next", line!());
        println!("== current loop end\n");
    }
    return dummy?.next;
}
