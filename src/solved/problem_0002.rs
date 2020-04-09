/* 已偷看答案，原因：遍历、生成ListNode时一直语法错，实在是不会遍历/生成Option<Box<ListNode>>数据结构啊 */
// 隐含约束1：如果一个链表较短则在前面补 00，比如 987 + 23 = 987 + 023 = 1010
// 隐含约束2：需要考虑进位
use std::boxed::Box;
/* 收获
1. Box<T>感觉像是空指针占位符，*node to unbox Box<ListNode>
2. Option<Box<ListNode>>不知道怎么遍历，一直报错，一会报错borrowed、一会又moved的，不想在浪费时间花在与算法无关的思考上
3. 还能同时match两个Option(然后2x2四个分支)，学到了！
4. if let 是 match optional的缩写版
if let (x) = optional { f(x) }
等价于：
match optional {
  Some(x) => f(x),
  _ => {}
}
5. 美服第一的代码：current = current.next.as_mut().unwrap() 轻松解压变量<Option<Box>>
6. 学到了用.is_some()方法去遍历Option类型
*/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

#[cfg(feature = "unused")]
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
#[cfg(feature = "unused")]
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

#[cfg(feature = "unused")]
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  // result链表的头节点，仅仅用于返回值(head_node.next)
  let mut head_node : Option<Box<ListNode>> = None;
  // 用于存储生成result链表的节点
  let mut current_node : &mut Option<Box<ListNode>> = &mut head_node;
  let (mut ln1, mut ln2) = (l1, l2);
  // 进入match时sum_or_carry表示进位，在match内经过计算后sum表示sum
  let mut sum_or_carry: i32 = 0;

  loop {
    // 像数字电路datasheet真值表一样...
    match (ln1, ln2) { // 必须要在每个分支都给ln1和ln2复制才能避免moved value的报错
      (Some(node1), Some(node2)) => {
        sum_or_carry = sum_or_carry + node1.val + node2.val;
        ln1 = node1.next;
        ln2 = node2.next;
      },
      (Some(node1), None) => {
        sum_or_carry += node1.val;
        ln1 = node1.next;
        ln2 = None;
      },
      (None, Some(node2)) => {
        sum_or_carry += node2.val;
        ln1 = None;
        ln2 = node2.next;
      },
      (None, None) => {
        if sum_or_carry != 0 {
          *current_node = Some(Box::new(ListNode::new(sum_or_carry)));
        }
        break;
      }
    }
    /* 遍历思路 */
    // 1. current_node一开始为None
    // 2. 初始化current_node的值
    // 3. 将current_node指向current_node.next(因为next为None，所以刚好回到第一步)
    *current_node = Some(Box::new(ListNode::new(sum_or_carry % 10)));
    sum_or_carry /= 10; // 此时的sum_or_carry看作进位值，传入下次迭代
    if let Some(current_node_unboxed) = current_node {
      current_node = &mut current_node_unboxed.next;
    }
  } // end of loop
  head_node
}

// match两个的用法1, 定义了一个匿名函数node_val用于取出当前节点的值，很新颖
#[cfg(feature = "unused")]
fn add_two_numbers_1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut cf = 0;
  let node_val = |x: &Option<Box<ListNode>>| {
    if let Some(node) = x {
      Some(node.val)
    } else {
      None
    }
  };
  let mut res: Option<Box<ListNode>> = None;
  let (mut p1, mut p2, mut pr) = (&l1, &l2, &mut res);

  loop {
    let mut sum = 0;
    match (node_val(p1), node_val(p2)) {
      (Some(val1), Some(val2)) => {
        sum = val1 + val2 + cf;
      }
      (Some(val1), None) => {
        sum = val1 + cf;
      }
      (None, Some(val2)) => {
        sum = val2 + cf;
      }
      (None, None) => {
        if cf == 1 {
          *pr = Some(Box::new(ListNode::new(1)));
        }
        break;
      }
    };

    cf = sum / 10;
    sum = sum - 10 * cf;
    *pr = Some(Box::new(ListNode::new(sum)));
    // println!("p1: {:?}, p2: {:?}, pr: {:?}, sum: {}", node_val(p1), node_val(p2), node_val(pr), sum);
    // p1 = &p1.as_ref().unwrap().next; 另一种写法，要求p1不为None；
    if let Some(node) = p1 {
      p1 = &node.next;
    }
    if let Some(node) = p2 {
      p2 = &node.next;
    }
    if let Some(node) = pr {
      pr = &mut node.next;
    }
  }

  res
}

// 「最佳答案」与解答1类似
#[cfg(feature = "unused")]
pub fn add_two_numbers_2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut sum = 0;
  let (mut l1, mut l2) = (l1, l2);
  let mut l = None;  // 在上下文中可以推断类型
  let mut p = &mut l;

  loop {
    match (l1, l2) {   // l1,l2是`move`语义匹配，每次匹配前都需要初始化值
      (Some(v1), Some(v2)) => {
        sum += v1.val + v2.val;
        l1 = v1.next;
        l2 = v2.next;
      }
      (Some(v1), None) => {
        sum += v1.val;
        l1 = v1.next;
        l2 = None;
      }
      (None, Some(v2)) => {
        sum += v2.val;
        l2 = v2.next;
        l1 = None;
      }
      (None, None) => {
        break;
      }
    }
    *p = Some(Box::new(ListNode::new(sum % 10))); //不管sum是否大于10，都可以使用sum%10的值来构建新“节点“
    sum /= 10; // 获取进位值，否则初始为0
    if let Some(p_box_node) = p {
      // 遍历思想：赋值好当前节点后，把p指向下一个节点(None)
      p = &mut p_box_node.next
    }
    /* 批注 if let(p_box_node) = p { f(p_box_node) } 等价于
    match p {
      Some(i) => f(p_box_node),
      _ => {},
    };
    */
  }
  if sum != 0 {
    *p = Some(Box::new(ListNode::new(sum)));
  }

  l
}

// 「美服第一」优化很多
#[cfg(feature = "unused")]
pub fn add_two_numbers_3(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut head = ListNode::new(0);
  let mut curr = &mut head;
  let mut carry = 0;

  loop {
    match (l1, l2) {
      (Some(mut v1), Some(mut v2)) => {
        let mut val = v1.val + v2.val + carry;

        if val >= 10 {
          val -= 10;
          carry = 1;
        } else {
          carry = 0;
        }

        let node = ListNode::new(val);
        curr.next = Some(Box::new(node));
        l1 = v1.next.take();
        l2 = v2.next.take();
      },
      (Some(mut v1), None) => {
        let mut val = v1.val + carry;

        if val >= 10 {
          val -= 10;
          carry = 1;
        } else {
          carry = 0;
        }

        let node = ListNode::new(val);
        curr.next = Some(Box::new(node));
        l1 = v1.next.take();
        l2 = None;
      },
      (None, Some(mut v2)) => {
        let mut val = v2.val + carry;

        if val >= 10 {
          val -= 10;
          carry = 1;
        } else {
          carry = 0;
        }

        let node = ListNode::new(val);
        curr.next = Some(Box::new(node));
        l2 = v2.next.take();
        l1 = None;
      },
      (None, None) => {
        if carry == 0 {
          break;
        }

        let node = ListNode::new(1);
        curr.next = Some(Box::new(node));
        break;
      },
    }

    curr = curr.next.as_mut().unwrap();
  }

  head.next
}

// 「国服第一」
#[cfg(feature = "unused")]
pub fn add_two_numbers_4(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut dump_head = ListNode::new(0);
  let mut current = &mut dump_head;
  let mut carry = 0;
  let mut p = l1.as_ref();
  let mut q = l2.as_ref();

  while p.is_some() || q.is_some() {
    let sum = match(&p, &q) {
      (Some(l1), Some(l2)) => l1.val + l2.val + carry,
      (Some(l1), None) => l1.val + carry,
      (None, Some(l2)) => l2.val + carry,
      (None, None) => 0 + carry,
    };

    carry = sum / 10;
    current.next = Some(Box::new(ListNode::new(sum % 10)));
    current = current.next.as_mut().unwrap();

    if p.is_some() && p.unwrap().next.is_some() {
      p = p.unwrap().next.as_ref();
    } else {
      p = None;
    }

    if q.is_some() && q.unwrap().next.is_some() {
      q = q.unwrap().next.as_ref();
    } else {
      q = None;
    }
  }

  if carry > 0 {
    current.next = Some(Box::new(ListNode::new(carry)));
  }

  dump_head.next
}

/*
crate leetcode的作者提供了一种效率较低，但是也是一种解决思路的方案
先把「难以遍历」的Option
*/
#[cfg(feature = "unused")]
fn list_to_linkedlist(l: Option<Box<ListNode>>) -> std::collections::LinkedList<i32> {
  let mut result = std::collections::LinkedList::new();
  let mut curr = l;

  while curr != None {
    let inner = curr.unwrap();
    result.push_back(inner.val);
    curr = inner.next;
  }

  result
}

#[cfg(feature = "unused")]
fn linkedlist_to_list(mut ll: std::collections::LinkedList<i32>) -> Option<Box<ListNode>> {
  let mut tail = None;

  while ll.front().is_some() {
    let v = *ll.front().unwrap();
    let node = ListNode { val: v, next: tail };
    tail = Some(Box::new(node));
    ll.pop_front();
  }

  tail
}
