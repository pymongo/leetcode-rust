/// https://leetcode.com/problems/burst-balloons/
fn burst_balloons(mut nums: Vec<i32>) -> i32 {
    nums.insert(0, 1);
    nums.push(1);
    let n = nums.len();
    // 初始值和从区间长度3开始遍历恰好能让区间长度2以内的得分全为0，使得dp计算区间长度为3的得分时为 0+戳中间气球得分+0
    // dp[i][j]表示区间[i,j]的最大得分，只有当区间长度大于3时才能有得分
    let mut dp = vec![vec![0; n]; n];

    // 由于区间长度至少为3，而且大区间的最大得分依赖其包含的更小区间，所以区间的起点i要从位置n-2开始倒序枚举
    // 也可以枚举区间长度进行遍历
    for i in (0..=n - 2).rev() {
        for j in i + 2..n {
            // k的含义是: 先戳爆i到k之间的所有气球，再戳爆k到j之间所有气球，最后剩余i,k,j三个气球相连
            for k in i + 1..j {
                dp[i][j] = dp[i][j].max(dp[i][k] + dp[k][j] + nums[i] * nums[k] * nums[j]);
                println!("i={},k={},j={}", i, k, j);
                for row in &dp {
                    eprintln!("row = {:?}", row);
                }
            }
        }
    }
    for row in &dp {
        eprintln!("row = {:?}", row);
    }
    dp[0][n - 1]
}

#[test]
fn test_burst_balloons() {
    const TEST_CASES: [(&[i32], i32); 1] = [
        // 可以想象数组左右两边各有一个隐藏的1，吹爆1得分3*1*5=15，吹爆3得分(1*)3*5，所以总分是15+15+5
        (&[3, 1, 5], 35),
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(burst_balloons(input.into()), output);
    }
}
