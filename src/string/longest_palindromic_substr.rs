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
