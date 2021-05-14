/// https://leetcode.com/problems/di-string-match/
fn di_string_match(s: String) -> Vec<i32> {
    let n = s.len();
    let mut ret = Vec::with_capacity(n + 1);
    let (mut lo, mut hi) = (0_i32, n as i32);
    for ch in s.into_bytes() {
        if ch == b'I' {
            ret.push(lo);
            lo += 1;
        } else {
            ret.push(hi);
            hi -= 1;
        }
    }
    ret.push(hi); // or ret.push(lo);
    ret
}

#[test]
fn test_di_string_match() {
    const TEST_CASES: [(&str, &[i32]); 2] = [
        // 输入一个仅有I和D组成的字符串，例如输入"IDID"长度为4的字符串，则准备0..=4总共5个数
        // 第一个I表示ret[1]>ret[0]，第二个D表示ret[2]<ret[1]
        ("IDID", &[0, 4, 1, 3, 2]),
        ("DDI", &[3, 2, 0, 1]),
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(di_string_match(input.to_string()), output);
    }
}
