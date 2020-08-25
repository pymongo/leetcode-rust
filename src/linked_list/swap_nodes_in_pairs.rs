use super::ListNode;
use std::boxed::Box;

struct Solution;

/* Python版方便理解记忆
def swap_pairs(self, head):
    if head and head.next:
        right = head.next
        head.next = self.swap_pairs(right.next)
        right.next = head
        return right
    # 5->None这种情况就不换
    return head
*/
impl Solution {
    pub fn swap_pairs_best(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // TODO 用map unwrap Option<T>，学到了，如果是head是None则不会走map的函数
        head.map(|mut left| match left.next.take() {
            None => left,
            Some(mut right) => {
                left.next = Self::swap_pairs(right.next.take());
                right.next = Some(left);
                right
            }
        })
    }

    fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::recursive(head)
    }

    fn recursive(mut node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if node.is_some() && node.as_ref()?.next.is_some() {
            // right是一对节点中靠右节点(奇数下标的节点)，node是靠左的
            let mut right = node.as_mut()?.next.take();
            // 下一对的第一个
            let next_pair = right.as_mut()?.next.take();
            node.as_mut()?.next = Solution::recursive(next_pair);

            right.as_mut()?.next = node;
            right
        } else {
            node
        }
    }
}
