//! https://leetcode-cn.com/problems/implement-strstr/
//! 实现C语言自带的strstr()以及Java的indexOf() API
//! 功能：查找字符串中的一个子串的出现位置，类似于find()、contains() API

#[cfg(test)]
const TEST_CASE: [(&str, &str, i32); 3] =
    [("hello", "ll", 2), ("aaaaa", "bba", -1), ("bike", "", 0)];

#[test]
fn test_kmp() {
    for case in &TEST_CASE {
        dbg!(case);
        assert_eq!(cheat(case.0.to_owned(), case.1.to_owned()), case.2);
    }
}

#[cfg(test)]
fn cheat(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1,
    }
}

// 尽管最佳解答使用了KMP算法，但是耗时也是4ms，还不如Rust内置的find API
/* KMP算法
相关知识：3. Longest Substring Without Repeating Characters
例如：ABABD中搜索ABD，KMP算法的优势在于发现第一个AB不合条件后，指针立即跳到第二个AB的A上，
有点像双指针滑动窗口遍历最长无重复子串中，遇到重复的子串时立即查表，去偏移尾指针

KMP用到的数据结构很像动态规划的dp数组，实际上是dfa(确定有限状态机)
状态转移图可以看这篇题解：
https://leetcode-cn.com/problems/implement-strstr/solution/kmp-suan-fa-xiang-jie-by-labuladong/
*/
#[cfg(not)]
pub fn global_best_api(haystack: String, needle: String) -> i32 {
    let haystack = haystack.into_bytes();
    let needle = needle.into_bytes();
    let prefix = Self::prefix_arr(&needle);
    println!("{:?}", &prefix);

    let mut needle_p = 0;
    let mut haystack_p = 0;

    loop {
        println!("needle: {}", needle_p);
        if needle_p >= needle.len() {
            return haystack_p as i32 - needle.len() as i32;
        }

        if haystack_p >= haystack.len() {
            break;
        }

        if needle[needle_p] == haystack[haystack_p] {
            needle_p += 1;
            haystack_p += 1;
            continue;
        }

        if needle_p == 0 {
            haystack_p += 1;
            continue;
        }

        needle_p = prefix[needle_p - 1];
    }

    -1
}

/*
这个数组应该叫dp或dfa，叫pattern或部分匹配表也行
"部分匹配值"就是"前缀"和"后缀"的最长的共有元素的长度。以"ABCDABD"为例，
 - "A"的前缀和后缀都为空集，共有元素的长度为0；
 - "AB"的前缀为[A]，后缀为[B]，共有元素的长度为0；
 - "ABC"的前缀为[A, AB]，后缀为[BC, C]，共有元素的长度0；
 - "ABCD"的前缀为[A, AB, ABC]，后缀为[BCD, CD, D]，共有元素的长度为0；
 - "ABCDA"的前缀为[A, AB, ABC, ABCD]，后缀为[BCDA, CDA, DA, A]，共有元素为"A"，长度为1；
 - "ABCDAB"的前缀为[A, AB, ABC, ABCD, ABCDA]，后缀为[BCDAB, CDAB, DAB, AB, B]，共有元素为"AB"，长度为2；
 - "ABCDABD"的前缀为[A, AB, ABC, ABCD, ABCDA, ABCDAB]，后缀为[BCDABD, CDABD, DABD, ABD, BD, D]，共有元素的长度为0。
所以最终的部分匹配表是
A B C D A B D
0 0 0 0 1 2 0

发现不匹配时的移动位数 = 已匹配的字符数 - 最后一个已匹配字符对应的部分匹配值
例如 haystack=ABCDABE, needle=ABCDABC
匹配到E不满足时，会前移6-2(B)个位置，有点像双指针最长无重复子串的尾指针前移的情况
*/
#[cfg(not)]
pub fn prefix_arr(needle: &[u8]) -> Vec<usize> {
    let mut p = 0;
    let mut res = vec![0; needle.len()];
    for i in 1..needle.len() {
        if needle[i] == needle[p] {
            p += 1;
        } else if needle[0] != needle[i] {
            p = 0;
        }
        res[i] = p;
    }
    res
}
