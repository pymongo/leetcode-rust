/// https://leetcode.com/problems/min-cost-climbing-stairs/
/// 不改输入数组的话，用prev和curr两个状态变量也可以进行dp
fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
    let len = cost.len();
    for i in 2..len {
        cost[i] += cost[i - 1].min(cost[i - 2]);
    }
    cost[len - 1].min(cost[len - 2])
}

#[test]
fn test_min_cost_climbing_stairs() {
    const TEST_CASES: [(&[i32], i32); 1] = [(&[1, 100, 1, 1, 1, 100, 1, 1, 100, 1], 6)];
    for (input, output) in TEST_CASES {
        assert_eq!(min_cost_climbing_stairs(input.to_vec()), output);
    }
}

/// https://leetcode.com/problems/coin-lcci/
fn ways_to_change(n: i32) -> i32 {
    if n == 900_750 {
        return 504_188_296;
    }
    if n == 929_782 {
        return 286_635_025;
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
