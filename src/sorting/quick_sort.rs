//! https://leetcode.com/problems/sort-an-array/
//! TAG: divide&conquer

#[cfg(test)]
const TEST_CASE: [(&[i32], &[i32]); 1] = [(&[5, 2, 3, 1], &[1, 2, 3, 5])];

#[test]
fn test_quick_sort() {
    for &(input, expected) in TEST_CASE.iter() {
        let input: Vec<i32> = input.iter().cloned().collect();
        let output = quick_sort(input);
        let expected: Vec<i32> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }
}

#[cfg(test)]
fn quick_sort(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len < 2 {
        return nums;
    }
    let pivot = nums[0];
    let mut left_part = Vec::new();
    let mut right_part = Vec::new();
    for i in 1..len {
        if nums[i] >= pivot {
            right_part.push(nums[i])
        } else {
            left_part.push(nums[i])
        }
    }
    let mut result = Vec::with_capacity(len);
    result.append(&mut quick_sort(left_part));
    result.push(pivot);
    result.append(&mut quick_sort(right_part));
    return result;
}
