/*! https://leetcode.com/problems/power-of-two/
https://leetcode.com/problems/power-of-three/
*/

/// is_power_of_x一类题型的万能解法模板
const fn is_power_of_two_normal_solution(mut n: i32) -> bool {
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
const fn is_power_of_two_bitwise_solution_1(n: i32) -> bool {
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
const fn is_power_of_two_bitwise_solution_2(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    n & (n - 1) == 0
}

/**
## reproduce ve'rclippy::float_cmp
error: strict comparison of `f32` or `f64`
   = note: `#[deny(clippy::float_cmp)]` on by default
   = note: `f32::EPSILON` and `f64::EPSILON` are available for the `error_margin`
const float EPSINON = 0.00001;
if ((x >= - EPSINON) && (x <= EPSINON) {}
*/
#[cfg(not)]
fn clippy_float() {
    assert_eq!(1f32, 1.0001f32);
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn test_is_power_of_two() {
    /// https://stackoverflow.com/questions/5796983/checking-if-float-is-an-integer
    fn is_f32_an_integer(float: f32) -> bool {
        (float - float.round()).abs() < f32::EPSILON
    }
    use crate::code_snippets::random_i32::random_i32;
    for _ in 0..10_000 {
        let input = random_i32();
        let input_is_power_of_2 = is_f32_an_integer((input as f32).log2());
        assert_eq!(
            is_power_of_two_bitwise_solution_1(input),
            input_is_power_of_2
        );
    }
}

/*
```text
    1 0b1
    3 0b11
    9 0b1001
   27 0b11011
   81 0b1010001
  243 0b11110011
  729 0b1011011001
```
3的幂的规律(自己瞎猜的)
1. 个位是1,3,9,7: n%3=0 or 1
2. 二进制最后一位是1: (n & -n) == 1
3. 二进制最高位是1: ?

获取从右边往左第一位非0 bit的索引位置的方法: math.log2(n&-n)+1

# 官方解答:

1. 转换为3进制后做正则
// Integer.toString(n, 3)表示将n转为3进制的字符串
// 字符串是否以1 ^1 开头，后跟 0 或 多个 0 0* 并且不包含任何其他值
return Integer.toString(n, 3).matches("^10*$");
2. 使用库函数log
if n <= 0:
    return False
# >>> math.log(243, 3)
# 4.999999999999999
return math.log(n, 3).is_integer()
3. 借助i32中最大的3的幂(3^19)
因为 3 是质数，所以 3^19(i32范围内最大的3的幂)的除数只有 3^0, 3^1, ...
因此我们只需要将 3^19 除以 n 若余数为 0 意味着 n 是 3^19 的除数，因此是 3 的幂
return n > 0 && 1162261467 % n == 0;
*/
//fn is_power_of_3() {}

/**
4=0100的幂只可能在奇数位上有1
如果n是2的幂同时只有奇数位上是1(n & 0xaaaaaaaa == 0)，那么n是4的幂
*/
#[allow(overflowing_literals)]
const fn is_power_of_4_solution1(n: i32) -> bool {
    // the literal `0xaaaaaaaa` (decimal `2863311530`) does not fit into the type `i32` and will become `-1431655766i32`
    n > 0 && (n & -n) == n && n & 0xaaaa_aaaa == 0
}

/**
若 xx 为 2 的幂且 x%3 == 1，则 xx 为 4 的幂
证明方法: 4^k % 3 = (3+1)^k % 3 = ...(二项式展开) = 1
*/
const fn is_power_of_4_solution2(n: i32) -> bool {
    n > 0 && (n & -n) == n && n % 3 == 1
}
