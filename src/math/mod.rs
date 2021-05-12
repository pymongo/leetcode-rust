//! 数学相关问题或位运算
mod count_primes;
mod pow;
mod sqrt;

/// https://leetcode.com/problems/ugly-number/
const fn is_ugly(mut num: i32) -> bool {
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
#[allow(clippy::cast_precision_loss)]
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
