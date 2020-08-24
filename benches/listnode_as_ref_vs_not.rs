#![feature(test)]
extern crate test;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn arr_to_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut curr = &mut head;
    for num in nums {
        *curr = Some(Box::new(ListNode::new(*num)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    head
}

#[bench]
fn traverse_without_as_ref(bencher: &mut test::Bencher) {
    let head = arr_to_linked_list(&[1, 2, 3, 4, 5]);
    bencher.iter(|| {
        let mut nums: Vec<i32> = Vec::new();
        let mut curr = &head;
        while let Some(curr_node) = curr {
            nums.push(curr_node.val);
            curr = &curr_node.next;
        }
    });
}

/*
test traverse_with_as_ref    ... bench:         166 ns/iter (+/- 28)
test traverse_without_as_ref ... bench:         168 ns/iter (+/- 28)
*/
#[bench]
fn traverse_with_as_ref(bencher: &mut test::Bencher) {
    let head = arr_to_linked_list(&[1, 2, 3, 4, 5]);
    bencher.iter(|| {
        let mut nums: Vec<i32> = Vec::new();
        let mut curr = head.as_ref();
        while let Some(curr_node) = curr {
            nums.push(curr_node.val);
            curr = curr_node.next.as_ref();
        }
    });
}
