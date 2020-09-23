/*!
数组中的每个元素代表你在该位置可以跳跃的最大长度

你的目标是使用最少的跳跃次数到达数组的最后一个位置
*/
struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![std::i32::MAX; n];
        dp[0] = 0;
        for i in 0..n {
            for j in (i + 1)..=(i + nums[i] as usize).min(n-1) {
                dp[j] = dp[j].min(dp[i] + 1);
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], i32); 1] = [
    (&[2, 3, 1, 1, 4], 2)
];

#[test]
fn test() {
    for &(nums, min_step) in &TEST_CASES {
        assert_eq!(Solution::jump(nums.to_vec()), min_step);
    }
}