use std::collections::VecDeque;

struct MonotonicQueue(VecDeque<i32>);

impl MonotonicQueue {
    fn new(cap: usize) -> Self {
        Self(VecDeque::with_capacity(cap))
    }

    fn push(&mut self, num: i32) {
        while let Some(&back) = self.0.back() {
            // 类似单调栈，新来num的会从左到右踢掉老的较小的数
            if back < num {
                self.0.pop_back().unwrap();
            } else {
                break;
            }
        }
        self.0.push_back(num);
    }

    fn pop(&mut self, num: i32) {
        if let Some(&front) = self.0.front() {
            // 之所以要看一下最老的一个元素值是否等于滑动窗口出队的值，是为了避免push的是否该值已被「踢掉」，如果push的时候被踢掉了，pop的时候就啥也不用管
            if front == num {
                self.0.pop_front().unwrap();
            }
        }
    }
}

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // 超时: nums.windows(k as usize).map(|window| *window.iter().max().unwrap()).collect()
    let n = nums.len();
    let k = k as usize;
    let mut monotonic_queue = MonotonicQueue::new(n);
    let mut i = 0_usize;
    let mut res = Vec::with_capacity(n - k + 1);
    while i < k - 1 {
        monotonic_queue.push(nums[i]);
        i += 1;
    }
    while i < n {
        monotonic_queue.push(nums[i]);
        res.push(*monotonic_queue.0.front().unwrap());
        monotonic_queue.pop(nums[i + 1 - k]);
        i += 1;
    }
    res
}

#[test]
fn test_max_sliding_window() {
    const TEST_CASES: [(&[i32], i32, &[i32]); 1] =
        [(&[1, 3, -1, -3, 5, 3, 6, 7], 3, &[3, 3, 5, 5, 6, 7])];
    for (nums, k, expected) in TEST_CASES {
        assert_eq!(max_sliding_window(nums.to_vec(), k), expected.to_vec());
    }
}
