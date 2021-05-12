/*! serialize or deserialize binary tree to leetcode vec format
leetcode binary tree vec format example: [3,9,20,null,null,15,7] or [3,9,20,#,#,15,7]
*/
use super::{Rc, RefCell, TreeLink, TreeNode};

pub fn serialize_binary_tree_to_vec(root: TreeLink) -> Vec<i32> {
    let mut ret = vec![];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        if let Some(node) = node {
            let node = node.borrow();
            ret.push(node.val);
            queue.push_back(node.left.clone());
            queue.push_back(node.right.clone());
        } else {
            ret.push(TreeNode::NULL);
        }
    }
    // 去掉末尾的None,使序列化结果是个严格的「完全二叉树」
    while let Some(last) = ret.last() {
        if *last == TreeNode::NULL {
            ret.pop().unwrap();
        } else {
            break;
        }
    }
    ret
}

/**
Reference: https://github.com/msbanik/drawtree/blob/c145ba9ca7687576323ea01a9a382bcfb76dac0d/drawtree/drawtree.py#L297
另一种BST反序列化的方法是利用类似以下解法的子节点的cursor，但cursor每次迭代都+2,容易越界
因为这种层级遍历反序列化的二叉树是「完全二叉树」而不是铺满的「完美二叉树」，通过父节点下标偏移算出的子节点容易越界
除了层级遍历二叉树的数组，还能用`1(2)(3)`这种括号字符串作为二叉树序列化格式

## 完满(full)二叉树
只要有子节点的节点必有两个子节点
*/
pub fn deserialize_vec_to_binary_tree(level_order: &[i32]) -> TreeLink {
    let nodes = level_order
        .iter()
        .map(|&num| {
            if num == TreeNode::NULL {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(num))))
            }
        })
        .collect::<Vec<_>>();
    let root = nodes.get(0)?.clone();

    let mut child_node_ptr = 1;
    for node in nodes.iter().flatten() {
        if let Some(child_node) = nodes.get(child_node_ptr) {
            node.borrow_mut().left = child_node.clone();
            child_node_ptr += 1;
        } else {
            break;
        }
        if let Some(child_node) = nodes.get(child_node_ptr) {
            node.borrow_mut().right = child_node.clone();
            child_node_ptr += 1;
        } else {
            break;
        }
        /*
        if child_node_ptr < len {
            node.borrow_mut().right = nodes[child_node_ptr].clone();
            child_node_ptr += 1;
        }
        */
    }

    root
}

/// https://github.com/msbanik/drawtree
/// https://stackoverflow.com/a/37958190/9970487
/// pip install drawtree
pub fn print_binary_tree(root: TreeLink) -> Result<(), Box<dyn std::error::Error>> {
    let node_vec = serialize_binary_tree_to_vec(root);
    let mut node_vec_str = String::new();
    for num in node_vec {
        if num == TreeNode::NULL {
            node_vec_str.push('#');
        } else {
            node_vec_str.push_str(num.to_string().as_str());
        }
        node_vec_str.push(',');
    }
    node_vec_str.pop().unwrap();
    let python3_drawtree_output = std::process::Command::new("python3")
        .arg("-c")
        .arg(format!(
            "from drawtree.drawtree import deserialize, drawtree; drawtree(deserialize('{}'))",
            node_vec_str
        ))
        .output()?;
    debug_assert!(python3_drawtree_output.status.success());
    println!("{}", unsafe {
        String::from_utf8_unchecked(python3_drawtree_output.stdout)
    });
    Ok(())
}

#[test]
fn test_serde_binary_tree_to_leetcode_vec() {
    let null = TreeNode::NULL;
    let level_order_1 = vec![1, 2, 3, null, null, 4, 5];
    let node = deserialize_vec_to_binary_tree(&level_order_1);
    print_binary_tree(node.clone()).unwrap();
    let level_order_2 = serialize_binary_tree_to_vec(node);
    assert_eq!(level_order_1, level_order_2);
}
