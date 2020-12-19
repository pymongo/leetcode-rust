/*!
数组中的每个元素代表你在该位置可以跳跃的最大长度

你的目标是使用最少的跳跃次数到达数组的最后一个位置

此题与跳跃游戏1不同的是，这题求的是最小步数，而跳跃游戏1求的是可行性
*/
struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    fn jump(nums: Vec<i32>) -> i32 {
        let nums = nums
            .into_iter()
            .map(|num| num as usize)
            .collect::<Vec<usize>>();
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let mut dp = vec![std::i32::MAX; n];
        dp[0] = 0;
        for i in 0..n {
            let right_most = i + nums[i];
            if right_most >= n - 1 {
                return dp[i] + 1;
            }
            for j in (i + 1)..=right_most {
                dp[j] = dp[j].min(dp[i] + 1);
            }
        }
        dp[n - 1]
    }

    #[allow(clippy::needless_range_loop)]
    fn greedy(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let nums = nums
            .into_iter()
            .map(|num| num as usize)
            .collect::<Vec<usize>>();
        let mut step = 0i32;
        let mut right_most = 0usize;
        // 当前这一步最远能跳到哪
        let mut curr_step_right_most = 0usize;
        // 注意遍历到终点的前一格
        for i in 0..n - 1 {
            right_most = right_most.max(nums[i] + i);
            // 如果已经走到当前层能走的最远距离，则更新下一层能走的最远距离，并让步数+1
            if i == curr_step_right_most {
                curr_step_right_most = right_most;
                step += 1;
            }
        }
        step
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], i32); 1] = [(&[2, 3, 1, 1, 4], 2)];

#[test]
fn test() {
    for &(nums, min_step) in &TEST_CASES {
        assert_eq!(Solution::jump(nums.to_vec()), min_step);
        assert_eq!(Solution::greedy(nums.to_vec()), min_step);
    }
}
