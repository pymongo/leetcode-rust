struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut res: i32 = 0;
        let b = &str[..].get;
        dbg!(b);
        let mut chars = str.chars();
        let mut is_positive: bool = true;
        loop {
            let char = chars.next();
            if char == None {
                break;
            }
            let char = char.unwrap();
            match char {
                '0'..='9' => {

                },
                'a'..='z' => {
                    // 测试用例: ("words and 987", 0)
                },
                '-' => is_positive = false,
                _ => continue
            }
        }

        return res;
    }
}

#[cfg(test)]
const TEST_CASES: [(&str, i32); 4] = [
    ("   -42", -42),
    ("4193 with words", 4193),
    ("words and 987", 0),
    ("-91283472332", -2147483648)
];

#[test]
fn test_solution() {
    for (s, integer) in &TEST_CASES {
        assert_eq!(Solution::my_atoi(s.to_string()), *integer);
    }
}