mod median_of_two_sorted_arrays;
mod merge_two_sorted_arrays;

#[cfg(test)]
const TEST_CASE: [(&[i32], &[i32]); 1] = [(&[5, 2, 3, 1], &[1, 2, 3, 5])];

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    #[cfg(not)]
    pub fn quick_sort_entrance(mut nums: Vec<i32>) -> Vec<i32> {
        Solution::quick_sort(0, nums.len() - 1, &mut nums);
        return nums;
    }

    fn quick_sort(start: usize, end: usize, nums: &mut Vec<i32>) {
        if start >= end {
            return;
        }
        let pivot = nums[start + (end - start) / 2];
        let mut left = start;
        let mut right = end;
        while left <= right {
            while left <= right && nums[left] < pivot {
                left += 1;
            }
            while left <= right && nums[right] > pivot {
                right -= 1;
            }
            if left <= right {
                let temp = nums[right];
                nums[right] = nums[left];
                nums[left] = temp;
                left += 1;
                right -= 1;
            }
        }
        Solution::quick_sort(start, right, nums);
        Solution::quick_sort(left, end, nums);
    }
}

#[test]
fn test_quick_sort() {
    for &(input, expected) in TEST_CASE.iter() {
        let mut input: Vec<i32> = input.to_vec();
        Solution::quick_sort(0, input.len()-1, &mut input);
        let expected: Vec<i32> = expected.to_vec();
        assert_eq!(input, expected);
    }
}