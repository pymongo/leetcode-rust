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
