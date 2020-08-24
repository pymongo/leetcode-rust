use super::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert(&mut root.as_mut()?.borrow_mut());
        root
        // if let Some(root_node) = root {
        //     Self::invert(&mut root_node.borrow_mut());
        //     Some(root_node)
        // } else {
        //     root
        // }
    }

    // 正确的做法: 即便是Rc<RefCell>也不需要Clone
    // 应当新建一个新的函数去递归，因为二叉树式In-Place原地修改，所以用解答的函数递归的返回值是多余且降低性能的
    fn invert(root: &mut TreeNode) {
        let left = &mut root.left;
        let right = &mut root.right;
        std::mem::swap(left, right);
        if let Some(node) = left {
            Self::invert(&mut node.borrow_mut());
        }
        if let Some(node) = right {
            Self::invert(&mut node.borrow_mut());
        }
    }

    fn unsafe_solution(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_ptr = root.as_mut()?.as_ptr();
        unsafe {
            let left = &mut (*root_ptr).left;
            let right = &mut (*root_ptr).right;
            std::mem::swap(left, right);
            Self::invert_tree(left.clone());
            Self::invert_tree(right.clone());
        }
        root
    }
}
