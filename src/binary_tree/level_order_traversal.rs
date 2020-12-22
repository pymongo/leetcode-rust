use super::{Rc, RefCell, TreeNode};

/// https://leetcode.com/problems/count-complete-tree-nodes/
/// return 0 if not root else 1+self.countNodes(root.left)+self.countNodes(root.right)
fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut nodes_count = 0;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    while let Some(option_node) = queue.pop_front() {
        if let Some(rc_node) = option_node {
            nodes_count += 1;
            let node = rc_node.borrow();
            if let Some(left) = node.left.clone() {
                queue.push_back(Some(left));
            }
            if let Some(right) = node.right.clone() {
                queue.push_back(Some(right));
            }
        }
    }
    nodes_count
}

/// https://leetcode.com/problems/binary-tree-level-order-traversal/
fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return Vec::new();
    }
    let mut ret = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    // add sentinel node to queue end
    queue.push_back(None);

    let mut cur_level = Vec::new();
    while let Some(option_node) = queue.pop_front() {
        if let Some(rc_node) = option_node {
            let node = rc_node.borrow();
            cur_level.push(node.val);
            if let Some(left) = node.left.clone() {
                queue.push_back(Some(left));
            }
            if let Some(right) = node.right.clone() {
                queue.push_back(Some(right));
            }
        } else {
            ret.push(cur_level.clone());
            // cur_level is end
            cur_level.clear();
            // add level separator to queue end
            if !queue.is_empty() {
                queue.push_back(None);
            }
        }
    }

    ret
}

/// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return Vec::new();
    }
    let mut left_to_right = true;
    let mut ret = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    queue.push_back(None);

    let mut cur_level = Vec::new();
    while let Some(option_node) = queue.pop_front() {
        if let Some(rc_node) = option_node {
            let node = rc_node.borrow();
            cur_level.push(node.val);
            if let Some(left) = node.left.clone() {
                queue.push_back(Some(left));
            }
            if let Some(right) = node.right.clone() {
                queue.push_back(Some(right));
            }
        } else {
            if !left_to_right {
                cur_level.reverse();
            }
            left_to_right = !left_to_right;
            ret.push(cur_level.clone());
            // cur_level is end
            cur_level.clear();
            // add level separator to queue end
            if !queue.is_empty() {
                queue.push_back(None);
            }
        }
    }

    ret
}
