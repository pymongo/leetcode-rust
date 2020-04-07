use std::boxed::Box;
/* 收获
1. Box<T>感觉像是空指针占位符，*node to unbox Box<ListNode>
2. Option<Box<ListNode>>不知道怎么遍历，一直报错，一会报错borrowed、一会又moved的，不想在浪费时间花在与算法无关的思考上
*/
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

  let mut sample_2_1 = ListNode::new(5);
  let mut sample_2_2 = ListNode::new(6);
  let sample_2_3 = ListNode::new(4);
  sample_2_2.next = Some(Box::new(sample_2_3));
  sample_2_1.next = Some(Box::new(sample_2_2));

  // // 遍历
  // let mut temp = sample_1_1;
  // loop {
  //   println!("{}", temp.val);
  //   match temp.next {
  //     Some(node) => temp = *node, // *node to unbox Box<ListNode>
  //     None => break
  //   }
  // }
  add_two_numbers(Some(Box::new(sample_1_1)), Some(Box::new(sample_2_1)));
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut arr : Vec<i32> = Vec::new();
  // 遍历
  let mut temp_list_1 = l1;
  let mut temp_list_2 = l2;
  loop {
    // println!("list_1.val = {}", *temp_list_1.val);
    // println!("list_2.val = {}", *temp_list_2.val);
    match temp_list_1 {
      Some(node1) => {
        match temp_list_2 {
          Some(node2) => {
            arr.push(node1.val + node2.val);
            temp_list_1 = node1.next;
            temp_list_2 = node2.next;
          },
          None => break
        }
      },
      None => break
    }
  }
  let last = *arr.last().unwrap();
  let mut first_node : Option<Box<ListNode>> = Some(Box::new(ListNode::new(last)));
  temp_list_1 = first_node.clone();
  for (index, num) in arr.iter().enumerate() {
    if index == arr.len() {
      continue;
    }
    println!("{}", num);
    temp_list_2 = Some(Box::new(ListNode::new(*num)));
    temp_list_1.unwrap().next = temp_list_2;
    temp_list_1 = temp_list_2.clone();
  }
  first_node
}