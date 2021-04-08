/*! https://leetcode.com/problems/trapping-rain-water/
trapping_rain_water接雨水: 当前柱子能接水的最大高度等于两侧柱子高度的最小值-当前柱子的高度
所以第一个柱子和最后一个柱子的两侧只有一个柱子，必定不能接水
我们可以用动态规划的思想去优化寻找左侧最高柱子和右侧最高柱子的过程
*/

fn trap(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = nums.len();
    if n == 0 || n == 1 {
        return 0;
    }
    let mut left_max = Vec::with_capacity(n);
    left_max.push(nums[0]);
    for i in 1..n {
        left_max.push(nums[i].max(left_max[i - 1]));
    }
    let mut right_max = vec![0; n];
    right_max[n - 1] = nums[n - 1];
    for i in (0..=n - 2).rev() {
        right_max[i] = nums[i].max(right_max[i + 1]);
    }
    // println!("{:?}", &left_max);
    // println!("{:?}", &right_max);
    for i in 1..n - 1 {
        res += left_max[i].min(right_max[i]) - nums[i];
    }
    res
}

#[allow(clippy::needless_range_loop)]
fn brute_force(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    for i in 1..n - 1 {
        // 寻找左侧和右侧最大值的代码可以用动态规划区优化
        let (mut left_max, mut right_max) = (0, 0);
        for j in (0..=i).rev() {
            left_max = left_max.max(nums[j]);
        }
        for j in i..n {
            right_max = right_max.max(nums[j]);
        }
        res += left_max.min(right_max) - nums[i];
    }
    res
}

#[cfg(test)]
const TEST_CASES: [(&[i32], i32); 1] = [(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6)];

#[test]
fn test() {
    for &(nums, res) in &TEST_CASES {
        assert_eq!(trap(nums.to_vec()), res);
        assert_eq!(brute_force(nums.to_vec()), res);
    }
}
