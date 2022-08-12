//! 所有用到了类似Python的collections.Counter的题型(包括very_easy的题)，以及没用到Counter但是有计数需求的简单
//! 或者用到类似计数过程，有的题可能需要组合数
mod anagrams;
mod count_good_meals;
mod count_num1_square_eq_two_num2_product;
mod find_all_numbers_disappeared_in_an_array;
mod increasing_decreasing_string;
mod number_of_equivalent_domino_pairs;
mod number_of_good_pairs;

/// https://leetcode.com/problems/first-unique-character-in-a-string/
fn first_unique_char(s: String) -> i32 {
    let s = s.into_bytes();
    let mut counter = [0_u16; 26];
    for &byte in &s {
        counter[(byte - b'a') as usize] += 1;
    }
    for (i, byte) in s.into_iter().enumerate() {
        if counter[(byte - b'a') as usize] == 1 {
            return i as i32;
        }
    }
    -1
}

/** https://leetcode.com/problems/unique-number-of-occurrences/
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
        }
        seen_count.insert(count);
    }
    true
}

#[test]
fn test_unique_occurrences() {
    assert!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    assert!(!unique_occurrences(vec![1, 2]));
}

/// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
/// 前缀和还有一个应用是币币交易撮合引擎的orderbook深度展示
fn smaller_numbers_than_current_prefix_sum_solution(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix_sum_counter = [0_i32; 101];
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

/// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut counter = [0_u8; 26];
    for each in chars.into_bytes() {
        counter[(each - b'a') as usize] += 1;
    }
    let mut ret = 0_usize;
    'next_word: for word in words {
        let mut cnt = counter;
        let word_len = word.len();
        for each in word.into_bytes() {
            let idx = (each - b'a') as usize;
            if cnt[idx] == 0 {
                continue 'next_word;
            }
            cnt[idx] -= 1;
        }
        ret += word_len;
    }
    ret as i32
}

/// https://leetcode.com/problems/intersection-of-three-sorted-arrays/
fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
    let mut counter = [0_u8; 2001];
    for num in arr1 {
        counter[num as usize] += 1;
    }
    for num in arr2 {
        counter[num as usize] += 1;
    }
    for num in arr3 {
        counter[num as usize] += 1;
    }
    let mut ret = vec![];
    for (i, count) in counter.iter().enumerate().take(2000).skip(1) {
        if *count == 3 {
            ret.push(i as i32);
        }
    }
    ret
}

/// https://leetcode.com/problems/sum-of-unique-elements/
fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut counter = [0_u8; 101];
    for num in nums {
        counter[num as usize] += 1;
    }
    let mut ret = 0;
    for (num, &count) in counter.iter().enumerate() {
        if count == 1 {
            ret += num;
        }
    }
    ret as i32
}

/** 测试 number-of-good-pairs 一题用固定长数组作为counter和HashMap作为counter的性能

> cargo bench -- counter::bench_hashmap_counter_vs_fixed_len_array_counter

number-of-good-pairs一题数值的范围在1..=100，而且长度范围也在1..=100
所谓counter值得就是类似Python collections.Counter行为的操作

除了Counter，还有已知输入全小写字母字符串需要一些记忆操作例如最长无重复子串，也建议用固定长数组，下标寻址比HashMap快

test bench_fixed_len_array_counter ... bench:          52 ns/iter (+/- 10)
test bench_hashmap_counter         ... bench:       1,576 ns/iter (+/- 158)
*/
#[cfg(test)]
mod bench_hashmap_counter_vs_fixed_len_array_counter {
    const NUMS: [i32; 97] = [
        3, 2, 3, 5, 4, 4, 2, 3, 5, 1, 4, 3, 2, 2, 5, 3, 1, 5, 4, 5, 3, 2, 5, 2, 3, 3, 2, 1, 4, 5,
        3, 3, 5, 5, 5, 2, 5, 5, 2, 1, 4, 4, 2, 2, 3, 4, 2, 1, 5, 3, 2, 2, 2, 1, 1, 1, 3, 4, 5, 4,
        5, 4, 3, 4, 3, 4, 3, 4, 5, 1, 5, 5, 1, 2, 2, 3, 5, 2, 3, 1, 3, 5, 1, 5, 1, 4, 2, 1, 5, 5,
        1, 3, 3, 2, 5, 3, 1,
    ];

    #[bench]
    fn bench_hashmap_counter(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let mut counter = std::collections::HashMap::with_capacity(NUMS.len());
            for num in NUMS {
                *counter.entry(num).or_insert(0_u8) += 1;
            }
            counter
                .into_iter()
                .map(|(_k, v)| (i32::from(v) - 1) * i32::from(v) / 2)
                .sum::<i32>();
        });
    }

    #[bench]
    fn bench_fixed_len_array_counter(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let mut counter = [0_u8; 101];
            for num in NUMS {
                counter[num as usize] += 1;
            }
            counter
                .iter()
                .map(|&v| (i32::from(v) - 1) * i32::from(v) / 2)
                .sum::<i32>();
        });
    }
}

/// https://leetcode.com/problems/sort-characters-by-frequency/
fn frequency_sort(s: String) -> String {
    let mut counter = [0_u16; 122];
    for chr in s.into_bytes() {
        counter[chr as usize] += 1;
    }
    let mut counter = counter
        .iter()
        .enumerate()
        .filter(|(_, &count)| count > 0)
        .map(|(chr, &count)| (chr as u8, count))
        .collect::<Vec<_>>();
    counter.sort_unstable_by_key(|x| x.1);
    counter.reverse();
    let mut ret = vec![];
    for (chr, count) in counter {
        ret.append(&mut [chr].repeat(count as usize));
    }
    unsafe { String::from_utf8_unchecked(ret) }
}

/// https://leetcode.com/problems/ransom-note/
fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut counter = [0u16; 26];
    for byte in magazine.into_bytes() {
        counter[usize::from(byte - b'a')] += 1;
    }
    for byte in ransom_note.into_bytes() {
        let i = usize::from(byte - b'a');
        // counter[i].checked_sub
        if counter[i] == 0 {
            return false;
        }
        counter[i] -= 1;
    }
    true
}

/// https://leetcode.com/problems/maximum-number-of-balloons/
fn max_number_of_balloons(text: String) -> i32 {
    let mut counter = [0; 26];
    for letter in text.into_bytes() {
        counter[(letter - b'a') as usize] += 1;
    }
    let mut times = 0;
    'out: loop {
        for letter in "balloon".bytes() {
            if counter[(letter - b'a') as usize] >= 1 {
                counter[(letter - b'a') as usize] -= 1;
            } else {
                break 'out;
            }
        }
        times += 1;
    }
    times
}

/// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = std::collections::HashMap::<i32, Vec<i32>>::new();
    for (id, group_size) in group_sizes.into_iter().enumerate() {
        map.entry(group_size).or_default().push(id as i32);
    }
    let mut ret = Vec::new();
    for (group_size, ids) in map {
        for chunk in ids.chunks(group_size as usize) {
            ret.push(chunk.to_vec());
        }
    }
    ret
}
