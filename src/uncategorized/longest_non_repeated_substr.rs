/** https://leetcode.com/problems/longest-substring-without-repeating-characters/
## 暴力遍历
两层for循环，穷举出所有可能的字符串逐个进行检查，通过HashSet等数据结构存储已出现过(seen)的字符串来提升性能

## sliding_window
通过左右两个浮标，按右浮标遍历String时发现重复的字符串时，让左浮标往右移到重复字符串的位置，从而缩小搜索范围
时间复杂度是O(N)，最坏的情况O(2N)左右浮标分别完整的遍历一次字符串

## (数据结构)ascii记录字符出现位置
leetcode大部分输入字符串的题全是小写字母，用ASCII表的数组性能比HashMap要好
比HashMap<char, Integer>更高效，用一个长度为128的数组，索引是ascii字母值，value是在字符串中的索引的存储结构效率最高

```python
def length_of_longest_substring(s: str) -> int:
    size = len(s)
    left = 0
    max_len = 0
    table = [-1 for _ in range(128)]

    for right in range(size):
        ord_right = ord(s[right])
        if table[ord_right] != -1:
            # 避免重复的字符「不在当前的移动窗口中」
            # max能确保左指针只会向前移动
            # 例如abba的用例，当right移到第二个a时，left在第二个b，此时虽有重复但是left不能往左移到第一个a，会导致计算的最大长度变大
            left = max(left, table[ord_right] + 1)
        table[ord_right] = right
        temp_max = right - left + 1
        if temp_max > max_len:
            max_len = temp_max
    return max_len
```
*/

fn sliding_window_ascii(s: String) -> i32 {
    let s = s.into_bytes();
    let len: i32 = s.len() as i32;
    if len <= 1 {
        return len;
    }
    // index: 索引表示 某小写字母 的ASCII值, value: 从窗函数的右边界往左看第一个 某小写字母的出现索引
    let mut ascii_char_occur_index = [-1_i32; 128];
    let (mut left, mut right, mut max_len) = (0_i32, 0_i32, 0_i32);
    while right < len {
        let current_char = s[right as usize];
        if ascii_char_occur_index[current_char as usize] != -1 {
            /*
            例如abba的用例，end=2时(第二个b)，start会跳到2
            slider.1指到3时(最后一个a)，第二次出现重复时，重复的是a，ascii_table中字母a的索引是0
            如果不进行判断start会后退到0+1
            max() prevent sliders.0's index go back (test case: abba)
            避免重复的字符「不在当前的移动窗口中」
            */
            left = left.max(ascii_char_occur_index[current_char as usize] + 1);
        }
        ascii_char_occur_index[current_char as usize] = right;
        max_len = max_len.max(right - left);
        right += 1;
    }
    max_len + 1
}

/// 即便用了性能不如ASCII数组的HashMap，依然是0ms的解法
fn sliding_window_hashmap(s: String) -> i32 {
    let s = s.into_bytes();
    let mut max_len = 0;
    let mut start = 0_usize;
    let mut map = std::collections::HashMap::new();
    for (i, letter) in s.into_iter().enumerate() {
        if let Some(&left_index) = map.get(&letter) {
            start = start.max(left_index + 1);
        }
        // 必须要更新完左边界再去算window长度才是正确的
        max_len = max_len.max(i - start + 1);
        // insert or update
        map.insert(letter, i);
    }
    max_len as i32
}

const TEST_CASES: [(&str, i32); 7] = [
    ("dvdf", 3),
    ("abcabcbb", 3),
    ("bbbbb", 1),
    ("pwwkew", 3),
    ("abcabcbb", 3),
    ("abba", 2),
    (" ", 1),
];

#[test]
fn test_i32_ascii_table() {
    for (input, expected) in TEST_CASES {
        assert_eq!(sliding_window_ascii(input.to_string()), expected);
    }
}

#[test]
fn test_sliding_window_hashmap() {
    for (input, expected) in TEST_CASES {
        assert_eq!(sliding_window_hashmap(input.to_string()), expected);
    }
}

#[test]
fn aa() {
    let a = 1;
    dbg!(a);
}
