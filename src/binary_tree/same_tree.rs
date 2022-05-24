use super::prelude::*;

/// https://leetcode.com/problems/same-tree/
fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // p == q
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let (p, q) = (p.borrow(), q.borrow());
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
        (_, _) => false,
    }
}
