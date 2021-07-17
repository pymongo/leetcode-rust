/*! https://leetcode.com/problems/maximum-subarray/
53. 最大子序和
给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和
O(n)的算法：贪心: if cur_sum+nums[i] < nums[i]，则以舍弃前面的元素，从i开始从新算最大长度
O(n)的算法：动态规划(「滚动数组」): 如果当前元素的前一个元素大于0，则当前元素的值 += 前一个元素的值
O(logn)的算法?分治?: 求区间内最值用「线段树」，但是加上预处理之后，就跟贪心一样是O(n)，本题没有logn的解法
这个分治方法类似于「线段树求解 LCIS 问题」的 pushUp 操作
*/

fn maximum_subarray_dp(mut nums: Vec<i32>) -> i32 {
    let size = nums.len();
    let mut max_sum = nums[0];
    for i in 1..size {
        if nums[i - 1] > 0 {
            nums[i] += nums[i - 1];
        }
        max_sum = max_sum.max(nums[i]);
    }
    max_sum
}
