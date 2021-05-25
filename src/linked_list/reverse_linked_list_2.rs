use super::ListNode;

#[test]
#[should_panic]
fn test_reverse_range_inplace() {
    const TEST_CASES: [(&[i32], i32, i32, &[i32]); 3] = [
        (&[1, 2, 3, 4, 5], 1, 5, &[5, 4, 3, 2, 1]),
        (&[1, 2, 3, 4], 1, 4, &[4, 3, 2, 1]),
        (&[1, 2, 3, 4, 5], 2, 4, &[1, 4, 3, 2, 5]),
    ];

    for (input, m, n, output) in TEST_CASES {
        let head = super::arr_to_linked_list(input);
        let output_head = unsafe { reverse_range_inplace(head, m, n) };
        assert_eq!(super::linked_list_to_vec(&output_head), output.to_vec());
    }
}

unsafe fn reverse_range_inplace(
    head: Option<Box<ListNode>>,
    m: i32,
    n: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut node_m_prev = &mut dummy as *mut Option<Box<ListNode>>;
    for _ in 0..m - 1 {
        node_m_prev = &mut (*node_m_prev).as_mut()?.next as *mut Option<Box<ListNode>>;
    }
    let mut rev_head = &mut (*node_m_prev).as_mut()?.next as *mut Option<Box<ListNode>>;
    for _ in m..n {
        dbg!(line!());
        dbg!(&(*rev_head));
        if (*rev_head).is_none() {
            break;
        }
        let mut rev_head_next = (*rev_head).as_mut()?.next.take();
        dbg!(line!());
        if rev_head_next.is_none() {
            dbg!(line!());
            // 相比C++/Python相同解法的代码，Rust要多写的部分
            (*rev_head).as_mut()?.next = (*node_m_prev).as_mut()?.next.take();
            (*node_m_prev).as_mut()?.next = (*rev_head).take();
            break;
        } else {
            dbg!(line!());
            (*rev_head).as_mut()?.next = rev_head_next.as_mut()?.next.take();
        }
        dbg!(line!());
        rev_head_next.as_mut()?.next = (*node_m_prev).as_mut()?.next.take();
        dbg!(line!());
        (*node_m_prev).as_mut()?.next = rev_head_next;
        dbg!(line!());
        // 相比C++/Python相同解法的代码，Rust要多写的部分
        dbg!(&(*rev_head));
        if (*rev_head).as_mut()?.next.is_none() {
            let temp = (*node_m_prev).as_mut()?.val;
            (*node_m_prev).as_mut()?.val = (*rev_head).as_mut()?.val;
            dbg!(line!());
            (*rev_head).as_mut()?.next = (*node_m_prev).as_mut()?.next.take();
            dbg!(line!());
            (*rev_head).as_mut()?.val = temp;
            dbg!(line!());
            (*node_m_prev).as_mut()?.next = (*rev_head).take();
            dbg!(line!());
            break;
        }
        rev_head = &mut (*rev_head).as_mut()?.next as *mut _;
        dbg!(&(*rev_head));
    }
    dbg!(&dummy);
    dummy?.next
}
