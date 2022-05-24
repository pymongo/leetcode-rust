use super::prelude::*;

fn merge_two_binary_trees(root1: TreeLink, root2: TreeLink) -> TreeLink {
    if root1.is_none() {
        return root2;
    }
    if root2.is_none() {
        return root1;
    }

    let node1 = root1.clone()?;
    let mut node1 = node1.borrow_mut();
    let node2 = root2?;
    let node2 = node2.borrow();

    node1.val += node2.val;
    node1.left = merge_two_binary_trees(node1.left.clone(), node2.left.clone());
    node1.right = merge_two_binary_trees(node1.right.clone(), node2.right.clone());

    root1
}

#[test]
fn test_merge_two_binary_trees() {
    const TEST_CASES: [(&[i32], &[i32], &[i32]); 1] = [(
        &[1, 3, 2, 5],
        &[2, 1, 3, null, 4, null, 7],
        &[3, 4, 5, 5, 4, null, 7],
    )];
    for (root1, root2, output) in TEST_CASES {
        let root1 = deserialize_vec_to_binary_tree(root1);
        let root2 = deserialize_vec_to_binary_tree(root2);
        let output_binary_tree = deserialize_vec_to_binary_tree(output);
        println!("root1 = ");
        print_binary_tree(root1.clone()).unwrap();
        println!("root2 = ");
        print_binary_tree(root2.clone()).unwrap();
        println!("output = ");
        print_binary_tree(output_binary_tree).unwrap();
        assert_eq!(
            serialize_binary_tree_to_vec(merge_two_binary_trees(root1, root2)),
            output
        );
    }
}
