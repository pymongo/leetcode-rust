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
*/
#[cfg(test)]
use super::{arr_to_linked_list, linked_list_to_vec, ListNode};
#[cfg(test)]
use std::boxed::Box;

#[cfg(test)]
fn add_two_linked_list(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // result链表(返回值链表)的头节点，用于记住result链表的「第一个节点」
    let mut head_node: Option<Box<ListNode>> = None;
    // 遍历时的当前节点，初始值是result链表的head(第一个节点)
    let mut curr: &mut Option<Box<ListNode>> = &mut head_node;
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
                    *curr = Some(Box::new(ListNode::new(sum_or_carry)));
                }
                break;
            }
        }
        /* 遍历思路 */
        // 1. current_node一开始为None
        // 2. 初始化current_node的值
        // 3. 将current_node指向current_node.next(因为next为None，所以刚好回到第一步)
        *curr = Some(Box::new(ListNode::new(sum_or_carry % 10)));
        sum_or_carry /= 10; // 此时的sum_or_carry看作进位值，传入下次迭代
        curr = &mut curr.as_mut().unwrap().next;
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
    for &(arr1, arr2, expected) in &TEST_CASES {
        let ln1 = arr_to_linked_list(arr1);
        let ln2 = arr_to_linked_list(arr2);
        let output_head = add_two_linked_list(ln1, ln2);
        assert_eq!(linked_list_to_vec(output_head), expected.to_vec());
    }
}
