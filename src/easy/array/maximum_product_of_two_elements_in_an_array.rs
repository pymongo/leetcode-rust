struct Solution;

impl Solution {
    fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by_key(|&num| std::cmp::Reverse(num));
        (nums[0] - 1) * (nums[1] - 1)
    }
}

#[test]
fn test_max_product() {
    assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
}
