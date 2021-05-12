/// https://leetcode.com/problems/valid-anagram/
fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counter = [0_u16; 26];
    for each in s.into_bytes() {
        counter[(each - b'a') as usize] += 1;
    }
    for each in t.into_bytes() {
        let idx = (each - b'a') as usize;
        if counter[idx] == 0 {
            return false;
        } else {
            counter[idx] -= 1;
        }
    }
    true
}

/// https://leetcode.com/problems/group-anagrams/
/// 由于Python没有原始数组，list是可变的不能Hash，所以list要转为tuple多了很多额外的操作
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut group = std::collections::HashMap::new();
    for s in strs {
        let mut counter = [0_u8; 26];
        for &byte in s.as_bytes() {
            counter[(byte - b'a') as usize] += 1;
        }
        group.entry(counter).or_insert_with(Vec::new).push(s);
    }
    // same as nightly `into_values` API: consume HashMap and get a vec of values
    group.into_iter().map(|(_k, v)| v).collect()
}

/// https://leetcode.com/problems/find-anagram-mappings/
/// 有两个互为anagram的数组(anagram的定义请看valid_anagram一题)
/// 请你找出A的每个元素在B中的索引，如有重复元素则返回任意一个索引
/// 例如A=[12,28],B=[28,12]，因为A[0]=B[1],A[1]=B[0]，所以返回[1,0]
fn anagram_mappings(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut map = std::collections::HashMap::with_capacity(b.len());
    for (i, b) in b.into_iter().enumerate() {
        map.insert(b, i as i32);
    }
    for a in a.iter_mut() {
        *a = *map.get(a).unwrap();
    }
    a
}
