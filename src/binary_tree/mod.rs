mod invert_binary_tree;
mod preorder;

pub use std::cell::RefCell;
pub use std::rc::Rc;

pub type OptionalTreeNode = Option<Rc<RefCell<TreeNode>>>;

/// TODO add OptionalTreeNode to str function
pub fn str_to_optional_tree_node(s: &str) -> OptionalTreeNode {
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
    if let Some(peek) = stack.last() {
        Some(peek.clone())
    } else {
        None
    }
}

#[test]
fn test_str_to_optional_tree_node() {
    // Rust的Debug可以完整地递归打印出二叉树，比我用Python写的打印二叉树更准更好，约等于leetcode的Python/Java print二叉树的效果
    dbg!(str_to_optional_tree_node("1()(2(3))"));
    dbg!(str_to_optional_tree_node("3(9)(20(15)(7))"));
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: OptionalTreeNode,
    pub right: OptionalTreeNode,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
