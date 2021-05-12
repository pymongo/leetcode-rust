//! https://leetcode-cn.com/problems/leaf-similar-trees/
//! 事实上这题 无论用前中后序遍历二叉树得到叶子节点
use super::TreeLink;

fn reverse_post_order(node: TreeLink) -> Vec<i32> {
    let mut ret = vec![];
    let mut stack = vec![node];
    while let Some(node) = stack.pop() {
        if let Some(node) = node {
            let node = node.borrow();
            // `根右左`「逆」后序遍历 (后序遍历=`左右根`)
            if node.left.is_none() && node.right.is_none() {
                ret.push(node.val);
            }
            stack.push(node.left.clone());
            stack.push(node.right.clone());
        }
    }
    ret
}

fn leaf_similar_trees(root1: TreeLink, root2: TreeLink) -> bool {
    let leafs1 = reverse_post_order(root1);
    let leafs2 = reverse_post_order(root2);
    leafs1 == leafs2
}

#[test]
fn test_leaf_similar_trees() {
    #[allow(non_upper_case_globals)]
    const null: i32 = super::TreeNode::NULL;
    const TEST_CASES: [(&[i32], &[i32], bool); 3] = [
        (
            &[3, 5, 1, 6, 2, 9, 8, null, null, 7, 4],
            &[3, 5, 1, 6, 2, 9, 8, null, null, 7, 4],
            true,
        ),
        (&[1], &[1], true),
        (&[1], &[2], false),
    ];
    for (root1, root2, is_leaf_similar) in TEST_CASES {
        let root1 = super::deserialize_vec_to_binary_tree(root1);
        let root2 = super::deserialize_vec_to_binary_tree(root2);
        assert_eq!(leaf_similar_trees(root1, root2), is_leaf_similar);
    }
}
