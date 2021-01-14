//! https://leetcode.com/problems/count-number-of-nice-subarrays/

/// O(n^2) time complex, not passed
fn number_of_subarrays_prefix_sum_brute_force(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    // nums数组延长一个长度，方便转为前缀和数组(nums[i]表示数组前i项里有几个奇数)
    nums.insert(0, 0);
    for i in 1..=n {
        nums[i] = nums[i - 1] + i32::from(nums[i] % 2 == 1);
    }
    let mut ret = 0;
    for i in 1..=n {
        // 因为k至少为1，因此j遍历到i的前一个位置即可
        for j in 0..i {
            // 判断子数组nums[j..=i]是否含有k个奇数
            if nums[i] - nums[j] == k {
                ret += 1;
            }
        }
    }
    ret
}

/// 有点贪心脑筋急转弯的思想，O(n)一次遍历
fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    // cnt有点像上面解法里的前缀和数组
    let mut cnt = vec![0u16; n + 1];
    // 拥有0个奇数的子数组有1个(就是空数组)
    cnt[0] = 1;
    // odd类似上面解法里的nums[i-1]
    let mut odd = 0usize;
    let k = k as usize;

    let mut ret = 0;
    for num in nums.into_iter() {
        if num % 2 == 1 {
            odd += 1;
        }
        if odd >= k {
            ret += cnt[odd - k] as i32;
        }
        cnt[odd] += 1;
    }

    ret
}
