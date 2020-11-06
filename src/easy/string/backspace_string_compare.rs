struct Solution;

/// '#'表示退格操作，请你比较两个含退格操作符的字符串是否相等
impl Solution {
    fn backspace_compare(s: String, t: String) -> bool {
        fn parse(s: String) -> Vec<u8> {
            let mut res: Vec<u8> = Vec::new();
            for byte in s.into_bytes() {
                if byte == b'#' {
                    let _ = res.pop();
                } else {
                    res.push(byte);
                }
            }
            res
        }
        parse(s) == parse(t)
    }
}

#[cfg(test)]
const TESTCASES: [(&str, &str, bool); 4] = [
    ("ab#c", "ad#c", true),
    ("ab##", "c#d#", true),
    ("a##c", "#a#c", true),
    ("a#c", "b", false),
];

#[test]
fn test_backspace_compare() {
    for &(s, t, expected) in &TESTCASES {
        assert_eq!(
            Solution::backspace_compare(s.to_string(), t.to_string()),
            expected
        );
    }
}
