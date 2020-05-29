//! https://leetcode.com/problems/to-lower-case/
//! |709|[To Lower Case](https://leetcode.com/problems/merge-sorted-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/impl_api/to_lowercase.rs)||
//! 我认为这题属于超简单的无聊题，遍历过程中判断是不是大写字母，是的话转为小写，就这么简单

#[test]
fn test_my_solution() {
    assert_eq!("apple", my_solution("Apple".to_string()));
}

#[cfg(test)]
fn my_solution(str: String) -> String {
    let mut new_string = String::new();
    for char in str.chars() {
        // Rust又不能像C语言那样很方便的将char类型的大写字母+32或位运算得到小写字母
        match char {
            'A' => new_string.push('a'),
            'B' => new_string.push('b'),
            'C' => new_string.push('c'),
            'D' => new_string.push('d'),
            'E' => new_string.push('e'),
            'F' => new_string.push('f'),
            'G' => new_string.push('g'),
            'H' => new_string.push('h'),
            'I' => new_string.push('i'),
            'J' => new_string.push('j'),
            'K' => new_string.push('k'),
            'L' => new_string.push('l'),
            'M' => new_string.push('m'),
            'N' => new_string.push('n'),
            'O' => new_string.push('o'),
            'P' => new_string.push('p'),
            'Q' => new_string.push('q'),
            'R' => new_string.push('r'),
            'S' => new_string.push('s'),
            'T' => new_string.push('t'),
            'U' => new_string.push('u'),
            'V' => new_string.push('v'),
            'W' => new_string.push('w'),
            'X' => new_string.push('x'),
            'Y' => new_string.push('y'),
            'Z' => new_string.push('z'),
            _ => new_string.push(char),
        }
    }
    new_string
}

#[cfg(not)]
// 或者遍历中途对每个char做to_ascii_lowercase的操作
fn using_standard_api(str: String) -> String {
    // str.as_str().to_lowercase()
    (&str).to_lowercase()
}
