/*! https://leetcode.com/problems/kth-largest-element-in-a-stream/
类似于福布斯富豪榜，新富豪和富豪榜中末位比，如果新富豪更有钱就能挤上去
代码实现上维护一个大小为k的小根堆
如果富豪人数小于k则持续入堆
如果富豪人数等于k则
    如果新富豪比末位富豪更有钱，则踢掉最穷的富豪，新富豪入堆，重新heapify
    否则小根堆维持不变
*/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    /// nums.len() > k
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut min_heap = BinaryHeap::with_capacity(k);
        for num in nums {
            if min_heap.len() < k {
                min_heap.push(Reverse(num));
                continue;
            }
            if num > min_heap.peek().unwrap().0 {
                min_heap.pop().unwrap();
                min_heap.push(Reverse(num));
            }
        }
        Self { min_heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.min_heap.len() < self.k {
            self.min_heap.push(Reverse(val));
        } else if val > self.min_heap.peek().unwrap().0 {
            self.min_heap.pop().unwrap();
            self.min_heap.push(Reverse(val));
        }
        self.min_heap.peek().unwrap().0
    }
}

#[test]
fn test_kth_in_a_stream() {
    let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(kth.add(3), 4);
    assert_eq!(kth.add(5), 5);
    assert_eq!(kth.add(10), 5);
    assert_eq!(kth.add(9), 8);
    assert_eq!(kth.add(4), 8);
}
