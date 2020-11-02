/**
数组nums按 \[x1,x2,...,xn,y1,y2,...,yn] 的格式排列

请你将数组按 [x1,y1,x2,y2,...,xn,yn] 格式重新排列
*/
struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n: usize = n as usize;
        let mut result: Vec<i32> = Vec::with_capacity(2 * n);
        for i in 0..n {
            result.push(nums[i]);
            result.push(nums[i + n]);
        }
        return result;
    }
}

#[cfg(test)]
const TESTCASES: [(&[i32], i32, &[i32]); 1] = [(&[2, 5, 1, 3, 4, 7], 3, &[2, 3, 5, 4, 1, 7])];

#[test]
fn test_shuffle() {
    for &(nums, n, expected) in TESTCASES.iter() {
        let output = Solution::shuffle(nums.to_vec(), n);
        assert_eq!(&output[..], expected);
    }
}
