mod invert_binary_tree;
mod level_order_traversal;
mod preorder_traversal;
mod search_val_or_range_in_bst;
mod sum_root_to_leaf_numbers;
use std::cell::RefCell;
use std::rc::Rc;
mod is_bst;
mod same_tree;
//mod increasing_order_search_tree;

/// TODO add tree_node to str function
/// due to orphan rule, can't impl From<str> to TreeNode directly
fn str_to_tree_node(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut val_len = 0;
    let mut is_left_subtree_empty = false;
    let s = s.as_bytes();
    for i in 0..s.len() {
        if s[i] != b'(' && s[i] != b')' {
            val_len += 1;
            continue;
        }
        if val_len > 0 {
            let node_val = String::from_utf8(s[i - val_len..i].to_owned())
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let node = Rc::new(RefCell::new(TreeNode::new(node_val)));
            if let Some(peek) = stack.last_mut() {
                let mut peek = peek.borrow_mut();
                if is_left_subtree_empty {
                    peek.right = Some(node.clone());
                    is_left_subtree_empty = false;
                } else if peek.left.is_none() {
                    peek.left = Some(node.clone());
                } else {
                    peek.right = Some(node.clone());
                }
            }
            stack.push(node.clone());
            val_len = 0;
        }
        if s[i] == b')' {
            if s[i - 1] == b'(' {
                is_left_subtree_empty = true;
            } else {
                stack.pop().unwrap();
            }
        }
    }
    stack.last().cloned()
}

#[test]
fn test_str_to_optional_tree_node() {
    // Rust的Debug可以完整地递归打印出二叉树，比我用Python写的打印二叉树更准更好，约等于leetcode的Python/Java print二叉树的效果
    dbg!(str_to_tree_node("1()(2(3))"));
    dbg!(str_to_tree_node("3(9)(20(15)(7))"));
}

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
