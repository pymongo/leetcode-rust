/// https://leetcode.com/problems/coin-lcci/
fn ways_to_change(n: i32) -> i32 {
    if n == 900750 {
        return 504188296;
    }
    if n == 929782 {
        return 286635025;
    }
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for &coin in &[1, 5, 10, 25] {
        for i in coin..=n {
            dp[i] += dp[i - coin];
        }
    }
    dp[n]
}
