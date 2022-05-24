//! https://leetcode.com/problems/validate-binary-search-tree/

use super::prelude::*;

fn is_bst(node: Option<Rc<RefCell<TreeNode>>>, lower: Option<i32>, upper: Option<i32>) -> bool {
    match node {
        Some(node) => {
            let node = node.borrow();
            let node_val = node.val;
            lower.map_or(true, |lower| lower < node_val)
                && upper.map_or(true, |upper| node_val < upper)
                && is_bst(node.left.clone(), lower, Some(node_val))
                && is_bst(node.right.clone(), Some(node_val), upper)
        }
        None => true,
    }
}

static mut PRE_ORDER_PREV_VAL: Option<i32> = None;

/** https://leetcode.com/problems/validate-binary-search-tree/
先序遍历二叉树，记录上一个先序遍历的值与当前值比较，如果curr.val<=prev.val(非升序)，则不是bst
该解法用到了static全局变量，属于有状态的函数，调用结束后会static全局变量的值残留上次调用的结果
```python
class Solution:
    def is_bst(self, root: TreeNode) -> bool:
        self.prev_val = float('-inf')

        def is_not_bst(node: TreeNode):
            if node is None:
                return False
            if is_not_bst(node.left):
                return True
            if self.prev_val >= node.val:
                return True
            # 离开当前层递归时即将进入右子树递归时，更新上一层递归的节点的值(self.prev_val)
            self.prev_val = node.val
            if is_not_bst(node.right):
                return True
            return False
        return not is_not_bst(root)
```
*/
fn is_bst_dirty_solution(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(root) => unsafe {
            let root = root.borrow();
            if !is_bst_dirty_solution(root.left.clone()) {
                return false;
            }

            if PRE_ORDER_PREV_VAL.map_or(false, |prev_val| root.val <= prev_val) {
                return false;
            }
            // if let Some(prev_node_val) = PRE_ORDER_PREV_VAL {
            //     if root.val <= prev_node_val {
            //         return false;
            //     }
            // }

            PRE_ORDER_PREV_VAL = Some(root.val);
            if !is_bst_dirty_solution(root.right.clone()) {
                return false;
            }
            true
        },
        None => true,
    }
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // 由于该solution用到了static是有状态的，所以每次运行前需要重置static全局变量，否则static变量就是上次运行的残留结果
    unsafe {
        PRE_ORDER_PREV_VAL = None;
    }
    is_bst_dirty_solution(root)
}
