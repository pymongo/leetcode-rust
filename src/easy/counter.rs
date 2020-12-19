//! 所有用到了类似Python的collections.Counter的题型

/** https://leetcode.com/problems/number-of-good-pairs/
count if nums[i] == nums[j] and i < j
思路: collection.Counter统计输入数组每个数字的出现次数，例如3出现了3次，那么就有math.comb(3,2)=3*2=6对满足i<j且nums[i]==nums[j]

除了Counter，还有已知输入全小写字母字符串需要一些记忆操作例如最长无重复子串，也建议用固定长数组，下标寻址比HashMap快
> return sum(map(lambda v: math.comb(v, 2), collections.Counter(nums).values()))
*/
fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut counter = [0u8; 101];
    for num in nums.into_iter() {
        counter[num as usize] += 1;
    }
    // array暂不支持into_iter, issue#66145
    counter
        .iter()
        .map(|&v| (v as i32 - 1) * v as i32 / 2)
        .sum::<i32>()
}

/// 以下都是number_of_good_pairs不断迭代的过程(counter算法的几重境界)
fn number_of_good_pairs_v1(nums: Vec<i32>) -> i32 {
    let mut counter = std::collections::HashMap::<i32, i32>::new();
    for &num in nums.iter() {
        *counter.entry(num).or_default() += 1;
    }
    counter.iter().map(|(_k, &v)| (v - 1) * v / 2).sum()
}

fn number_of_good_pairs_v2(nums: Vec<i32>) -> i32 {
    let mut counter = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
    for num in nums.into_iter() {
        *counter.entry(num).or_default() += 1;
    }
    counter.into_iter().map(|(_k, v)| (v - 1) * v / 2).sum()
}

/**
既然题目提示了nums的长度范围是1..=100，意味着计数的范围也是1到100
那么or_insert(0)可以优化成or_insert(0u8)，0u8比默认的0i32节省3/4内存
但是HashMap的value是u8的话，(v-1)*v/2会有两个溢出问题:
1. v-1当v=0时会向下溢出成负数，v-1要改成v.saturating_sub(1)
2. (V-1)*v两个u8相乘可能会超过255
*/
fn number_of_good_pairs_v3(nums: Vec<i32>) -> i32 {
    let mut counter = std::collections::HashMap::<i32, u8>::with_capacity(nums.len());
    for num in nums.into_iter() {
        *counter.entry(num).or_default() += 1;
    }
    counter
        .into_iter()
        .map(|(_k, v)| (v as i32 - 1) * v as i32 / 2)
        .sum::<i32>()
}

/// https://leetcode.com/problems/first-unique-character-in-a-string/
fn first_unique_char(s: String) -> i32 {
    let s = s.into_bytes();
    let mut counter = [0u16; 26];
    for &byte in s.iter() {
        counter[(byte - b'a') as usize] += 1;
    }
    for (i, byte) in s.into_iter().enumerate() {
        if counter[(byte - b'a') as usize] == 1 {
            return i as i32;
        }
    }
    -1
}

/// https://leetcode.com/problems/group-anagrams/
/// 由于Python没有原始数组，list是可变的不能Hash，所以list要转为tuple多了很多额外的操作
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut group = std::collections::HashMap::new();
    for s in strs.into_iter() {
        let mut counter = [0u8; 26];
        for &byte in s.as_bytes() {
            counter[(byte - b'a') as usize] += 1;
        }
        group.entry(counter).or_insert_with(Vec::new).push(s);
    }
    // same as nightly `into_values` API: consume HashMap and get a vec of values
    group.into_iter().map(|(_k, v)| v).collect()
}

/// https://leetcode.com/problems/top-k-frequent-elements/
/// return [num for num, _ in collections.Counter(nums).most_common(k)]
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut counter = std::collections::HashMap::<i32, i32>::with_capacity(n);
    for &num in &nums {
        *counter.entry(num).or_default() += 1;
    }
    // 小根堆: (-出现次数, 数字)，所以堆顶会是出现次数最低的数字，随时可以被别人挤掉
    let mut heap = std::collections::BinaryHeap::<(i32, i32)>::with_capacity(k);
    for (&num, &cnt) in &counter {
        if heap.len() == k {
            if -cnt < heap.peek().unwrap().0 {
                heap.pop();
                heap.push((-cnt, num));
            }
        } else {
            heap.push((-cnt, num));
        }
    }
    heap.into_iter().rev().map(|(_, num)| num).collect()
}

/// https://leetcode.com/problems/increasing-decreasing-string/
#[allow(clippy::needless_range_loop)]
fn sort_string(s: String) -> String {
    let n = s.len();
    let mut counter = [0u8; 26];
    for byte in s.into_bytes().into_iter() {
        counter[(byte - b'a') as usize] += 1;
    }

    let mut ret = Vec::with_capacity(n);
    while ret.len() < n {
        for i in 0..26 {
            if counter[i] > 0 {
                counter[i] -= 1;
                ret.push(b'a' + i as u8);
            }
        }
        for i in (0..26).rev() {
            if counter[i] > 0 {
                counter[i] -= 1;
                ret.push(b'a' + i as u8);
            }
        }
    }

    unsafe { String::from_utf8_unchecked(ret) }
}

#[test]
fn test_sort_string() {
    const TEST_CASES: [(&str, &str); 2] =
        [("aaaabbbbcccc", "abccbaabccba"), ("leetcode", "cdelotee")];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(sort_string(input.to_string()), output.to_string());
    }
}

/**
需要用到第三方库itertools进行unique操作的解法
```compile_fail
fn unique_occurrences(arr: Vec<i32>) -> bool {
    arr.into_iter()
        .map(|x| arr.into_iter().filter(|&y| x==y).count())
        .unique()
        .sum::<usize>()==arr.len()
}
```
*/
fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut counter = std::collections::HashMap::new();
    for num in arr {
        *counter.entry(num).or_insert(0) += 1;
    }
    let mut seen_count = std::collections::HashSet::new();
    /* Second solution
    for &count in counter.values() {
        seen_count.insert(count)
    }
    return counter.len() == seen_count.len();
    */
    for &count in counter.values() {
        if seen_count.contains(&count) {
            return false;
        } else {
            seen_count.insert(count);
        }
    }
    true
}

#[test]
fn test_unique_occurrences() {
    assert_eq!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    assert_eq!(unique_occurrences(vec![1, 2]), false);
}

/// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
/// 前缀和还有一个应用是币币交易撮合引擎的orderbook深度展示
fn smaller_numbers_than_current_prefix_sum_solution(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix_sum_counter = [0i32; 101];
    for &num in &nums {
        prefix_sum_counter[num as usize] += 1;
    }
    for i in 1..=100 {
        prefix_sum_counter[i] += prefix_sum_counter[i - 1];
    }

    let mut res = Vec::with_capacity(nums.len());
    for num in nums {
        if num == 0 {
            res.push(0);
        } else {
            res.push(prefix_sum_counter[(num - 1) as usize]);
        }
    }
    res
}

#[test]
fn test_smaller_numbers_than_current() {
    const TEST_CASES: [(&[i32], &[i32]); 1] = [(&[8, 1, 2, 2, 3], &[4, 0, 1, 1, 3])];
    for &(nums, output) in &TEST_CASES {
        assert_eq!(
            smaller_numbers_than_current_prefix_sum_solution(nums.to_vec()),
            output.to_vec()
        );
    }
}
