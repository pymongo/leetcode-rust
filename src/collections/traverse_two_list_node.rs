//! https://leetcode.com/problems/add-two-numbers/
//! 已偷看答案，原因：遍历、生成ListNode时一直语法错，实在是不会遍历/生成Option<Box<ListNode>>数据结构啊
//! 隐含约束1：遍历时要注意，两个输入的链表长度可能不相等，最长的链表遍历完才算遍历结束
//! 隐含约束2：如果一个链表较短则在前面补 00，比如 987 + 23 = 987 + 023 = 1010
//! 隐含约束3：需要考虑进位
/* 收获
1. Box<T>感觉像是空指针占位符(Allocates memory on the heap)，*node to unbox Box<ListNode>
2. Option<Box<ListNode>>不知道怎么遍历，一直报错，一会报错borrowed、一会又moved的，不想在浪费时间花在与算法无关的思考上
3. 还能同时match两个Option(通过排列组合，2个option都用Some和None分支总共4个分支)，学到了！
4. if let 是 match optional的缩写版
if let (x) = optional { f(x) }
等价于：
match optional {
  Some(x) => f(x),
  _ => {}
}
5. 全球服第一的代码：current = current.next.as_mut().unwrap() 轻松解压变量<Option<Box>>
6. 学到了用.is_some()方法去遍历Option类型
*/
use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 仅用于生成测试用例的Helper方法
#[cfg(test)]
fn vector_to_list_node(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    // result链表(返回值链表)的头节点，用于记住result链表的「第一个节点」
    let mut head_node: Option<Box<ListNode>> = None;
    // 遍历时的当前节点，初始值是result链表的head(第一个节点)
    let mut current_node: &mut Option<Box<ListNode>> = &mut head_node;
    for number in numbers {
        // 通过ListNode给构造方法传入val创建一个新的ListNode，next指针为None
        *current_node = Some(Box::new(ListNode::new(number)));
        if let Some(current_node_unwrap) = current_node {
            // 将current_node指针改为current_node的下一个节点(None)
            current_node = &mut current_node_unwrap.next;
        }
    }
    head_node
}

// 仅用于单元测试中验证返回值的Helper方法
#[cfg(test)]
fn list_node_to_vector(list_node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut current_node = list_node;
    let mut numbers: Vec<i32> = Vec::new();
    loop {
        match current_node {
            Some(node) => {
                numbers.push(node.val);
                current_node = node.next;
            }
            None => break,
        }
    }
    numbers
}

#[test]
fn test_helper_methods() {
    let list_node = vector_to_list_node(vec![2, 4, 3]);
    assert_eq!(list_node_to_vector(list_node), vec![2, 4, 3]);
}

#[allow(dead_code)]
fn traverse_two_list_node(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // result链表(返回值链表)的头节点，用于记住result链表的「第一个节点」
    let mut head_node: Option<Box<ListNode>> = None;
    // 遍历时的当前节点，初始值是result链表的head(第一个节点)
    let mut current_node: &mut Option<Box<ListNode>> = &mut head_node;
    let (mut ln1, mut ln2) = (l1, l2);
    // 进入match时sum_or_carry表示进位，在match内经过计算后sum表示sum
    let mut sum_or_carry: i32 = 0;

    loop {
        // 像数字电路datasheet真值表一样...
        match (ln1, ln2) {
            // 必须要在每个分支都给ln1和ln2复制才能避免moved value的报错
            (Some(node1), Some(node2)) => {
                sum_or_carry = sum_or_carry + node1.val + node2.val;
                ln1 = node1.next;
                ln2 = node2.next;
            }
            (Some(node1), None) => {
                sum_or_carry += node1.val;
                ln1 = node1.next;
                ln2 = None;
            }
            (None, Some(node2)) => {
                sum_or_carry += node2.val;
                ln1 = None;
                ln2 = node2.next;
            }
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
        if let Some(current_node_unwrap) = current_node {
            current_node = &mut current_node_unwrap.next;
        }
    } // end of loop
    head_node
}

/*
输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
输出： 7 -> 0 -> 8
*/
#[cfg(test)]
const TEST_CASES: [(&[i32], &[i32], &[i32]); 1] = [(&[2, 4, 3], &[5, 6, 4], &[7, 0, 8])];

#[test]
fn test_traverse_two_list_node() {
    for case in &TEST_CASES {
        let input_list_node_1 = vector_to_list_node(case.0.iter().cloned().collect());
        let input_list_node_2 = vector_to_list_node(case.1.iter().cloned().collect());
        let output_list_node = traverse_two_list_node(input_list_node_1, input_list_node_2);
        let expected_vector: Vec<i32> = case.2.iter().cloned().collect();
        assert_eq!(list_node_to_vector(output_list_node), expected_vector);
    }
}

// 「国服第一」
#[cfg(not)]
pub fn cn_best_answer(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dump_head = ListNode::new(0);
    let mut current = &mut dump_head;
    let mut carry = 0;
    let mut p = l1.as_ref();
    let mut q = l2.as_ref();

    while p.is_some() || q.is_some() {
        let sum = match (&p, &q) {
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

// 「全球服第一」，等下！题目的入参l1、l2没有mut修饰，这个答案直接改入参居然也通过了...
#[cfg(not)]
pub fn en_best_answer(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
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
            }
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
            }
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
            }
            (None, None) => {
                if carry == 0 {
                    break;
                }

                let node = ListNode::new(1);
                curr.next = Some(Box::new(node));
                break;
            }
        }

        curr = curr.next.as_mut().unwrap();
    }

    head.next
}
