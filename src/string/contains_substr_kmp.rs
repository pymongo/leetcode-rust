//! https://leetcode-cn.com/problems/implement-strstr/
//! 实现C语言自带的strstr()以及Java的indexOf() API
//! 功能：查找字符串中的一个子串的出现位置，类似于find()、contains() API

#[cfg(test)]
const TEST_CASE: [(&str, &str, i32); 3] = [
    ("hello", "ll", 2),
    ("aaaaa", "bba", -1),
    ("bike", "", 0),
];

#[test]
fn test_str_str() {
    for case in &TEST_CASE {
        dbg!(case);
        assert_eq!(str_str(case.0.to_owned(), case.1.to_owned()), case.2);
    }
}

fn str_str(haystack: String, needle: String) -> i32 {
    // if needle.is_empty() {
    //     return 0;
    // }
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1
    }
}

#[cfg(not)]
fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1
    }
}
