use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val,
    }
  }
}

/*
输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出：7 -> 0 -> 8
*/
pub fn run() {
  // 初始化输入用例
  let mut sample_1_1 = ListNode::new(2);
  let mut sample_1_2 = ListNode::new(4);
  let sample_1_3 = ListNode::new(3);
  sample_1_2.next = Some(Box::new(sample_1_3));
  sample_1_1.next = Some(Box::new(sample_1_2));

  // 遍历
  let mut temp = sample_1_1;
  loop {
    println!("{}", temp.val);
    match temp.next {
      Some(node) => temp = *node,
      None => break
    }
  }
}