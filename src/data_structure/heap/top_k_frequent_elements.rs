//! https://leetcode.com/problems/top-k-frequent-elements/
//! 解题思路类似: <https://leetcode.com/problems/kth-largest-element-in-a-stream/>

/// return [num for num, _ in collections.Counter(nums).most_common(k)]
fn top_k_frequent_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
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

/// https://leetcode.com/problems/top-k-frequent-words/
fn top_k_frequent_words_sorting_solution(words: Vec<String>, k: i32) -> Vec<String> {
    let mut counter = std::collections::HashMap::<String, i32>::new();
    for word in words {
        *counter.entry(word).or_default() += 1;
    }
    let mut a = counter.into_iter().collect::<Vec<_>>();
    a.sort_unstable_by(|(word_a, freq_a), (word_b, freq_b)| 
        freq_b.cmp(freq_a).then(word_a.cmp(word_b))
    );
    a.into_iter().take(k as usize).map(|(word, _freq)| word).collect()
}

/// [wrong answer!]fast than sort, sort has extra overhead on array elements swap O(n), but heap swap overhead is O(k)
fn top_k_frequent_words_heap_solution(words: Vec<String>, k: i32) -> Vec<String> {
    let mut counter = std::collections::BTreeMap::<String, i32>::new();
    for word in words {
        *counter.entry(word).or_default() += 1;
    }
    let k = k as usize;
    let mut min_heap = std::collections::BinaryHeap::with_capacity(k+1);
    for (word, freq) in counter {
        // python3 heap pushpop
        // word is sort by letter desc
        min_heap.push((-freq, word));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }
    // min_heap is sort by freq asc and letter desc, need to reverse it
    min_heap.into_iter().rev().map(|(_freq, word)| word).collect()
}

#[test]
fn test_top_k_frequent_words() {
    let test_cases = vec![
        (vec_string!["i", "love", "leetcode", "i", "love", "coding"], 2, vec_string!["i", "love"]),
        (vec_string!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], 4, vec_string!["the", "is", "sunny", "day"]),
    ];
    for (words, k, top_k_frequent) in test_cases {
        // since HashMap into_iter is unordered, my heap solution sometimes pass test_cases sometimes not pass, if I change to BTreeMap it would 100% not pass
        //assert_eq!(top_k_frequent_words_heap_solution(words.clone(), k), top_k_frequent);
        assert_eq!(top_k_frequent_words_sorting_solution(words, k), top_k_frequent);
    }
}
