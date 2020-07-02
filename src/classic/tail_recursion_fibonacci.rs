//! https://leetcode.com/problems/fibonacci-number/
//! https://leetcode.com/problems/climbing-stairs/

#[cfg(test)]
const TEST_CASE: [(i32, i32); 5] = [(1, 1), (2, 2), (3, 3), (4, 5), (5, 8)];

#[test]
fn test_tail_recursion() {
    for &(input, expected) in TEST_CASE.iter() {
        assert_eq!(tail_recursion_helper(input), expected);
    }
}

#[cfg(test)]
fn tail_recursion_helper(n: i32) -> i32 {
    tail_recursion(n as u32, 1, 1) as i32
}

/* 耗时0ms，内存2MB，时间复杂度O(n)
还有O(log(n))的解法：
1. 斐波那契公式(公式中的乘方需要log(n)时间复杂度)
2. Binet's formula 利用矩阵解斐波那契
*/
#[cfg(test)]
fn tail_recursion(n: u32, a: u32, b: u32) -> u32 {
    if n == 1 {
        b
    } else {
        // 注意尾递归解法只能从1逼近n，普通递归解法一般是从f(n-1)一直加到f(1)
        tail_recursion(n - 1, b, a + b)
    }
}

#[test]
fn test_iterate() {
    for &(input, expected) in TEST_CASE.iter() {
        assert_eq!(iterate(input), expected);
    }
}

/*
def fib(n: int) -> int:
    a, b= 0, 1
    while (n > 0):
        a, b = b, a+b
        n -= 1
    return a
*/
#[cfg(test)]
fn iterate(n: i32) -> i32 {
    let (mut a, mut b) = (1u32, 1u32);
    let mut temp: u32;
    let mut n = n;
    while n > 1 {
        temp = b;
        b = a + b;
        a = temp;
        n -= 1;
    }
    b as i32
}
