use super::prelude::*;

/// https://leetcode.cn/problems/evaluate-boolean-binary-tree/
fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root = root.unwrap();
    let root = root.borrow();
    if root.left.is_none() && root.right.is_none() {
        return root.val == 1;
    }
    let left = evaluate_tree(root.left.clone());
    let right = evaluate_tree(root.right.clone());
    if root.val == 2 {
        left || right
    } else {
        left && right
    }
}
