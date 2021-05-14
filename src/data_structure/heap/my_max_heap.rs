/**
## 性能警告
本人手写的Heap性能不到标准库BinaryHeap的1/4，仅供学习和参考，生产环境请用标准库的BinaryHeap

## 为什么堆也叫优先队列
因为Java标准库没有heap,只有约等于heap的优先队列，所以堆也可以叫做优先队列

## 数组模拟二叉树
例如如下的二叉树

```text
      0
     / \
    /   \
   /     \
  1       2
 / \     / \
3   4   5   6
```

在 leetcode 中会通过层级遍历(level_order)编号然后序列成这样的数组: [0,1,2,3,4,5,6]

由于MaxHeap是一个完全二叉树，非常适合用层级遍历的数组去表达一个完全二叉树

用level_order遍历编号的数组表示堆，比用二叉树性能好

观察可发现，假设数组表示的二叉树的节点下标是n，那么

- 左子节点的下标是: 2n+1
- 右子节点的下标是: 2n+2

倒推可得节点n父节点的下标是(n-1)/2

除了堆，线段树也是底层用数组进行存储居多

## 用堆可以解决的一些问题

Dijkstra=BFS+heap+greedy

- [ ] Huffman coding(出现频率高的字符权重更高)(贪心算法的一个典型例子，采用优先队列构建最优的前缀编码树(prefixEncodeTree))
- [ ] Prim's algorithm for minimum spanning tree
- [x] top k largest
- [ ] 407. 接雨水 II
- [ ] 778. 水位上升的泳池中游泳
*/
pub struct MyMaxHeap<T: Ord>(Vec<T>);

impl<T: Ord> MyMaxHeap<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, val: T) {
        self.0.push(val);
        self.sift_up(self.0.len() - 1);
    }

    /// 由于数组末尾元素会是个较小值，所以swap(0, len-1)后sift_down(0)会把堆顶左右儿子换一个最大的到堆顶，然后再往下找个合适位置给这个较小值
    #[must_use]
    fn pop(&mut self) -> T {
        //debug_assert!(!self.0.is_empty());
        let last_index = self.0.len() - 1;
        self.0.swap(0, last_index);
        let max_val = self.0.pop().unwrap();
        self.sift_down(0);
        max_val
    }

    /// 作用: 让堆变得"有序"，平均O(n)时间复杂度
    /// 只能用同时比较三个节点的sift_down，sift_up只能用于堆整体已经有序然后插入一个新值
    fn heapify(&mut self) {
        // 从堆顶一直sift_down到self.len/2-1是最后一个非叶子节点，根据节点n父节点的下标是(n-1)/2的公式
        #[allow(clippy::range_minus_one)]
        for i in (0..=(self.0.len() / 2 - 1)).rev() {
            self.sift_down(i);
        }
    }

    #[must_use]
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut ret = (0..self.0.len())
            .into_iter()
            .map(|_| self.pop())
            .collect::<Vec<_>>();
        ret.reverse();
        ret
    }

    /** MaxHeap的sift_up
    ## sift_up的过程
    输入一个index，记为curr，如果curr>curr.parent则交换curr和curr.parent
    然后curr=curr.parent继续迭代直到 到堆顶 或 parent的值比current大

    ## sift_up应用
    用于heap.push()的时候，把新的值放到堆数组尾部，然后对数组最后一个元素进行sift_up
    对于一个已经「"有序"(heapify)」的堆来说，让新的值从底部往上不断冒泡两两交换，直到让新值换到「合适的位置」
    */
    #[inline]
    fn sift_up(&mut self, mut curr_index: usize) {
        while curr_index > 0 {
            // SAFETY: since curr_index>=1, parent_index must>=0
            let parent_index = (curr_index - 1) / 2;
            if self.0[parent_index].gt(&self.0[curr_index]) {
                break;
            }
            self.0.swap(curr_index, parent_index);
            curr_index = parent_index;
        }
    }

    /**
    ## sift_down的过程
    输入一个index，记为curr，找到max_index=max(curr,curr.left,curr.right)
    如果max_index!=curr，则进行上下交换，curr=max_index继续迭代
    只比较当前节点和左右儿子，如果发生挪到，则curr=变动的左/右儿子，迭代会不断往二叉树底部方向走

    ## sift_down的应用
    1. heapify
    2. heappop掉堆顶元素后，把堆数组最后一个元素(会是个较小值)换到堆顶，然后sift_down(0)
    */
    #[inline]
    fn sift_down(&mut self, mut curr_index: usize) {
        let len = self.0.len();
        while curr_index < len {
            let left_child_index = curr_index * 2 + 1;
            let right_child_index = left_child_index + 1;
            let mut max_index = curr_index;
            if left_child_index < len && self.0[left_child_index].gt(&self.0[max_index]) {
                max_index = left_child_index;
            }
            if right_child_index < len && self.0[right_child_index].gt(&self.0[max_index]) {
                max_index = right_child_index;
            }
            // 没有发生交换
            if max_index == curr_index {
                break;
            }
            self.0.swap(curr_index, max_index);
            curr_index = max_index;
        }
    }
}

impl<T: Ord> std::iter::FromIterator<T> for MyMaxHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        for each in iter {
            heap.push(each);
        }
        heap
    }
}

#[test]
fn test_my_heap() {
    use crate::code_snippets::sorting::random_numbers_test_case;
    let test_cases = random_numbers_test_case();
    //let test_cases = vec![3,7,4,8];
    let heap = test_cases.into_iter().collect::<MyMaxHeap<_>>();
    assert!(heap.into_sorted_vec().is_sorted());
}
