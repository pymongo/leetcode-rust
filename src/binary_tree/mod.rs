mod invert_binary_tree;
mod is_bst;
mod leaf_similar_trees;
mod level_order_traversal;
mod max_depth_of_binary_tree;
mod preorder_traversal;
mod same_tree;
mod search_val_or_range_in_bst;
mod serde_binary_tree_to_leetcode_vec;
mod serde_binary_tree_to_parentheses_str;
mod sum_root_to_leaf_numbers;
pub use serde_binary_tree_to_leetcode_vec::{
    deserialize_vec_to_binary_tree, print_binary_tree, serialize_binary_tree_to_vec,
};
pub use serde_binary_tree_to_parentheses_str::parentheses_str_to_binary_tree;
use std::cell::RefCell;
use std::rc::Rc;

/// 正常的二叉树的节点也不可能有两个父亲，所以leetcode用Rc<RefCell>真是多余
/// 我做过那么多题也没见过二叉树节点的左右儿子是同一个节点
/// https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md#leetcode
/// Option<Rc<RefCell<Node>>> is overkill for tree links
/// Rust的Debug可以递归打印出二叉树，比我用Python写的打印二叉树更准更好，约等于leetcode的Python二叉树的__repr__()的效果
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: TreeLink,
    right: TreeLink,
}

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    pub const NULL: i32 = i32::MIN;
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}
