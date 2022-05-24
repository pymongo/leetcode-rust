use super::prelude::*;

fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    helper(root, false)
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
    match node {
        Some(node) => {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                if is_left {
                    return node.val;
                }
                return 0;
            }
            helper(node.left.clone(), true) + helper(node.right.clone(), false)
        }
        None => 0,
    }
}

#[test]
fn test_sum_of_left_leaves() {
    const TEST_CASES: [(&[i32], i32); 1] = [(&[3, 9, 20, null, null, 15, 7], 24)];
    for (input, output) in TEST_CASES {
        let input = deserialize_vec_to_binary_tree(input);
        assert_eq!(sum_of_left_leaves(input), output);
    }
}
