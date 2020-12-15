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

/// https://leetcode.com/problems/hamming-distance/
/// 两个整数之间的汉明距离指的是这两个数字对应二进制位不同的位置的数目
/// 思路: 异或后数位1的个数
fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

/// https://leetcode.com/problems/number-of-1-bits/
/// Rust: n.count_ones(), Java: Integer.bitCount(n)
fn hamming_weight(n: u32) -> i32 {
    n.count_ones();
    fn impl_count_ones_best(n: u32) -> i32 {
        let mut count = 0;
        let mut mask = 0b1;
        for _ in 0..32 {
            if n & mask == 1 {
                count += 1;
            }
            mask <<= 1;
        }
        count
    }
    fn impl_count_ones_by_mask(mut n: u32) -> i32 {
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
    impl_count_ones_best(n)
}

/// https://leetcode.com/problems/reverse-bits/
fn reverse_bits(x: u32) -> u32 {
    fn reverse_bits_best(mut n: u32) -> u32 {
        // ret = return
        let (mut ret, mut power) = (0u32, 0u32);
        while n != 0 {
            ret += (n & 1) << power;
            n >>= 1;
            power -= 1;
        }
        ret
    }

    x.reverse_bits()
}
