/// https://leetcode.com/problems/single-element-in-a-sorted-array
#[allow(clippy::collapsible_else_if)]
fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if mid % 2 == 0 {
            if nums[mid] == nums[mid + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        } else {
            if nums[mid] == nums[mid - 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
    }
    nums[left]
}

#[test]
fn test_single_non_duplicate() {
    assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
}
