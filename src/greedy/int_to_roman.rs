/*! https://leetcode.com/problems/integer-to-roman/
& https://leetcode.com/problems/roman-to-integer/

# 贪心: 每次尽可能匹配数值更大的罗马字母
例如M是最大的罗马字母，如果num=9000，则将M重复9次
```python
def in_to_roman(num: int) -> str:
    ret = ''
    for (integer, roman) in int_to_roman:
        if num >= integer:
            count, num = divmod(num, integer)
            ret += roman * count
    return ret
```

def roman_to_int(s: str) -> int:
    ret = 0
    # 用HashMap可能会快点
    for (integer, roman) in int_to_roman:
        n = len(roman)
        while s.startswith(roman):
            ret += integer
            s = s[n:]
    return ret
*/
const INT_TO_ROMAN: [(i32, &[u8]); 13] = [
    (1000, b"M"),
    (900, b"CM"),
    (500, b"D"),
    (400, b"CD"),
    (100, b"C"),
    (90, b"XC"),
    (50, b"L"),
    (40, b"XL"),
    (10, b"X"),
    (9, b"IX"),
    (5, b"V"),
    (4, b"IV"),
    (1, b"I"),
];

fn int_to_roman(mut num: i32) -> String {
    let mut ret = Vec::new();
    for &(int, roman) in &INT_TO_ROMAN {
        // while num >= int {
        //     ret.extend_from_slice(roman);
        //     num -= int;
        // }
        if num >= int {
            ret.extend(roman.repeat((num / int) as usize));
            num %= int;
        }
    }
    unsafe { String::from_utf8_unchecked(ret) }
}

fn roman_to_int(s: String) -> i32 {
    let s = s.into_bytes();
    let len = s.len();
    let mut i = 0;
    let mut ret = 0;
    for &(int, roman) in &INT_TO_ROMAN {
        let roman_len = roman.len();
        while s[i..].starts_with(roman) {
            ret += int;
            i += roman_len;
            if i >= len {
                return ret;
            }
        }
    }
    unreachable!()
}

#[test]
fn test() {
    const TEST_CASES: [(&str, i32); 2] = [
        ("LVIII", 58),
        ("MCMXCIV", 1994)
    ];
    for &(roman, int) in &TEST_CASES {
        assert_eq!(int_to_roman(int), roman);
        assert_eq!(roman_to_int(roman.to_string()), int);
    }
}
