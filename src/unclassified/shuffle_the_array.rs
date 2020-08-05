struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n: usize = n as usize;
        let mut result: Vec<i32> = Vec::with_capacity(2 * n);
        for i in 0..n {
            result.push(nums[i]);
            result.push(nums[i+n]);
        }
        return result;
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], i32, &[i32]); 1] = [
    (&[2,5,1,3,4,7], 3, &[2,3,5,4,1,7]),
];

#[test]
fn test_shuffle() {
    for &(nums, n, expected) in TEST_CASES.iter() {
        // let nums: Vec<i32> = nums.to_vec();
        // let nums_vec: Vec<i32> = nums.to_vec();
        let nums_vec: Vec<i32> = Vec::from(nums);
        let output = Solution::shuffle(nums_vec, n);
        assert_eq!(&output[..], expected);
    }
}