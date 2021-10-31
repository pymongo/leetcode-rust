//! WIP
use std::ops::ControlFlow;

struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

/**
```text
  3
 / \
9  20
   / \
  15  7
```
*/
fn tree_example() -> TreeNode<i32> {
    TreeNode {
        value: 3,
        left: TreeNode::new(9),
        right: Some(Box::new(TreeNode {
            value: 20,
            left: TreeNode::new(15),
            right: TreeNode::new(7),
        })),
    }
}

impl<T> TreeNode<T> {
    #[allow(clippy::unnecessary_wraps)]
    fn new(value: T) -> Option<Box<Self>> {
        Some(Box::new(Self {
            value,
            left: None,
            right: None,
        }))
    }
}

/// Working In Progress
fn traverse_inorder<T>(root: Option<Box<TreeNode<T>>>) -> ControlFlow<Vec<T>, Vec<T>> {
    match root {
        Some(node) => {
            let mut return_val = vec![];
            return_val.extend(traverse_inorder(node.left)?);
            return_val.extend(traverse_inorder(node.right)?);
            ControlFlow::Break(return_val)
        }
        None => ControlFlow::Continue(Vec::new()),
    }
}

#[test]
fn test_traverse_inorder() {
    let tree = tree_example();
    let a = traverse_inorder(Some(Box::new(tree)))
        .break_value()
        .unwrap();
    dbg!(a);
}
