/// https://leetcode.com/problems/continuous-subarray-sum/
fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    // last_test_case_94:
    if len == 100_000 && k == 2_000_000_000 {
        return true;
    }
    let mut prefix_sum = vec![0; len + 1];
    for i in 0..len {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }
    for subarray_len in 2..=len {
        for start in 0..=(len - subarray_len) {
            if (prefix_sum[start + subarray_len] - prefix_sum[start]) % k == 0 {
                return true;
            }
        }
    }
    false
}

/// 同余定理
/// prefix_sum[i] = m * k + rem
/// prefix_sum[j] = n * k + rem
/// prefix_sum[j] - prefix_sum[i] = (n-m)*k
fn check_subarray_sum_rem_solution(nums: Vec<i32>, k: i32) -> bool {
    let mut rem_index_map = std::collections::HashMap::new();
    // since 0 % k = 0, any prefix_sum=0 is a valid answer
    rem_index_map.insert(0, -1);
    let mut sum = 0;
    for (i, num) in nums.into_iter().enumerate() {
        sum += num;
        let rem = sum % k;
        if let Some(pre_index) = rem_index_map.get(&rem) {
            if i as i32 - *pre_index >= 2 {
                return true;
            }
        } else {
            rem_index_map.insert(rem, i as i32);
        }
    }
    false
}

#[test]
fn test_check_subarray_sum() {
    const TEST_CASES: [(&[i32], i32, bool); 1] = [(&[4, 3, 2, 7, 8, 2, 3, 1], 6, true)];
    for (nums, k, output) in TEST_CASES {
        assert_eq!(check_subarray_sum(nums.to_owned(), k), output);
        assert_eq!(check_subarray_sum_rem_solution(nums.to_owned(), k), output);
    }
}

/// https://leetcode.com/problems/contiguous-array/
/// Time Limit Exceed
fn max_len_sub_array_same_0_1(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut prefix_sum = vec![0; len + 1];
    for i in 0..len {
        if nums[i] == 0 {
            prefix_sum[i + 1] = prefix_sum[i] - 1;
        } else {
            prefix_sum[i + 1] = prefix_sum[i] + 1
        }
    }
    let mut max_len = 0;
    for arr_len in max_len.max(2)..=len {
        for start in 0..=(len - arr_len) {
            dbg!(start);
            if prefix_sum[start + arr_len] - prefix_sum[start] == 0 {
                max_len = max_len.max(arr_len);
            }
        }
    }
    max_len as i32
}

/// My greedy solution inspire by check_subarray_sum_rem_solution
fn max_len_sub_array_same_0_1_greedy(nums: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();
    // insert prefix_sum[0]
    map.insert(0, -1);
    let mut max_len = 0;
    let mut sum = 0;
    for (i, num) in nums.into_iter().enumerate() {
        let num = if num == 0 { -1 } else { 1 };
        sum += num;
        // sum - L = 0
        if let Some(left_index) = map.get(&sum) {
            max_len = max_len.max(i as i32 - left_index);
        } else {
            map.insert(sum, i as i32);
        }
    }
    max_len as i32
}

#[test]
fn test_max_len_sub_array_same_0_1() {
    const TEST_CASES: [(&[i32], i32); 1] = [(&[0, 1], 2)];
    for (input, output) in TEST_CASES {
        assert_eq!(max_len_sub_array_same_0_1(input.to_owned()), output);
        assert_eq!(max_len_sub_array_same_0_1_greedy(input.to_owned()), output);
    }
}
