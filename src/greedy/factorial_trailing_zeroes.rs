/// https://leetcode.com/problems/factorial-trailing-zeroes/
const fn trailing_zeroes(mut n: i32) -> i32 {
    let mut factor_5_count = 0;
    while n != 0 {
        n /= 5;
        factor_5_count += n;
    }
    factor_5_count
}
