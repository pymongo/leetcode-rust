/*!
完全背包问题: 一个物品可以选一次或多次
零钱兑换(coin-change)也属于完全背包问题，类似题型:
- perfect_squares: 硬币可重复，求凑出target_value的「最少」硬币数
- 零钱兑换-i  :  硬币可重复，求凑出target_value的「最少」硬币数
- 零钱兑换-ii :  硬币可重复，求凑出target_value的硬币「方案数」
    方案数是number of combinations, (1,2)和(2,1)被认为是同一个方案
    所以要「先遍历coin」再遍历value
- combination-sum-iv :  硬币可重复，求凑出target_value的硬币「方案数」
    方案数是number of combinations, (1,2)和(2,1)被认为是「不同」方案
    所以要「先遍历value」再遍历coin
*/

/// https://leetcode.com/problems/coin-change/
/// 外层是`for value`，内层是`for coin`
fn coin_change_i_dp_1(coins: Vec<i32>, amount: i32) -> i32 {
    let coins = coins
        .into_iter()
        .map(|coin| coin as usize)
        .collect::<Vec<_>>();
    let target_value = amount as usize;
    let mut dp = vec![None; target_value + 1];
    dp[0] = Some(0);
    for value in 1..=target_value {
        for &coin in &coins {
            if coin <= value {
                if let Some(dp_value_minus_coin) = dp[value - coin] {
                    if let Some(ref mut dp_value) = dp[value] {
                        *dp_value = (*dp_value).min(dp_value_minus_coin + 1);
                    } else {
                        dp[value] = Some(dp_value_minus_coin + 1);
                    }
                }
            }
        }
    }
    dp[target_value].unwrap_or(-1)
}

/// BestSolution: 外层是`for coin`，内层是`for value`
fn coin_change_i_dp_2(coins: Vec<i32>, amount: i32) -> i32 {
    let target_value = amount as usize;
    let mut dp = vec![i32::from(i16::MAX); target_value + 1];
    dp[0] = 0;
    for coin in coins {
        let coin = coin as usize;
        for value in coin..=target_value {
            dp[value] = dp[value].min(dp[value - coin] + 1);
        }
    }
    if dp[target_value] == i32::from(i16::MAX) {
        return -1;
    }
    dp[target_value]
}

/// coin can repreat, return min coins total value equal amount
fn coin_change_i_dfs_tle(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = CoinChangeDfsState { min_coins: None };
    dp.dfs(0, amount, &coins);
    dp.min_coins.unwrap_or(-1)
}

/// Time Limit Exceeded
struct CoinChangeDfsState {
    min_coins: Option<i32>,
}

impl CoinChangeDfsState {
    fn dfs(&mut self, used_coins: i32, target_value: i32, coins: &[i32]) {
        if target_value < 0 {
            return;
        }
        if target_value == 0 {
            if let Some(min_coins) = self.min_coins {
                self.min_coins = Some(min_coins.min(used_coins));
            } else {
                self.min_coins = Some(used_coins);
            }
            return;
        }
        for &coin in coins {
            self.dfs(used_coins + 1, target_value - coin, coins);
        }
    }
}

#[test]
fn test_coin_change_i() {
    const TEST_CASES: [(&[i32], i32, i32); 4] = [
        (&[1, 2, 5], 11, 3),
        (&[1, 0x7FFF_FFFF], 2, 2),
        (&[1, 2, 5], 100, 20),
        (&[2], 3, -1),
    ];
    for (coins, amount, min_coins) in TEST_CASES {
        //assert_eq!(coin_change_dfs_tle(coins.to_owned(), amount), min_coins);
        assert_eq!(coin_change_i_dp_1(coins.to_owned(), amount), min_coins);
        assert_eq!(coin_change_i_dp_2(coins.to_owned(), amount), min_coins);
    }
}

/// https://leetcode.com/problems/coin-change-2/
/// (1,2)和(2,1)被认为是同一种方案，所以只能外层遍历coins能让方案总数较少
fn coin_change_ii(amount: i32, coins: Vec<i32>) -> i32 {
    let target_value = amount as usize;
    let mut dp = vec![0; target_value + 1];
    dp[0] = 1;
    for coin in coins {
        let coin = coin as usize;
        for value in coin..=target_value {
            dp[value] += dp[value - coin];
        }
    }
    dp[target_value]
}

/// https://leetcode.com/problems/perfect-squares/
/// least number of square numbers that sum to n
fn perfect_squares(n: i32) -> i32 {
    let target_value = n as usize;
    let mut dp = vec![i32::from(i16::MAX); target_value + 1];
    dp[0] = 0;
    let max_coin = f64::from(n).sqrt() as usize;
    for coin in 1..=max_coin {
        let coin = coin * coin;
        for value in coin..=target_value {
            dp[value] = dp[value].min(dp[value - coin] + 1);
        }
    }
    if dp[target_value] == i32::from(i16::MAX) {
        return 0;
    }
    dp[target_value]
}

#[test]
fn test_perfect_squares() {
    const TEST_CASES: [(i32, i32); 2] = [
        (12, 3), // 4+4+4
        (13, 2), // 4+9
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(perfect_squares(input), output);
    }
}

/// https://leetcode.com/problems/combination-sum-iv/
/// (1,2)和(2,1)被认为是同一种方案，所以只能外层遍历value能让方案总数尽可能多
/// 不能解决nums中有负数的情况(硬币面值为负数)
fn combination_sum_iv(nums: Vec<i32>, target: i32) -> i32 {
    let coins = nums.into_iter().map(|num| num as usize).collect::<Vec<_>>();
    let target_value = target as usize;
    let mut dp = vec![0; target_value + 1];
    dp[0] = 1;
    for value in 1..=target_value {
        for &coin in &coins {
            if coin <= value {
                dp[value] += dp[value - coin];
            }
        }
    }
    dp[target_value]
}

#[test]
fn test_combination_sum_iv() {
    const TEST_CASES: [(&[i32], i32, i32); 2] = [
        (&[1, 2, 3], 4, 7),
        (&[9], 3, 0),
    ];
    for (nums, target, min_coins) in TEST_CASES {
        assert_eq!(combination_sum_iv(nums.to_owned(), target), min_coins);
    }
}
