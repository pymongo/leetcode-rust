//! https://leetcode.com/problems/univalued-binary-tree/
use super::prelude::*;

fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(root) => {
            let root = root.borrow();

            if !is_unival_tree(root.left.clone()) {
                return false;
            }
            if !is_unival_tree(root.right.clone()) {
                return false;
            }

            if let Some(ref child) = root.left {
                if root.val != child.borrow().val {
                    return false;
                }
            }
            if let Some(ref child) = root.right {
                if root.val != child.borrow().val {
                    return false;
                }
            }

            true
        }
        None => true,
    }
}

#[test]
fn test_is_unival_tree() {
    for (tree, is_unival) in [
        (vec![1, 1, 1, 1, 1, null, 1], true),
        (vec![2, 2, 2, 5, 2], false),
    ] {
        assert_eq!(
            is_unival_tree(deserialize_vec_to_binary_tree(&tree)),
            is_unival
        );
    }
}
