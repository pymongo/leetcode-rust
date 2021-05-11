use super::{Rc, RefCell, TreeLink, TreeNode};

/// TODO add tree_node to str function
pub fn parentheses_str_to_binary_tree(s: &str) -> TreeLink {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut val_len = 0;
    let mut is_left_subtree_empty = false;
    let s = s.as_bytes();
    for i in 0..s.len() {
        if s[i] != b'(' && s[i] != b')' {
            val_len += 1;
            continue;
        }
        if val_len > 0 {
            let node_val = String::from_utf8(s[i - val_len..i].to_owned())
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let node = Rc::new(RefCell::new(TreeNode::new(node_val)));
            if let Some(peek) = stack.last_mut() {
                let mut peek = peek.borrow_mut();
                if is_left_subtree_empty {
                    peek.right = Some(node.clone());
                    is_left_subtree_empty = false;
                } else if peek.left.is_none() {
                    peek.left = Some(node.clone());
                } else {
                    peek.right = Some(node.clone());
                }
            }
            stack.push(node.clone());
            val_len = 0;
        }
        if s[i] == b')' {
            if s[i - 1] == b'(' {
                is_left_subtree_empty = true;
            } else {
                stack.pop().unwrap();
            }
        }
    }
    stack.last().cloned()
}

#[test]
fn test_str_to_optional_tree_node() {
    let node = parentheses_str_to_binary_tree("1()(2(3))");
    super::print_binary_tree(node).unwrap();
}
