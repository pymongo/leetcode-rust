/// https://leetcode.com/problems/increasing-decreasing-string/
#[allow(clippy::needless_range_loop)]
fn sort_string(s: String) -> String {
    let n = s.len();
    let mut counter = [0_u8; 26];
    for byte in s.into_bytes() {
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
    for (input, output) in TEST_CASES {
        assert_eq!(sort_string(input.to_string()), output.to_string());
    }
}
