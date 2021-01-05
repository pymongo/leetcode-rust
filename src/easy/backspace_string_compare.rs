/// '#'表示退格操作，请你比较两个含退格操作符的字符串是否相等
fn backspace_compare(s: String, t: String) -> bool {
    fn parse(s: String) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::new();
        for byte in s.into_bytes() {
            if byte == b'#' {
                let _ = ret.pop();
            } else {
                ret.push(byte);
            }
        }
        ret
    }
    parse(s) == parse(t)
}

#[test]
fn test_backspace_compare() {
    const TEST_CASES: [(&str, &str, bool); 4] = [
        ("ab#c", "ad#c", true),
        ("ab##", "c#d#", true),
        ("a##c", "#a#c", true),
        ("a#c", "b", false),
    ];
    for &(s, t, expected) in &TEST_CASES {
        assert_eq!(
            Solution::backspace_compare(s.to_string(), t.to_string()),
            expected
        );
    }
}
