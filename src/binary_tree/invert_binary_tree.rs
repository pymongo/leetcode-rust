use super::prelude::*;

/**
```python
def invert_binary_tree(root: TreeNode) -> TreeNode:
    if root is None:
        return root
    root.left, root.right = root.right, root.left
    invert_binary_tree(root.left)
    invert_binary_tree(root.right)
    return root
```
*/
fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    /// 应当新建一个新的函数去递归，因为二叉树式In-Place原地修改，所以用解答的函数递归的返回值是多余且降低性能的
    fn invert(root: &mut TreeNode) {
        let left = &mut root.left;
        let right = &mut root.right;
        std::mem::swap(left, right);
        if let Some(node) = left {
            invert(&mut node.borrow_mut());
        }
        if let Some(node) = right {
            invert(&mut node.borrow_mut());
        }
    }
    invert(&mut root.as_mut()?.borrow_mut());
    root
}

fn invert_tree_unsafe_solution(
    mut root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root_ptr = root.as_mut()?.as_ptr();
    unsafe {
        let left = &mut (*root_ptr).left;
        let right = &mut (*root_ptr).right;
        std::mem::swap(left, right);
        invert_tree_unsafe_solution(left.clone());
        invert_tree_unsafe_solution(right.clone());
    }
    root
}
