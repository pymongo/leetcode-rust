/** https://leetcode.com/problems/number-of-good-pairs/
count if nums[i] == nums[j] and i < j
思路: collection.Counter统计输入数组每个数字的出现次数，例如3出现了3次，那么就有math.comb(3,2)=3*2=6对满足i<j且nums[i]==nums[j]

除了Counter，还有已知输入全小写字母字符串需要一些记忆操作例如最长无重复子串，也建议用固定长数组，下标寻址比HashMap快
> return sum(map(lambda v: math.comb(v, 2), collections.Counter(nums).values()))
*/
fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut counter = [0_u8; 101];
    for num in nums {
        counter[num as usize] += 1;
    }
    // array暂不支持into_iter, issue#66145
    counter
        .iter()
        .map(|&v| (i32::from(v) - 1) * i32::from(v) / 2)
        .sum::<i32>()
}

/// 以下都是number_of_good_pairs不断迭代的过程(counter算法的几重境界)
fn number_of_good_pairs_v1(nums: Vec<i32>) -> i32 {
    let mut counter = std::collections::HashMap::<i32, i32>::new();
    for num in nums {
        *counter.entry(num).or_default() += 1;
    }
    counter.iter().map(|(_k, &v)| (v - 1) * v / 2).sum()
}

fn number_of_good_pairs_v2(nums: Vec<i32>) -> i32 {
    let mut counter = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
    for num in nums {
        *counter.entry(num).or_default() += 1;
    }
    counter.into_values().map(|v| (v - 1) * v / 2).sum()
}

/**
既然题目提示了nums的长度范围是1..=100，意味着计数的范围也是1到100
那么or_insert(0)可以优化成or_insert(0_u8)，0_u8比默认的0_i32节省3/4内存
但是HashMap的value是u8的话，(v-1)*v/2会有两个溢出问题:
1. v-1当v=0时会向下溢出成负数，v-1要改成v.saturating_sub(1)
2. (V-1)*v两个u8相乘可能会超过255
*/
fn number_of_good_pairs_v3(nums: Vec<i32>) -> i32 {
    let mut counter = std::collections::HashMap::<i32, u8>::with_capacity(nums.len());
    for num in nums {
        *counter.entry(num).or_default() += 1;
    }
    counter
        .into_values()
        .map(|v| (i32::from(v) - 1) * i32::from(v) / 2)
        .sum::<i32>()
}
