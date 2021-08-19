use super::{Rc, RefCell, TreeNode};

fn search_val_in_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root;
    while let Some(node_outer) = cur {
        let node = node_outer.borrow();
        match node.val.cmp(&val) {
            std::cmp::Ordering::Less => cur = node.right.clone(),
            std::cmp::Ordering::Greater => cur = node.left.clone(),
            std::cmp::Ordering::Equal => {
                drop(node);
                return Some(node_outer);
            }
        }
    }
    None
}

/// https://leetcode.com/problems/range-sum-of-bst/
fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut res = 0;
    let mut stack = vec![root];
    while let Some(Some(peek)) = stack.pop() {
        let peek = peek.borrow();

        if low <= peek.val && peek.val <= high {
            res += peek.val;
        }
        stack.push(peek.right.clone());
        stack.push(peek.left.clone());
    }
    res
}
