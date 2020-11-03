struct Solution;

// return [num for num, _ in collections.Counter(nums).most_common(k)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut counter = std::collections::HashMap::<i32, i32>::with_capacity(n);
        for &num in &nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        // 小根堆: (-出现次数, 数字)，所以堆顶会是出现次数最低的数字，随时可以被别人挤掉
        let mut heap = std::collections::BinaryHeap::<(i32, i32)>::with_capacity(k);
        for (&num, &cnt) in &counter {
            if heap.len() == k {
                if -cnt < heap.peek().unwrap().0 {
                    heap.pop();
                    heap.push((-cnt, num));
                }
            } else {
                heap.push((-cnt, num));
            }
        }
        heap.iter().rev().map(|(_, num)| *num).collect()
    }
}
