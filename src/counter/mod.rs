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

/// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut counter = [0u8; 26];
    for each in chars.into_bytes().into_iter() {
        counter[(each - b'a') as usize] += 1;
    }
    let mut ret = 0usize;
    'next_word: for word in words.into_iter() {
        let mut cnt = counter;
        let word_len = word.len();
        for each in word.into_bytes().into_iter() {
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
    let mut counter = [0u8; 2001];
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
    let mut counter = [0u8; 101];
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
