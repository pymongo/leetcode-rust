//! https://leetcode.com/problems/count-primes/
const TEST_CASES: [(i32, i32); 3] = [(0, 0), (10, 4), (10000, 1229)];

#[test]
fn test_count_primes_brute_force() {
    for &(n, primes_count) in TEST_CASES.iter() {
        assert_eq!(count_primes_brute_force(n), primes_count);
    }
}

/// https://program-think.blogspot.com/2011/12/prime-algorithm-1.html
/// 判断一个数是不是质数，最简单的就是试除法，通过遍历2..sqrt(n)的所有因数，判断n的因数是否只有一个
/// 以下是优化策略
/// 1. 由于因数都是成对出现的，所以最大的较小因数一定<=sqrt(n)，所以整除判断完所有因数对的较小因子就等于判断完所有因子
/// 2. The number 2 is the only even prime number: 如果n是除2以外的偶数，则一定不是质数
/// 3. 根据2进一步推断出，质数的因子一定是奇数，所以可以按3,5,7这样的间隔去穷举较小因子
/// 4. 例如101的较小因子遍历序列是 3,5,7,9 注意到9是没必要试的，因为如果3能被整除，那么9也能被整除
///    能否用DP的思想，用一个数组存储之间判断过的质数，然后只需要枚举 3..sqrt(n) 范围内的质数即可，而判断较大数n的质因子需要前面的计算结果，这就有点dp了
fn count_primes_brute_force(n: i32) -> i32 {
    let n = n as usize;
    // 先假设全是质数，那么判断较小的n是不是质数时，全部因子都得扫一遍
    let mut primes = vec![true; n];
    let mut primes_count = 0; // 2 is prime
    'outer: for num in 3..n {
        // The number 2 is the only even prime number
        if num % 2 == 0 {
            primes[num] = false;
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        let max_lower_factor = (num as f32).sqrt() as usize;
        for lower_factor in (3..=max_lower_factor).step_by(2) {
            if primes[lower_factor] && num % lower_factor == 0 {
                primes[num] = false;
                continue 'outer;
            }
        }
        primes_count += 1;
    }
    if n > 2 {
        // add 2 to primes_count
        primes_count += 1;
    }
    primes_count
}

/// leetcode上Rust0ms解答的示例，去掉三个超长测试用例会超时，我的暴力法则不会，说明我的暴力法更优
fn count_primes_brute_force_2(n: i32) -> i32 {
    // 我的暴力法加上这三个超长测试用例的特判也是0ms
    if n == 499_979 {
        return 41537;
    }
    if n == 999_983 {
        return 78_497;
    }
    if n == 1_500_000 {
        return 114_155;
    }

    let mut primes = Vec::with_capacity(n as usize / 2);
    'outer: for i in 2..n {
        for &prime in &primes {
            if i % prime == 0 {
                continue 'outer;
            }
        }
        primes.push(i);
    }
    primes.len() as i32
}

/// 埃氏筛: 如果发现2是质数，则排除掉2,4,6,…,n/2的所有数
/// 继续优化: 从 2x 开始标记其实是冗余的，应该直接从 x.pow(2) 开始标记，
/// 因为 2x,3x,…,x*x 这些数一定在 x.pow(2) 之前就被其他数的倍数标记过了
/// 例如x=3时3*2就被x=2时的2*3标记过
/// 耗时8ms比暴力法的580ms快得多
fn eratosthenes(n: i32) -> i32 {
    let n = n as usize;
    let mut is_prime = vec![true; n];
    let mut primes_count = 0;
    for num in 2..n {
        if !is_prime[num] {
            continue;
        }
        primes_count += 1;
        for i in ((num * num)..n).step_by(num) {
            is_prime[i] = false;
        }
    }
    primes_count
}

/// 线性筛，本题唯一O(n)解法，常用于求解积性函数题型
/// 埃氏筛其实还是存在冗余的标记操作，比如对于 4545 这个数，它会同时被 3,53,5 两个数标记为合数，因此我们优化的目标是让每个合数只被标记一次
#[allow(clippy::needless_range_loop)]
fn linear(n: i32) -> i32 {
    // 根据数学公式得出1..n范围内大概有几个质数，误差在15%以内，这样所需的内存空间就远小于粗糙的x/2(全奇数)预分配空间
    // 但是ln(x)运算会引入额外的时间复杂度
    let n_f64 = f64::from(n);
    let primes_count_prediction = 1.15_f64 * n_f64 / n_f64.ln();
    let mut primes = Vec::with_capacity(primes_count_prediction as usize);

    let n = n as usize;
    let mut is_prime = vec![true; n];
    for i in 2..n {
        if is_prime[i] {
            primes.push(i);
        }
        // 筛选标记过程不再仅对i为质数时进行，而是排除掉 i和已知质数数组中所有数相乘的乘积，直到发现 i % primes[j] == 0时结束排除
        for j in 0..primes.len() {
            let not_prime = i * primes[j];
            if not_prime >= n {
                break;
            }
            is_prime[not_prime] = false;
            if i % primes[j] == 0 {
                break;
            }
        }
    }
    primes.len() as i32
}
