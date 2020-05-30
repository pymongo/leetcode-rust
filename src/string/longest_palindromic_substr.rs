//! 本题多达五种解法：
//! 1. O(n), Manacher:
//! 2. O(n^2), dp:

#[cfg(test)]
const TEST_CASES: [(&str, &str); 4] = [
    ("aba", "aba"),
    ("ac", "a"),
    ("abadd", "aba"),
    ("ccc", "ccc"),
];

#[test]
fn test_dp() {
    for case in &TEST_CASES {
        assert_eq!(dp(case.0.to_owned()), case.1.to_owned());
    }
}

/*
动态规划(数学归纳法)的解法：
递推/状态转移方程：如果a[0]==a[-1]，而且a[1..-2]是个回文数，则a也是个回文数
因为需要原顺序与逆序进行比较，所以可以列出 横坐标是start 纵坐标是end 的表
以 "cbba" 为例，列出一个bool的二维数组：
dp初始值：a[k]==a[k]时为true，表示是回文数
\对角线是dp的初始值，因为竖着的end必须比start大，所以只需遍历左下三角
  c b b a
c T
b ? T
b   % T
a       T
(1)案例分析1 (?处)
例如我想知道end=2, start=0构成的子串cb(如图问号处)是不是回文
先判断a[start] == a[end]，因为不满足条件所以不是，?处填上F
(2)案例分析2 (%处)
%是满足a[start]==a[end]，再看看%的上一项是在%的右上方，属于右上角区域，所以是空字符串也满足回文条件
所以%也是个回文数
因此我们遍历时需要从右边往左，从上到下才能保证每次取右上时能有值
(3)遍历
len = 4;
初始：start=len-2, end=len-1遍历到start+1
start=2, end=3->3;
start=1, end=3->2;
改良：
写完后我才发现start作为纵坐标更合适，刚好能让二位数组的index变为s[start][end]
*/
// 性能：耗时28ms，比暴力破解的650多毫秒强多了👍
#[cfg(test)]
fn dp(s: String) -> String {
    let len = s.len();
    if len <= 1 {
        // 应对极端情况会导致我数组subtract with overflow
        return s;
    }
    // 判断逆序后是否相等：应对ccc的测试用例
    if s.chars().rev().collect::<String>() == s {
        // 这个dp该怎么处理ccc的情况呢？无解啊，遍历就是从len-1开始
        // 第二种办法是判断table结束后是否仍为全true
        return s;
    }


    let bytes = s.as_bytes();
    // Rust的数组只能使用Const来定义长度，不能用s.len
    // let mut table: [[bool; s.len()]; s.len()] = [[false; s.len()]; s.len()];
    let mut table = vec![vec![true; s.len()]; s.len()];
    // 初始化_先把table对角线右上部分元素设为true(表示该项是回文数)
    // for i in 0..len {
    //   table[i][i] = true;
    // }
    let mut max_len = 0;
    let mut best_start = 0;
    let mut best_end = len - 1;
    let mut start = len - 2;
    let mut end;
    loop {
        end = len - 1;
        loop {
            // println!("end = {}, start = {}", end, start);
            if bytes[start] == bytes[end]
                && table[end - 1][start + 1] {
                table[end][start] = true;
                if end - start > max_len {
                    best_start = start;
                    best_end = end;
                    max_len = end - start;
                }
            } else {
                table[end][start] = false;
            }
            end -= 1;
            if end <= start {
                break;
            }
        }
        if start == 0 {
            break;
        } else {
            start -= 1;
        }
    }
    if max_len == 0 {
        best_end = 0;
    }
    s[best_start..=best_end].parse().unwrap()
}

// 国服第一0ms的解答，Manacher是唯一能实现O(n)时间复杂度的算法
#[cfg(not)]
pub fn longest_palindrome_manacher(s: String) -> String {
    /* Manacher Algorithm
     * step1: add '$''#' into string
     * The character `$` here is just to prevent overbounds
     * there is an even palindrome `abba` and an odd palindrome `opxpo` in `s="abbahopxpo"`,
     * which are converted into `#a#b#b#a#` and `#o#p#x#p#o#`, and the length is converted into odd
     */
    let mut new_str = vec!['$', '#'];
    for ch in s.chars() {
        new_str.push(ch);
        new_str.push('#');
    }
    new_str.push('\0');

    // length of the new string
    let len = new_str.len();

    // Define a secondary array p[], where p[i] represents the radius of the longest palindrome centered on i.
    let mut p = vec![0usize; len];

    // `max_len`: The length of the longest palindrome string in the original string
    let mut max_len = 0usize;

    // Define two variables, `mx` and` id`
    // `mx` represents the right boundary of the longest palindrome centered on` id`, which is `mx = id + p[id]`
    let mut id = 0usize;
    let mut mx = 0usize;
    for i in 1..(len - 1) {
        if i < mx {
            p[i] = p[2 * id - i].min(mx - i);
        } else {
            p[i] = 1;
        }

        while new_str[i - p[i]] == new_str[i + p[i]] {
            p[i] += 1;
        }

        if mx < i + p[i] {
            id = i;
            mx = i + p[i];
        }
        // `p[i] - 1` is exactly the length of the longest palindrome string in the original string
        max_len = max_len.max(p[i] - 1);
    }

    /* Get longest palindromic substring
     * left: left boundery of the longest palindromic substring
     * right: right boundery of the longest palindromic substring
     */
    let left = p.iter().position(|&x| x == (max_len + 1)).unwrap() - max_len + 1;
    let right = left + max_len * 2;
    let mut longest_palindrome_substring = String::from("");
    for i in left..right {
        if new_str[i] != '#' {
            longest_palindrome_substring.push(new_str[i]);
        }
    }

    /* Return longest palindromic substring */
    longest_palindrome_substring
}


// 全球服第一的答案，似乎并不是Manacher算法
#[cfg(not)]
fn longest_palindrome_global_best(s: String) -> String {
    let seq: Vec<char> = s.chars().collect();
    let len = seq.len();
    if len < 1 { return s; }
    let (mut idx, mut curr_len, mut curr_start, mut curr_end) = (0, 0, 0, 0);
    while idx < len {
        let (mut i, mut j) = (idx, idx);
        let ch = seq[idx];
        // handle same char
        while i > 0 && seq[i - 1] == ch { i -= 1 };
        while j < len - 1 && seq[j + 1] == ch { j += 1 };
        idx = j + 1;
        while i > 0 && j < len - 1 && seq[i - 1] == seq[j + 1] {
            i -= 1;
            j += 1;
        }
        let max_len = j - i + 1;
        if max_len > curr_len {
            curr_len = max_len;
            curr_start = i;
            curr_end = j;
        }
        if max_len >= len - 1 {
            break;
        }
    }

    s[curr_start..curr_end + 1].to_owned()
}

/*
执行用时: 648 ms, 在所有 Rust 提交中击败了5.69%的用户
内存消耗: 2 MB, 在所有 Rust 提交中击败了66.67%的用户
时间复杂度，O(n^3)++
*/
#[cfg(not)]
fn my_brute_force(s: String) -> String {
    let len = s.len();
    if len <= 1 {
        return s;
    }
    let sub_string = &mut String::new();
    let mut result = String::new();
    let mut max_len = 0;

    for i in 0..len {
        for j in i..len {
            // dbg!((i,j));
            if (j - i) < max_len {
                continue;
            }
            *sub_string = (&s[i..=j]).parse().unwrap();
            if is_palindromic(sub_string) {
                if sub_string.len() > max_len {
                    result = sub_string.clone();
                    max_len = sub_string.len();
                }
            }
        }
    }
    result
}

#[cfg(not)]
fn my_brute_force_is_palindromic(s: &String) -> bool {
    let bytes = (*s).as_bytes();
    let mut start: usize = 0;
    let mut end: usize = bytes.len() - 1;
    while start < end {
        if bytes[start] != bytes[end] {
            return false;
        } else {
            start += 1;
            end -= 1;
        }
    }
    true
}

// 这老哥用paris存储start/end的组合也是挺有意思的，不过我还是习惯使用left/right
#[cfg(not)]
fn longest_palindrome_best_brute_force(s: String) -> String {
    if s.as_str() { return s; }
    let s: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut pair = (0, 0);
    while i < (s.len() - 1) {
        let mut j;
        let mut k;
        if s[i] == s[i + 1] {
            j = i;
            k = i + 1;
            while j > 0 && k < s.len() - 1 && s[j] == s[k] {
                j -= 1;
                k += 1;
            }
            if s[j] != s[k] {
                j += 1;
                k -= 1;
            }
            if k - j > pair.1 - pair.0 {
                pair.0 = j;
                pair.1 = k;
            }
        }
        if i > 0 && s[i - 1] == s[i + 1] {
            j = i - 1;
            k = i + 1;
            while j > 0 && k < s.len() - 1 && s[j] == s[k] {
                j -= 1;
                k += 1;
            }
            if s[j] != s[k] {
                j += 1;
                k -= 1;
            }
            if k - j > pair.1 - pair.0 {
                pair.0 = j;
                pair.1 = k;
            }
        }
        i += 1;
    }
    s[pair.0..pair.1 + 1].iter().collect()
}
