use std::collections::HashSet;

struct Solution;

impl Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        }
        let n = nums.len();
        let k = if k > n as i32 { n } else { k as usize };
        let mut set: HashSet<i32> = HashSet::with_capacity(k);
        for i in 0..k {
            if set.contains(&nums[i]) {
                return true;
            }
            set.insert(nums[i]);
        }
        for i in k..n {
            if set.contains(&nums[i]) {
                return true;
            }
            set.remove(&nums[i - k]);
            set.insert(nums[i]);
        }
        false
    }
}
