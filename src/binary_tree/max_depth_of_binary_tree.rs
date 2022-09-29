use super::TreeLink;

/// https://leetcode.com/problems/maximum-depth-of-binary-tree/
#[allow(clippy::option_if_let_else)]
fn max_depth(root: &TreeLink) -> usize {
    match root {
        Some(node) => {
            let node = node.borrow();
            max_depth(&node.left).max(max_depth(&node.right)) + 1
        }
        None => 0,
    }
}
