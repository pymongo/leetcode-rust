/// https://leetcode.com/problems/peak-index-in-a-mountain-array/
fn peak_index(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] > nums[mid + 1] {
            if nums[mid] > nums[mid - 1] {
                return mid as i32;
            }
            right = mid;
        } else {
            left = mid;
        }
    }
    left as i32
}

fn peak_index_brute_force(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    while nums[i] < nums[i + 1] {
        i += 1;
    }
    i as i32
}

fn peak_index_best(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[mid + 1] {
            left = mid + 1;
        } else {
            // mid索引不满足递增条件，所以山顶会在[left,mid]之间，但不能排除掉mid，mid可能是个peak,也可能是下坡的其中一个索引
            right = mid;
        }
    }
    left as i32
}

#[test]
fn test_peak_index_in_a_mountain_array() {
    const TEST_CASES: [(&[i32], i32); 2] = [
        (&[0, 2, 1, 0], 1),
        (&[24, 69, 100, 99, 79, 78, 67, 36, 26, 19], 2),
    ];
    for (nums, peak_index_output) in TEST_CASES {
        assert_eq!(peak_index(nums.to_vec()), peak_index_output);
        assert_eq!(peak_index_brute_force(nums.to_vec()), peak_index_output);
        assert_eq!(peak_index_best(nums.to_vec()), peak_index_output);
    }
}
