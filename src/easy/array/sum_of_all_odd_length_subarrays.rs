/*!
请你计算所有可能的奇数长度子数组的和。
输入：arr = [1,4,2,5,3]
输出：58
解释：所有奇数长度子数组和它们的和为：
\[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
我们将所有值求和得到 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
*/

struct Solution;

impl Solution {
    fn sum_odd_length_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total_sum = 0;
        let mut window_len = 1;
        while window_len <= n {
            let (mut left, mut right) = (0, window_len);
            let mut window_sum = (0..window_len).map(|i| nums[i]).sum::<i32>();
            total_sum += window_sum;
            while right < n {
                window_sum += nums[right] - nums[left];
                total_sum += window_sum;
                left += 1;
                right += 1;
            }
            window_len += 2;
        }
        total_sum
    }

    fn solution_use_slice_windows_api(nums: Vec<i32>) -> i32 {
        (1..=nums.len())
            .step_by(2)
            .into_iter()
            .map(|window_len| {
                nums.windows(window_len)
                    .into_iter()
                    .map(|window| window.iter().sum::<i32>())
                    .sum::<i32>()
            })
            .sum()
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], i32); 1] = [(&[1, 4, 2, 5, 3], 58)];

#[test]
fn test() {
    for &(nums, sum) in &TEST_CASES {
        assert_eq!(Solution::sum_odd_length_subarrays(nums.to_vec()), sum);
        assert_eq!(Solution::solution_use_slice_windows_api(nums.to_vec()), sum);
    }
}
