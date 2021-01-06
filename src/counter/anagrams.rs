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
