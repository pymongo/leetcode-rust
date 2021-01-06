use super::{Rc, RefCell, TreeNode};

struct Solution;

impl Solution {
    fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs_backtracking(
            node: Option<Rc<RefCell<TreeNode>>>,
            curr_path_sum: &mut i32,
            sum: &mut i32,
        ) {
            if let Some(node) = node {
                let node = node.borrow();
                let old_curr_path_sum = *curr_path_sum;
                *curr_path_sum = *curr_path_sum * 10 + node.val;

                // found leaf node
                if node.left.is_none() && node.right.is_none() {
                    // dbg!(*curr_path_sum, *sum);
                    *sum += *curr_path_sum;
                } else {
                    // dfs search
                    if node.left.is_some() {
                        dfs_backtracking(node.left.clone(), curr_path_sum, sum);
                    }
                    if node.right.is_some() {
                        dfs_backtracking(node.right.clone(), curr_path_sum, sum);
                    }
                }

                // leave current recursive bfs_dfs_backtracking
                *curr_path_sum = old_curr_path_sum;
            }
        }
        let mut curr_path_sum = 0;
        let mut sum = 0;
        dfs_backtracking(root, &mut curr_path_sum, &mut sum);
        sum
    }

    fn sum_numbers_best(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs_backtracking_best(
            node: Option<Rc<RefCell<TreeNode>>>,
            curr_path_sum: &mut i32,
        ) -> i32 {
            let node = node.unwrap();
            let node = node.borrow();
            let old_curr_path_sum = *curr_path_sum;
            *curr_path_sum = *curr_path_sum * 10 + node.val;
            let mut sum = 0;
            if node.left.is_none() && node.right.is_none() {
                sum += *curr_path_sum;
            } else {
                if node.left.is_some() {
                    sum += dfs_backtracking_best(node.left.clone(), curr_path_sum);
                }
                if node.right.is_some() {
                    sum += dfs_backtracking_best(node.right.clone(), curr_path_sum);
                }
            }
            // leave current recursive bfs_dfs_backtracking
            *curr_path_sum = old_curr_path_sum;
            sum
        }

        if root.is_none() {
            return 0;
        }
        dfs_backtracking_best(root, &mut 0i32)
    }
}

#[test]
fn test_preorder_traversal() {
    // 4->9->5, 4->9->1, 4->0: 495+491+40=1026
    let root = super::str_to_tree_node("4(9(5)(1))(0)");
    assert_eq!(Solution::sum_numbers(root.clone()), 1026);
    assert_eq!(Solution::sum_numbers_best(root), 1026);
}
