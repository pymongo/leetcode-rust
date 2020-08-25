use std::collections::HashMap;

struct Solution;

impl Solution {
    fn eggs_drop(k: i32, n: i32) -> i32 {
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
        return dfs(k, n, &mut memo);
    }
}

/*
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
*/
// TODO 这个解法只能在lintcode上AC，leetcode会超时
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

    let mut res = std::i32::MAX;
    // 穷举在第i层扔下鸡蛋后可能的情况(没碎或碎)
    for i in 1..=n {
        // dp(k  , n-i): 鸡蛋没碎，那么刚扔下的鸡蛋还可以继续用从i+1..=n层的范围搜索，但是还是有k次机会
        // dp(k-1, i-1): 鸡蛋碎了，只好拿k-1个鸡蛋去试1..=i-1层
        // 最后不管碎或不碎，尝试次数都+1
        res = res.min(std::cmp::max(
            dfs(k, n - i, memo),
            dfs(k - 1, i - 1, memo) + 1,
        ));
    }

    // FIXME 为什么n=5000时，加上print语句就会爆栈
    // println!("k={}, n={}, res={}", k, n, k);
    // dbg!(k, n, res);
    memo.insert((k, n), res);

    return res;
}

const TEST_CASES: [(i32, i32, i32); 2] = [
    // 1+2+..+n=10 -> n(n+1)/2=10 -> n=times=4
    (2, 10, 4),
    (4, 5000, 19),
];

#[test]
fn test() {
    for &(eggs_k, n, times) in TEST_CASES.iter() {
        assert_eq!(Solution::eggs_drop(eggs_k, n), times);
    }
}
