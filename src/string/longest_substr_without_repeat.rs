// 最长无重复子串 - 字典存索引的解法
#[allow(dead_code)]
fn sliding_window_ascii_table(s: String) -> i32 {
    let len: i32 = s.len() as i32;
    if len <= 1 {
        return len;
    }
    // TODO array的值用usize更好，只不过没有像-1这样显著体现未初始化状态可读性好
    // 如果假定输入字符串最大长度是1000，确实可用1000作未初始化的值
    // index: value of char in ascii
    // value: index of char
    let mut ascii_table: [i32; 128] = [-1; 128];
    let (mut start, mut end, mut max) = (0_i32, 0_i32, 0_i32);
    let mut temp_max: i32;
    let bytes = s.as_bytes();
    let mut current_char: u8;
    while end < len {
        current_char = bytes[end as usize];
        if ascii_table[current_char as usize] != -1 {
            // 例如abba的用例，end=2时(第二个b)，start会跳到2
            // slider.1指到3时(最后一个a)，第二次出现重复时，重复的是a，ascii_table中字母a的索引是0
            // 如果不进行判断start会后退到0+1
            // max() prevent sliders.0's index go back (test case: abba)
            start = std::cmp::max(start, ascii_table[current_char as usize] + 1);
        }
        ascii_table[current_char as usize] = end;
        temp_max = end - start;
        if temp_max > max {
            max = temp_max;
        }
        end += 1;
    }
    max + 1
}

#[cfg(test)]
const TEST_CASES: [(&str, i32); 5] = [
    ("abcabcbb", 3),
    ("bbbbb", 1),
    ("pwwkew", 3),
    ("abcabcbb", 3),
    ("abba", 2),
];

#[test]
fn test_cases() {
    for case in &TEST_CASES {
        assert_eq!(sliding_window_ascii_table(case.0.to_string()), case.1)
    }
}

/*
pub fn sliding_window_ascii_table(s: String) -> i32 {
    let len = s.len();
    if len <= 1 {
        return len as i32;
    }
    let mut dict = [1000; 256];
    let mut sliders = (0, 0);
    let bytes = s.as_bytes();
    let mut max = 0;
    let mut temp_max = 0;
    let mut current_char;
    while sliders.1 < len {
        current_char = bytes[sliders.1] as usize;
        if dict[current_char] != usize::max_value() {
            // 例如abba的用例，slider.1指到2时，slider.0会跳到2
            // slider.1指到3时(最后一个a)，第二次出现重复时，如果不进行判断slider.0会后退到1，导致输出结果变大
            // max() prevent sliders.0's index go back (test case: abba)
            sliders.0 = std::cmp::max(sliders.0, dict[current_char] + 1);
        }
        dict[current_char] = sliders.1;
        temp_max = sliders.1 - sliders.0;
        if temp_max > max {
            max = sliders.1 - sliders.0;
        }
        sliders.1 += 1;
    }
    (max + 1) as i32
}
*/
