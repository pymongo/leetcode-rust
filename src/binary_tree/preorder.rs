use super::{TreeNode, Rc, RefCell};

struct Solution;

impl Solution {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = vec![root];
        while let Some(peek) = stack.pop() {
            if let Some(peek) = peek {
                let peek = peek.borrow();
                res.push(peek.val);
                stack.push(peek.right.clone());
                stack.push(peek.left.clone());
            }
        }
        res
    }
}

#[test]
fn test_preorder_traversal() {
    let root = super::str_to_optional_tree_node("1()(2(3))");
    assert_eq!(Solution::preorder_traversal(root), vec![1, 2, 3]);
}
