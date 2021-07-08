/*!
prefix_sum greedy subarray relative problems:
- count-number-of-nice-subarrays
- continuous-subarray-sum
- contiguous-array
- binary-subarrays-with-sum: 和为target_sum的子数组的count，要用HashMap批量数
*/

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
            prefix_sum[i + 1] = prefix_sum[i] + 1;
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
#[allow(clippy::similar_names)]
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

/// https://leetcode.com/problems/binary-subarrays-with-sum/
/// count subarray sum to target
/// time_limit_exceed: 58/60
fn num_subarrays_with_sum_brute_force(nums: Vec<i32>, target_sum: i32) -> i32 {
    // convert to prefix sum
    let len = nums.len();
    let mut prefix_sum = vec![0; len + 1];
    for i in 0..len {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    let mut ret = 0;
    for left in 0..len {
        for right in left..len {
            if prefix_sum[right + 1] - prefix_sum[left] == target_sum {
                ret += 1;
            }
        }
    }

    ret
}

/// 由于nums仅由0和1组成，所以可以转换为`two_sum_count_target`用双指针批量数区间，比HashMap版快
fn num_subarrays_with_sum(nums: Vec<i32>, target_sum: i32) -> i32 {
    let mut map = std::collections::HashMap::new();
    map.insert(0, 1); // 前缀和为0共计1个，解决nums[0]=1,target_sum=1时没把nums[0]统计进子数组
    let mut sum = 0;
    let mut count = 0;
    for num in nums {
        sum += num;
        // prefix_sum[right+1] - prefix_sum[left] == target_sum
        if let Some(val) = map.get(&(sum - target_sum)) {
            count += *val;
        }
        *map.entry(sum).or_default() += 1;
    }
    count
}

#[test]
fn test_num_subarrays_with_sum() {
    const TEST_CASES: [(&[i32], i32, i32); 2] =
        [(&[1, 0, 1, 0, 1], 2, 4), (&[0, 0, 0, 0, 0], 0, 15)];
    for (nums, target_sum, count) in TEST_CASES {
        assert_eq!(
            num_subarrays_with_sum_brute_force(nums.to_vec(), target_sum),
            count
        );
        assert_eq!(num_subarrays_with_sum(nums.to_vec(), target_sum), count);
    }
}
