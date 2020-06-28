//! https://leetcode.com/problems/longest-substring-without-repeating-characters/
//! ## 暴力遍历
//! 两层for循环，穷举出所有可能的字符串逐个进行检查，通过HashSet等数据结构存储已出现过(seen)的字符串来提升性能
//!
//! ## sliding_window
//! 通过左右两个浮标，按右浮标遍历String时发现重复的字符串时，让左浮标往右移到重复字符串的位置，从而缩小搜索范围
//! 时间复杂度是O(N)，最坏的情况O(2N)左右浮标分别完整的遍历一次字符串
//!
//! ## (数据结构)ascii记录字符出现位置
//! 比HashMap<char, Integer>更高效，用一个长度为128的数组，索引是ascii字母值，value是在字符串中的索引的存储结构效率最高

pub const TEST_CASES: [(&str, i32); 5] = [
    ("abcabcbb", 3),
    ("bbbbb", 1),
    ("pwwkew", 3),
    ("abcabcbb", 3),
    ("abba", 2),
];

#[test]
fn test_i32_ascii_table() {
    for &(input, expected) in TEST_CASES.iter() {
        assert_eq!(usize_ascii_table(input.to_string()), expected)
    }
}

pub fn i32_ascii_table(s: String) -> i32 {
    let len: i32 = s.len() as i32;
    if len <= 1 {
        return len;
    }
    // TODO array的值用usize更好，只不过没有像-1这样显著体现未初始化状态可读性好
    // 如果假定输入字符串最大长度是1000，确实可用1000作未初始化的值
    // index: value of char in ascii
    // value: index of char
    let mut ascii_table: [i32; 128] = [-1; 128];
    let (mut left, mut right, mut max) = (0i32, 0i32, 0i32);
    let mut temp_max: i32;
    let string_bytes = s.as_bytes();
    let mut current_char: u8;
    while right < len {
        current_char = string_bytes[right as usize];
        if ascii_table[current_char as usize] != -1 {
            // 例如abba的用例，end=2时(第二个b)，start会跳到2
            // slider.1指到3时(最后一个a)，第二次出现重复时，重复的是a，ascii_table中字母a的索引是0
            // 如果不进行判断start会后退到0+1
            // max() prevent sliders.0's index go back (test case: abba)
            // 避免重复的字符「不在当前的移动窗口中」
            left = left.max(ascii_table[current_char as usize] + 1);
        }
        ascii_table[current_char as usize] = right;
        temp_max = right - left;
        if temp_max > max {
            max = temp_max;
        }
        right += 1;
    }
    max + 1
}

#[test]
fn test_usize_ascii_table() {
    for &(input, expected) in TEST_CASES.iter() {
        assert_eq!(i32_ascii_table(input.to_string()), expected)
    }
}

pub fn usize_ascii_table(s: String) -> i32 {
    let len = s.len();
    if len <= 1 {
        return len as i32;
    }
    // 假设数组最大长度是31001(leetcode最长测试用例)，用1024表示ascii_table中没有出现过这个字符串
    const UNINITIALIZED: usize = 31001;
    let mut ascii_table: [usize; 128] = [UNINITIALIZED; 128];
    let (mut left, mut right, mut max) = (0usize, 0usize, 0usize);
    let mut temp_max: usize;
    let string_bytes = s.as_bytes();
    let mut current_char: u8;
    while right < len {
        current_char = string_bytes[right as usize];
        if ascii_table[current_char as usize] != UNINITIALIZED {
            left = left.max(ascii_table[current_char as usize] + 1);
        }
        ascii_table[current_char as usize] = right;
        temp_max = right - left;
        if temp_max > max {
            max = temp_max;
        }
        right += 1;
    }
    max as i32 + 1
}

#[cfg(not)]
#[test]
fn test_brute_force() {
    for &(input, expected) in TEST_CASES.iter() {
        assert_eq!(my_brute_force(input.to_string()), expected)
    }
}

// 164 ms, faster than 12.34%
// 这个方法太慢了，其实一层遍历就够了
#[cfg(not)]
fn my_brute_force(s: String) -> i32 {
    let mut max: usize = 0;
    let mut uniques: std::collections::BTreeSet<char> = std::collections::BTreeSet::new();
    for (i, char1) in s.chars().enumerate() {
        uniques.clear();
        uniques.insert(char1);
        for char2 in s[i + 1..].chars() {
            if uniques.contains(&char2) {
                // 保证连续不重复
                break;
            } else {
                uniques.insert(char2);
            }
        }
        max = max.max(uniques.len());
    }
    max as i32
}

#[cfg(not)]
fn btree_brute_force(s: String) -> i32 {
    let size: usize = s.len();
    let mut index: usize = 0;
    let mut char: char;
    let mut max: usize = 0;
    let mut unique: std::collections::BTreeSet<char> = std::collections::BTreeSet::new();

    while index < size {
        char = s.chars().nth(index).unwrap();
        if unique.contains(&char) {
            println!(" same! {}", char);
            max = max.max(unique.len());
            unique.clear();
        } else {
            print!("unique {}", char);
            unique.insert(char);
            index += 1;
        }
    }
    // 防止连续不重复的字符串出现在最后
    max = max.max(unique.len());
    max as i32
}
