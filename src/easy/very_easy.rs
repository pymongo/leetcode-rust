/*!
简短一行Rust代码能解决的题:
[剑指Offer 17. 打印从1到最大n位的十进制数]: (1..10i32.pow(n as u32)).collect()
*/

/** https://leetcode.com/problems/nim-game/
nim游戏规则：有若干个石头，两人每回合轮流拿走一些石头，每个人可以拿走1-3块石头
如果轮到A的回合时，石头数量是4的倍数，那么A必败(博弈问题的必败态)
或者利用二进制判断是不是 4 的倍数，
只需要通过和 3 （二进制 11）进行相与，
如果是 4 的倍数，那么结果一定是 0。

算法如下：
    x&3==0，则是4的倍数。
原理：
先来看一组数字的二进制表示
    4　　　　0100
    8　　　　1000
    12      1100
    16     10000
    20     10100
由此可见 4 的倍数的二进制表示的后 2 为一定为 0。

从另外一个角度来看，4 的二进制表示是 0100，任何 4 的倍数一定是在此基础上增加 n 个 0100
由此也可得 4 的倍数的二进制表示的后 2 为一定为 0。
*/
const fn nim_game_bitwise(n: i32) -> bool {
    // (n % 4) != 0
    (n & 3) != 0
}

/** https://leetcode.com/problems/shuffle-the-array/
数组nums按 \[x1,x2,...,xn,y1,y2,...,yn] 的格式排列

请你将数组按 [x1,y1,x2,y2,...,xn,yn] 格式重新排列
*/
fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n: usize = n as usize;
    let mut result: Vec<i32> = Vec::with_capacity(2 * n);
    for i in 0..n {
        result.push(nums[i]);
        result.push(nums[i + n]);
    }
    result
}

#[test]
fn test_shuffle() {
    const TESTCASES: [(&[i32], i32, &[i32]); 1] = [(&[2, 5, 1, 3, 4, 7], 3, &[2, 3, 5, 4, 1, 7])];
    for &(nums, n, expected) in TESTCASES.iter() {
        let output = Solution::shuffle(nums.to_vec(), n);
        assert_eq!(&output[..], expected);
    }
}

/** https://leetcode.com/problems/climbing-stairs/
& https://leetcode.com/problems/fibonacci-number/
比尾递归O(n)更快的还有O(log(n))的解法：
1. 斐波那契公式(公式中的乘方需要log(n)时间复杂度)
2. Binet's formula 利用矩阵解斐波那契
*/
fn fib_recursive(n: u32, a: u32, b: u32) -> u32 {
    if n == 1 {
        b
    } else {
        // 注意尾递归解法只能从1逼近n，普通递归解法一般是从f(n-1)一直加到f(1)
        fib_recursive(n - 1, b, a + b)
    }
}

fn fib_iterative(n: i32) -> i32 {
    let (mut a, mut b) = (1u32, 1u32);
    let mut temp: u32;
    let mut n = n;
    while n > 1 {
        temp = b;
        b += a;
        a = temp;
        n -= 1;
    }
    b as i32
}

/// https://leetcode.com/problems/to-lower-case/
fn to_lower_case(s: String) -> String {
    // 既然是ASCII编码，更高效的做法可能是u8数组判断在大写范围的挨个-32
    s.to_ascii_lowercase()
}
