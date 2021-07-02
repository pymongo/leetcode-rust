use super::{Rc, RefCell, TreeNode};

/* reproduce cannot borrow `stack` as mutable because it is also borrowed as immutable
// HashMap的Entry的出现为了解决所有权内部元素可变的限制，例如counter中想同时通过插入元素修改HashMap本身，还同时修改HashMap内部的某个值，可能会报错，Entry的出现就是为了解决此问题
fn preorder_traversal_mut_borrow_err(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut stack = vec![root];
    // immutable borrow occurs here
    while let Some(peek) = stack.last() {
        if let Some(peek) = peek {
            let peek = peek.borrow();
            res.push(peek.val);
            // mutable borrow occurs here
            stack.push(peek.right.clone());
            stack.push(peek.left.clone());
        }
    }
    res
}
*/

/**
递归版preorder遍历太简单了，就不写了
栈遍历的升级版是: 「莫里斯遍历+线索二叉树」
莫尼斯遍历不需要借助队列或栈的空间，与前序遍历不同的是莫里斯遍历每个节点只会访问一次
*/
fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    let mut stack = vec![root];
    while let Some(peek) = stack.pop() {
        if let Some(peek) = peek {
            let peek = peek.borrow();
            ret.push(peek.val);
            stack.push(peek.right.clone());
            stack.push(peek.left.clone());
        }
    }
    ret
}

fn preorder_traversal_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn helper(ret: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) -> Option<()> {
        let mut stack = vec![node];
        while let Some(peek) = stack.pop()? {
            let peek = peek.borrow();
            ret.push(peek.val);
            stack.push(peek.right.clone());
            stack.push(peek.left.clone());
        }
        Some(())
    }
    let mut ret = Vec::new();
    helper(&mut ret, root).unwrap_or_default();
    ret
}

#[test]
fn test_preorder_traversal() {
    let root =
        super::serde_binary_tree_to_parentheses_str::parentheses_str_to_binary_tree("1()(2(3))");
    assert_eq!(preorder_traversal(root), vec![1, 2, 3]);
}
