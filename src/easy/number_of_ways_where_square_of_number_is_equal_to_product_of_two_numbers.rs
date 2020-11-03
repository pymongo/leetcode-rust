/*
周赛#205的第二题: https://leetcode-cn.com/contest/weekly-contest-205/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/
解释测试用例1: 4*4=2*8, 所以得到三元组(1,1,2)
三元组的要求
1. nums1的某个数的平方=nums2某两个数的乘积
2. nums2的某个数的平方=nums1某两个数的乘积

解题思路1: 排序后能不能批量数方案?two_sum?
解题思路2: 因数分解num*num，看看另一个数组是否存在这些因子
*/
use std::collections::HashMap;

struct Solution;

impl Solution {
    fn helper(nums1: &[i32], nums2: &[i32]) -> i32 {
        // 用HashMap加快在num2中找因子的过程, key: num2, value: count of num2 in nums2
        let mut counter: HashMap<i32, i32> = HashMap::with_capacity(nums2.len());
        for &num in nums2 {
            if let Some(cnt) = counter.get_mut(&num) {
                *cnt += 1
            } else {
                counter.insert(num, 1);
            }
        }

        let mut res = 0;
        for &num1 in nums1 {
            let target = num1 * num1;
            for factor1 in 1..num1 {
                if target % factor1 != 0 {
                    continue;
                }
                let factor2 = target / factor1;
                if let (Some(cnt1), Some(cnt2)) = (counter.get(&factor1), counter.get(&factor2)) {
                    res += cnt1 * cnt2;
                }
            }
            // factor1 == factor2的情况，应对类似([1,1], [1,1,1])这样有重复的输入用例
            if let Some(cnt) = counter.get(&num1) {
                res += cnt * (cnt - 1) / 2;
            }
        }
        res
    }

    fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::helper(&nums1, &nums2) + Self::helper(&nums2, &nums1)
    }
}

#[cfg(test)]
const TESTCASES: [(&[i32], &[i32], i32); 4] = [
    (&[1, 1], &[1, 1, 1], 9),
    (&[7, 4], &[5, 2, 8, 9], 1),
    (&[7, 7, 8, 3], &[1, 2, 9, 7], 2),
    (&[4, 7, 9, 11, 23], &[3, 5, 1024, 12, 18], 0),
];

#[test]
fn test_diagonal_sum() {
    for &(nums1, nums2, res) in &TESTCASES {
        assert_eq!(Solution::num_triplets(nums1.to_vec(), nums2.to_vec()), res);
    }
}
