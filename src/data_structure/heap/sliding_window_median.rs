/*!
```tex
min_heap
\    /
 \  /
  \/
median
  /\
 /  \
/    \
max_heap

---

## max_heap.len()

- min_heap.len()     if nums.len() is even
- min_heap.len() + 1 if nums.len() is odd

## median

- min_heap.peek() if nums.len() is odd
- (min_heap.peek() + max_heap.peek()) / 2 if nums.len() is even

## Insert
if n

```
*/
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// median in a data stream or dynamic median in nums
/// 用 BST 也能保证根节点就是中位数(长度为奇数时)，但 BST 的增删麻烦
#[derive(Default)]
struct MinMaxHeapMedian {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
    deleted: HashSet<i32>,
}

impl MinMaxHeapMedian {
    /// 类似 AVL aka Balanced BinaryTree, 保证增删操作后左右两边最多相差一个节点数
    fn insert(&mut self, val: i32) {
        if self.min_heap.is_empty() {
            self.min_heap.push(Reverse(val));
            return;
        }
        if val <= self.min_peek() {
            self.min_heap.push(Reverse(val));
            if self.min_heap.len() >= self.max_heap.len() + 2 {
                // 「balance」pop a min_heap to max_heap
                self.max_heap.push(self.min_heap.pop().unwrap().0);
            }
        } else {
            self.max_heap.push(val);
            if self.max_heap.len() >= self.min_heap.len() + 2 {
                // balance: move node from max_heap -> min_heap
                self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
            }
        }
    }

    fn lazy_delete(&mut self, val: i32) {
        self.deleted.insert(val);
    }

    fn min_peek(&mut self) -> i32 {
        self.min_heap.pop().unwrap();
        eprintln!("TODO: WIP");
        -1
    }

    fn max_peek(&mut self) -> i32 {
        let max_peek;
        loop {
            let peek = self.max_heap.peek().unwrap();
            if !self.deleted.contains(peek) {
                max_peek = *peek;
                break;
            }
            assert!(self.deleted.remove(peek));
            self.min_heap.pop().unwrap();
        }
        max_peek
    }

    fn len(&self) -> usize {
        self.min_heap.len() + self.max_heap.len() - self.deleted.len()
    }

    fn median(&mut self) -> f64 {
        if self.len() % 2 == 0 {
            f64::from(self.min_peek() + self.max_peek()) / 2.0
        } else {
            f64::from(self.max_peek())
        }
    }
}

/// https://leetcode.com/problems/sliding-window-median/
fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let k = k as usize;
    let len = nums.len();
    let mut medians = Vec::with_capacity(len - k);
    let mut min_max_heap = MinMaxHeapMedian::default();
    for i in 0..len {
        if i < k {
            min_max_heap.insert(nums[i]);
            continue;
        }
        dbg!(&min_max_heap.min_heap);
        dbg!(&min_max_heap.max_heap);
        medians.push(min_max_heap.median());
        min_max_heap.lazy_delete(nums[i - k]);
        min_max_heap.insert(nums[i]);
    }
    medians
}

#[test]
#[should_panic]
fn test_median_sliding_window() {
    const TEST_CASES: [(&[i32], i32, &[f64]); 1] = [(
        &[1, 3, -1, -3, 5, 3, 6, 7],
        3,
        &[1.0, -1.0, -1.0, 3.0, 5.0, 6.0],
    )];
    for (nums, k, medians) in TEST_CASES {
        assert_eq!(median_sliding_window(nums.to_vec(), k), medians);
    }
}

/**
```text
min_heap
\    /
 \  /
  \/
median
  /\
 /  \
/    \
max_heap
```
或者用双指针
*/
/// https://leetcode.com/problems/find-median-from-data-stream/
//struct MedianFinder<T: Ord + From<f64>> {
struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() {
            self.max_heap.push(num);
            return;
        }
        if num <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(num);
            if self.max_heap.len() == self.min_heap.len() + 2 {
                // balance
                self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
            }
        } else {
            self.min_heap.push(Reverse(num));
            if self.min_heap.len() == self.max_heap.len() + 2 {
                // balance
                self.max_heap.push(self.min_heap.pop().unwrap().0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if (self.min_heap.len() + self.max_heap.len()) % 2 == 0 {
            f64::from(self.min_heap.peek().unwrap().0 + self.max_heap.peek().unwrap()) / 2.0
        } else if self.max_heap.len() > self.min_heap.len() {
            f64::from(*self.max_heap.peek().unwrap())
        } else {
            f64::from(self.min_heap.peek().unwrap().0)
        }
    }
}

#[test]
fn test_find_median_data_stream() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);
    dbg!(&finder.max_heap);
    dbg!(&finder.min_heap);
    dbg!(finder.find_median());
    finder.add_num(3);
    dbg!(&finder.max_heap);
    dbg!(&finder.min_heap);
}
