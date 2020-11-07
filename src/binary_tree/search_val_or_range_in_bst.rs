use super::{Rc, RefCell, TreeNode};

fn search_val_in_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
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
