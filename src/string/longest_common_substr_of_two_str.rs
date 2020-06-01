/// ## 两个字符串之间的最长公共子串
/// 本算法不是leetcode的题，只是leetcode第五题的最长公共子串的解法的基础
/// DP思路：如果s1[x]==s2[y]且s1[x-1]==s2[y-1]，说明当前子串是公共子串
/// 一个字符串与其倒序的公共字符串很可能是最长回文子串，如果最长公共子串的最后一位等于第一位，那么它就是回文串
/// 很新颖的方法，不过我又不能用严谨的数学证明它是对的，不推荐用，了解下即可

// #[cfg(test)]
// const TEST_CASES: [(&str, &str, &str); 1] = [
//     ("caba", "abac", "aba"),
// ];

#[cfg(test)]
fn dp(s1: String, s2: String) -> usize {
    let (str1, str2) = (s1.as_bytes(), s2.as_bytes());
    let (str1_len, str2_len) = (str1.len(), str2.len());
    let mut max: usize = 0;
    let mut max_end_index: usize = 0;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; str1_len]; str2_len];
    for x in 0..str1_len {
        for y in 0..str2_len {
            if str1[x] == str2[y] {
                if x == 0 || y == 0 {
                    dp[x][y] = 1;
                } else {
                    // 遍历第N行时只需用到N-1行的数据
                    // 所以用两个1*N的数组即可，将空间复杂度从O(n^2)降低到O(2n)
                    dp[x][y] = dp[x - 1][y - 1] + 1;
                }
                if dp[x][y] > max {
                    max = dp[x][y];
                    max_end_index = x;
                }
            }
        }
    }
    dbg!(dp);
    dbg!(String::from_utf8_lossy(
        &str1[(max_end_index - max + 1)..=max_end_index]
    ));
    max
}

#[test]
fn test() {
    dp("caba".to_owned(), "abac".to_owned());
}
