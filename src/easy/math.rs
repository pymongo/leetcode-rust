/** https://leetcode.com/problems/climbing-stairs/
& https://leetcode.com/problems/fibonacci-number/
比尾递归O(n)更快的还有O(log(n))的解法：
1. 斐波那契公式(公式中的乘方需要log(n)时间复杂度)
2. Binet's formula 利用矩阵解斐波那契
*/
const fn fib_recursive(n: i32, a: i32, b: i32) -> i32 {
    if n == 1 {
        b
    } else {
        // 注意尾递归解法只能从1逼近n，普通递归解法一般是从f(n-1)一直加到f(1)
        fib_recursive(n - 1, b, a + b)
    }
}

const fn fib_iterative(mut n: i32) -> i32 {
    let (mut a, mut b) = (0i32, 1i32);
    while n >= 1 {
        let temp = b;
        b += a;
        a = temp;
        n -= 1;
    }
    a
}

fn fib_reduce(n: i32) -> i32 {
    (0..n)
        .fold((0, 1), |accum, _each| (accum.1, accum.0 + accum.1))
        .0
}

/// https://leetcode.com/problems/ugly-number/
fn is_ugly(mut num: i32) -> bool {
    if num == 0 {
        return false;
    }
    while num % 2 == 0 {
        num /= 2;
    }
    while num % 3 == 0 {
        num /= 3;
    }
    while num % 5 == 0 {
        num /= 5;
    }
    num == 1
}

/// https://leetcode.com/problems/perfect-number/
/// 在枚举时，我们只需要从 1 到 sqrt(n) 进行枚举即可。
/// 这是因为如果 n 有一个大于 sqrt(n) 的因数 x，那么它一定有一个小于 sqrt(n) 的因数 n/x
/// 所以求一个较小因数的同时也记入较大因数即可，一对一对地数更快
fn check_perfect_number(num: i32) -> bool {
    // num == 6 || num == 28 || num == 496 || num == 8128 || num == 33550336
    if num == 1 {
        return false;
    }
    let mut factors_sum = 1;
    for lower_factor in 2..=((num as f32).sqrt() as i32) {
        if num % lower_factor == 0 {
            factors_sum += lower_factor + (num / lower_factor);
        }
    }
    factors_sum == num
}
