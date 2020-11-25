/*! https://leetcode.com/problems/freedom-trail/
有一台老式转盘拨号的电话机(rotary telephone)
给定一个密码key，需要求出转盘旋转次数最少拨出key的步数
最初，ring环的第一个字母指向12点钟方向
*/

/** 从后往前推导的dp思路(状态转移层未经优化)
初始值: 最后一步一定是 转盘ring对着 key的最后一个字母key[m]
最终值: dp[0][0]加上对准了长度为m的key之后，key的m个字母各需要一次拨号，所以最终步数为dp[0][0]+m
从dp[m-1]一直推到dp[0]，这样`dp[0][0]+m`就是所需结果
求dp[m-1]表示求倒数第二步dp[m-1][j]的步数，j表示穷举0..n(ring的所有位置j)，直到找到一个转盘上的位置k使得转盘字母ring[k]=倒数第一步所需字母key[m]
然后分别计算出转盘从j正转到k和从j反转的k的步数的最小值
对于倒数第二步来说，状态转移方程就是: dp[i][j]=dp[i][j(ring[k]=key[i+1])].min(dp[i+1][k]+j到k正转或反转的最小步数)
*/
fn find_rotate_steps(ring: String, key: String) -> i32 {
    let (ring, key) = (ring.into_bytes(), key.into_bytes());
    let n = ring.len();
    let m = key.len();

    // 解释m为什么要+1，因为m表示key的下标值
    let mut dp = vec![vec![std::usize::MAX; n]; m + 1];
    // 因为状态转移方程/dp填表依赖当前行的下一行的值，初始填表时倒数第二行，所以除最后一行置零以外全部初始化为最大值
    for j in 0..n {
        dp[m][j] = 0;
    }

    for i in (0..m).rev() {
        for j in 0..n {
            // dp[i][j]表示转盘已经对完key的前i个字母后，穷举转盘12点钟方向可能指向的任意位置j
            // 我们可以假设转盘对完上个字母a(key[i])后，可以调皮的随意多移动到字母b，反正并不是每步都要最优解总体才是最优解
            // i: 转盘下一步要拨到key[i]的字母
            // j: 枚举转盘当前步的所有位置
            // k: 枚举转盘可以达到下一步key[i]字母的目标位置
            for k in 0..n {
                // 可以参考leetcode官方题解，优化状态转移层，用类似Python的collections.Counter去统计每个小写字母在ring中出现在哪些索引位置
                // for &k in ring_counter.get(&key[i]).unwrap()，由于ring全为小写字母，所以Counter用数组会比Hashmap性能更好
                if ring[k] == key[i] {
                    // 从穷举的当前位置j，到转盘其中一个字母等于key[i]的位置k，所需要的正转/反转的步数
                    let diff = if j < k { k - j } else { j - k };
                    // 选正转或反转的步数最小值
                    let step = diff.min(n - diff);
                    // 状态转移方程: dp[i][j]=dp[i][j(ring[k]=key[i+1])].min(dp[i+1][k]+j到k正转或反转的最小步数)
                    dp[i][j] = dp[i][j].min(dp[i + 1][k] + step);
                }
            }
        }
    }

    // 解释+m: 转盘对准了key的m个字母后，key的m个字母各需要一个步数去拨号，所以要+m
    (dp[0][0] + m) as i32
}

fn find_rotate_steps_optimized(ring: String, key: String) -> i32 {
    let (ring, key) = (ring.into_bytes(), key.into_bytes());
    let n = ring.len();
    let m = key.len();

    // 由于ring中全为小写字母，所以用ASCII数组会比Hashmap性能好
    let mut ring_counter = vec![Vec::new(); 26];
    // the trait `Copy` is not implemented for `Vec<usize>`
    // let mut ring_counter = [Vec::with_capacity(0); 26];
    for (i, ring_char) in ring.into_iter().enumerate() {
        ring_counter[(ring_char - b'a') as usize].push(i);
    }
    // 解释m为什么要+1，因为m表示key的下标值
    let mut dp = vec![vec![std::usize::MAX; n]; m + 1];
    // 因为状态转移方程/dp填表依赖当前行的下一行的值，初始填表时倒数第二行，所以除最后一行置零以外全部初始化为最大值
    for j in 0..n {
        dp[m][j] = 0;
    }

    for i in (0..m).rev() {
        for j in 0..n {
            // dp[i][j]表示转盘已经对完key的前i个字母后，穷举转盘12点钟方向可能指向的任意位置j
            // i: 转盘下一步要拨到key[i]的字母
            // j: 枚举转盘当前步的所有位置
            // k: 枚举转盘可以达到下一步key[i]字母的目标位置
            for &k in ring_counter[(key[i] - b'a') as usize].iter() {
                let diff = if j < k { k - j } else { j - k };
                // 选正转或反转的步数最小值
                let step = diff.min(n - diff);
                dp[i][j] = dp[i][j].min(step + dp[i + 1][k]);
            }
        }
    }

    // 解释+m: 转盘对准了key的m个字母后，key的m个字母各需要一个步数去拨号，所以要+m
    (dp[0][0] + m) as i32
}

#[cfg(test)]
const TEST_CASES: [(&str, &str, i32); 1] = [("godding", "gd", 4)];

#[test]
fn test_find_rotate_steps() {
    for &(ring, key, steps) in TEST_CASES.iter() {
        assert_eq!(find_rotate_steps(ring.to_owned(), key.to_owned()), steps);
        assert_eq!(
            find_rotate_steps_optimized(ring.to_owned(), key.to_owned()),
            steps
        );
    }
}
