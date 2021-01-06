/// ## 两个字符串之间的最长公共子串
/// 本算法不是leetcode的题，只是leetcode第五题的最长公共子串的解法的引申出「如何寻找两个字符串的最长公共部分」d的问题
/// DP思路：如果s1[x]==s2[y]且s1[x-1]==s2[y-1]，说明当前子串是公共子串
#[allow(clippy::needless_range_loop)]
fn longest_common_substr_dp(s1: String, s2: String) -> String {
    let (s1, s2) = (s1.into_bytes(), s2.into_bytes());
    let (str1_len, str2_len) = (s1.len(), s2.len());
    let mut max_len: usize = 0;
    let mut max_end_index: usize = 0;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; str1_len]; str2_len];
    for x in 0..str1_len {
        for y in 0..str2_len {
            if s1[x] == s2[y] {
                if x == 0 || y == 0 {
                    dp[x][y] = 1;
                } else {
                    // 遍历第N行时只需用到N-1行的数据
                    // 所以用两个1*N的数组即可，将空间复杂度从O(n^2)降低到O(2n)
                    dp[x][y] = dp[x - 1][y - 1] + 1;
                }
                if dp[x][y] > max_len {
                    max_len = dp[x][y];
                    max_end_index = x;
                }
            }
        }
    }
    unsafe {
        String::from_utf8_unchecked(s1[(max_end_index - max_len + 1)..=max_end_index].to_vec())
    }
}

#[test]
fn test_longest_common_substr_dp() {
    const TEST_CASES: [(&str, &str, &str); 1] = [("caba", "abac", "aba")];
    for &(input1, input2, expected) in TEST_CASES.iter() {
        assert_eq!(
            longest_common_substr_dp(input1.to_string(), input2.to_string()),
            expected
        );
    }
}
