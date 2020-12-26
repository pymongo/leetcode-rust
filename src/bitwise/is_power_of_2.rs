//! https://leetcode.com/problems/power-of-two/
fn is_power_of_two_normal_solution(mut n: i32) -> bool {
    if n == 0 {
        return false;
    }
    while n % 2 == 0 {
        n /= 2;
    }
    // 如果n能被2整除，那么跳出while循环必然是n=2/2=1，所以能被2整数时在这里是1
    n == 1
}

/** 如果一个数是2的幂，那么它的二进制表示中有且只有一位是1
(n & -n): 只获取n的二进制的从左到右第一个1，其余位置0，如果n的二进制中只有一个1，那么(n & -n) == n
Example1:
 3 = 0011
-3 = 1101
3 & -3 = 1
Example2:
 4 = 0100
-4 = 1100
4 & -4 = 0100
*/
fn is_power_of_two_bitwise_solution_1(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    (n & -n) == n
}

/**
(n - 1) 代表了将 n 最右边的 1 设置为 0，并且将较低位设置为 1
(n & n-1): 去掉n的二进制中最后一个1，如果去掉之后等于0则说明是2的幂
Example1:
3 = 0011
2 = 0010
3 & 2 = 1
Example2:
4 = 0100
3 = 0011
4 & 3 = 0
*/
fn is_power_of_two_bitwise_solution_2(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    n & (n - 1) == 0
}

/**
## clippy::float_cmp reproduce
error: strict comparison of `f32` or `f64`
   = note: `#[deny(clippy::float_cmp)]` on by default
   = note: `f32::EPSILON` and `f64::EPSILON` are available for the `error_margin`
*/
#[cfg(not)]
fn clippy_float() {
    assert_eq!(1f32, 1.0001f32);
}

#[test]
fn test_is_power_of_two() {
    /// https://stackoverflow.com/questions/5796983/checking-if-float-is-an-integer
    fn is_f32_an_integer(float: f32) -> bool {
        (float - float.round()).abs() < f32::EPSILON
    }
    extern "C" {
        fn time(time: *mut isize) -> isize;
        fn rand() -> i32;
        fn srand(seed: u32);
    }
    unsafe {
        srand(time(&mut std::mem::zeroed()) as u32);
    }
    for _ in 0..100_000 {
        let input = unsafe { rand() };
        let input_is_power_of_2 = is_f32_an_integer((input as f32).log2());
        assert_eq!(
            is_power_of_two_bitwise_solution_1(input),
            input_is_power_of_2
        );
    }
}
