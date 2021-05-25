use std::collections::HashMap;

fn eggs_drop(k: i32, n: i32) -> i32 {
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    dfs(k, n, &mut memo)
}

fn dp_binary_search(k: i32, n: i32) -> i32 {
    let (k, n) = (k as usize, n as usize);
    // dp[i][j]: i层楼有j个鸡蛋的最小尝试次数
    let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX; k + 1]; n + 1];
    // 楼层为0时，第0行: 不管鸡蛋的个数多少，都测不出鸡蛋的耐摔层度，所以次数全为0
    for j in 0..=k {
        dp[0][j] = 0;
    }
    // 楼层为1时，只要鸡蛋个数大于1就只用试1次
    for j in 1..=k {
        dp[1][j] = 1;
    }
    // 鸡蛋个数为0时尝试次数只能为0
    // 鸡蛋个数为1时尝试次数就是楼层高度
    for (i, dp_row) in dp.iter_mut().take(n + 1).enumerate() {
        dp_row[0] = 0;
        dp_row[1] = i;
    }

    // 由于本题类似答案集二分的题型，鸡蛋个数固定时，尝试次数和楼层高度成正比，所以可以在DP决策层用「二分」
    for i in 2..=n {
        for j in 2..=k {
            let (mut left, mut right) = (1, i);
            while left + 1 < right {
                // 注意要用find_first的二分法模板
                let mid = left + (right - left) / 2;

                let broken = dp[mid - 1][j - 1];
                let not_broken = dp[i - mid][j];
                if broken > not_broken {
                    right = mid;
                } else {
                    left = mid;
                }
            }
            dp[i][j] = 1 + std::cmp::max(dp[left - 1][j - 1], dp[i - left][j]);
        }
    }
    dp[n][k] as i32
}

/**
```text
def dp(k, n):
    if k == 1:
        return n
    if n == 0:
        return 0
    if (k, n) in memo:
        return memo[(k, n)]

    res = float('inf')
    for i in range(1, n+1):
        res = min(res, max(
            dp(k, n-i),
            dp(k-1, i-1)
        )+1)

    memo[(k, n)] = res
    return res
```
*/
// TODO 这个解法只能在lintcode上AC，leetcode会超时(因为没有用二分群优化时间复杂度)
fn dfs(k: i32, n: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    // 如果只有一个鸡蛋，那么肯定要试k次
    if k == 1 {
        return n;
    }
    // 如果只有0层楼，那就不需要扔鸡蛋🥚了
    if n == 0 {
        return 0;
    }
    if let Some(val) = memo.get(&(k, n)) {
        return *val;
    }

    let mut res = i32::MAX;
    // 穷举在第i层扔下鸡蛋后可能的情况(没碎或碎)
    for i in 1..=n {
        // dp(k  , n-i): 鸡蛋没碎，那么刚扔下的鸡蛋还可以继续用从i+1..=n层的范围搜索，但是还是有k次机会
        // dp(k-1, i-1): 鸡蛋碎了，只好拿k-1个鸡蛋去试1..=i-1层
        // 最后不管碎或不碎，尝试次数都+1
        res = res.min(1 + std::cmp::max(dfs(k, n - i, memo), dfs(k - 1, i - 1, memo)));
    }

    // FIXME 为什么n=5000时，加上print语句就会爆栈
    // println!("k={}, n={}, res={}", k, n, k);
    // dbg!(k, n, res);
    memo.insert((k, n), res);
    res
}

const TEST_CASES: [(i32, i32, i32); 2] = [
    // 1+2+..+n=10 -> n(n+1)/2=10 -> n=times=4
    (2, 10, 4),
    (4, 5000, 19),
];

#[test]
fn test() {
    for (eggs_k, n, times) in TEST_CASES {
        assert_eq!(dp_binary_search(eggs_k, n), times);
    }
}
