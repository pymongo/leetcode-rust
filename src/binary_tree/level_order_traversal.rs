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
/// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
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
            ret.push(std::mem::take(&mut cur_level));
            // add level separator to queue end
            if !queue.is_empty() {
                queue.push_back(None);
            }
        }
    }

    // level-order-ii这题，将ret.reverse()即可，频繁insert(0)性能很差，reverse操作是In-Place的
    // ret.reverse()
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
            ret.push(std::mem::take(&mut cur_level));
            // add level separator to queue end
            if !queue.is_empty() {
                queue.push_back(None);
            }
        }
    }
    ret
}

/// https://leetcode.com/problems/cousins-in-binary-tree/
/// 如果二叉树的两个节点深度相同(处于同一层)，但**父节点不同**，则它们是一对堂兄弟节点
/// 需要知道每个节点的三个信息: 层数、值、父节点的值
fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    // (depth, parent)
    let mut node_x: Option<(u8, i32)> = None;
    let mut node_y: Option<(u8, i32)> = None;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((root, 0)); // (cur_node, cur_parent)
                                // add sentinel node to queue end
    queue.push_back((None, 0));
    let mut cur_depth = 0_u8;
    while let Some((cur_node, cur_parent)) = queue.pop_front() {
        if let Some(cur_node) = cur_node {
            let cur_node = cur_node.borrow();

            if cur_node.val == x {
                if let Some((node_y_depth, node_y_parent)) = node_y {
                    return cur_depth == node_y_depth && cur_parent != node_y_parent;
                } else {
                    node_x = Some((cur_depth, cur_parent));
                }
            } else if cur_node.val == y {
                if let Some((node_x_depth, node_x_parent)) = node_x {
                    return cur_depth == node_x_depth && cur_parent != node_x_parent;
                } else {
                    node_y = Some((cur_depth, cur_parent));
                }
            }

            if cur_node.left.is_some() {
                queue.push_back((cur_node.left.clone(), cur_node.val));
            }
            if cur_node.right.is_some() {
                queue.push_back((cur_node.right.clone(), cur_node.val));
            }
        } else {
            cur_depth += 1;
            // add level separator to queue end
            if !queue.is_empty() {
                queue.push_back((None, 0));
            }
        }
    }
    false
}

#[test]
fn test_is_cousins() {
    #[allow(non_upper_case_globals)]
    const null: i32 = TreeNode::NULL;
    const TEST_CASES: [(&[i32], i32, i32, bool); 3] = [
        (&[1, 2, 3, 4], 4, 3, false),
        (&[1, 2, 3, null, 4, null, 5], 5, 4, true),
        (&[1, 2, 3, null, 4], 2, 3, false),
    ];
    for (root, x, y, expected) in TEST_CASES {
        let root = super::deserialize_vec_to_binary_tree(root);
        println!("{}", "=".repeat(20));
        super::print_binary_tree(root.clone()).unwrap();
        println!("x={}, y={}, expected={}", x, y, expected);
        assert_eq!(is_cousins(root, x, y), expected);
    }
}
