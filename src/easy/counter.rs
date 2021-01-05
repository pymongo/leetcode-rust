//! 所有用到了类似Python的collections.Counter的题型，以及没用到Counter但是有计数需求的简单

/** https://leetcode.com/problems/number-of-good-pairs/
count if nums[i] == nums[j] and i < j
思路: collection.Counter统计输入数组每个数字的出现次数，例如3出现了3次，那么就有math.comb(3,2)=3*2=6对满足i<j且nums[i]==nums[j]

除了Counter，还有已知输入全小写字母字符串需要一些记忆操作例如最长无重复子串，也建议用固定长数组，下标寻址比HashMap快
> return sum(map(lambda v: math_or_puzzle_game.comb(v, 2), collections.Counter(nums).values()))
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

/// https://leetcode.com/problems/count-good-meals/
fn count_pairs_permutation_solution(nums: Vec<i32>) -> i32 {
    fn is_power_of_2(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        n & (n - 1) == 0
    }
    let mut counter_map = std::collections::BTreeMap::new();
    for num in nums.into_iter() {
        *counter_map.entry(num).or_insert(0) += 1;
    }
    let n = counter_map.keys().len();
    let mut unique = Vec::with_capacity(n);
    let mut counter = Vec::with_capacity(n);
    for (key, value) in counter_map.into_iter() {
        unique.push(key);
        counter.push(value as i64);
    }
    // unique.sort_unstable(); // 由于nums是有序的，插入counter时也是有序的，所以不用排序
    let n = unique.len();
    let mut ret = 0i64;
    for i in 0..n {
        for j in i..n {
            if is_power_of_2(unique[i] + unique[j]) {
                if i == j {
                    // math_or_puzzle_game.comb(count, 2)
                    ret += (counter[i] - 1) * counter[i] / 2;
                } else {
                    ret += counter[i] * counter[j];
                }
            }
        }
    }
    (ret % (10i64.pow(9) + 7)) as i32
}

/// 由于 0<=nums[i]<=2^20，所以nums[i]+nums[i]只可能是2^0..=2^21，最小是0+1，最大是2^20+2^20
/// leetcode.com版本太低不支持const fn内while loop
const fn gen_power_of_2() -> [i32; 22] {
    let mut ret = [0i32; 22];
    let mut i = 0;
    while i < 22 {
        ret[i] = 1 << i;
        i += 1;
    }
    ret
}

/// 生成从2^0到2^N次方的等比数列
const fn gen_twos_geometric_series<const N: usize>() -> [i32; N] {
    let mut ret = [0i32; N];
    let mut i = 0usize;
    while i < N {
        ret[i] = 2i32.pow(i as u32);
        i += 1;
    }
    ret
}

#[test]
fn test_gen_twos_geometric_series() {
    println!("{:?}", gen_twos_geometric_series::<22>());
}

/// 这个解法也就是把时间复杂度从O(n^2)降低到O(22n）
fn count_pairs_two_sum_solution(nums: Vec<i32>) -> i32 {
    // 照顾下leetcode.com的const fn不支持while loop
    // const TWO_SUMS: [i32; 22] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152];
    const TWO_SUMS: [i32; 22] = gen_twos_geometric_series::<22>();
    let mut counter = std::collections::BTreeMap::new();
    let mut ret = 0i64;
    for num in nums.into_iter() {
        for &two_sum in &TWO_SUMS {
            let target = two_sum - num;
            if target < 0 {
                continue;
            }
            if let Some(&count) = counter.get(&target) {
                ret += count as i64;
            }
        }
        *counter.entry(num).or_insert(0i32) += 1;
    }
    (ret % (10i64.pow(9) + 7)) as i32
}

#[test]
fn test_count_pairs() {
    const TEST_CASES: [(&[i32], i32); 4] = [
        (&[1, 1, 3, 7, 15, 31, 63, 127, 255, 511], 17),
        (
            &[
                149, 107, 1, 63, 0, 1, 6867, 1325, 5611, 2581, 39, 89, 46, 18, 12, 20, 22, 234,
            ],
            12,
        ),
        (&[1, 3, 5, 7, 9], 4),
        (&[1, 1, 1, 3, 3, 3, 7], 15),
    ];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(count_pairs_permutation_solution(input.into()), output);
        assert_eq!(count_pairs_two_sum_solution(input.into()), output);
    }
}

/// https://leetcode.com/problems/valid-anagram/
fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counter = [0u16; 26];
    for each in s.into_bytes().into_iter() {
        counter[(each - b'a') as usize] += 1;
    }
    for each in t.into_bytes().into_iter() {
        let idx = (each - b'a') as usize;
        if counter[idx] == 0 {
            return false;
        } else {
            counter[idx] -= 1;
        }
    }
    true
}
