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

/// number_of_1_bits: n.count_ones(); // Java: Integer.bitCount(n)
mod count_ones {
    /// https://leetcode.com/problems/number-of-1-bits/
    fn hamming_weight_count_ones_solution(n: u32) -> i32 {
        n.count_ones() as i32
    }

    /// 用汇编的popcnt指令去统计正整数里有几个1
    fn hamming_weight_asm_popcnt_instruction_solution(n: u32) -> i32 {
        let popcnt_input: usize = n as usize;
        let popcnt_output: usize;
        unsafe {
            asm!(
                "popcnt {popcnt_output}, {popcnt_input}",
                popcnt_input = in(reg) popcnt_input,
                popcnt_output = out(reg) popcnt_output,
            );
        }
        popcnt_output as i32
    }

    #[test]
    fn t() {
        dbg!(hamming_weight_asm_popcnt_instruction_solution(5));
    }

    /// count_ones最容易理解和记忆的解法
    fn hamming_weight_bitmask_easy_solution(n: u32) -> i32 {
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

    /// 利用`is_power_of_2`一题的相同的位运算技巧取出从右往左的第一个1
    fn impl_count_ones_by_mask(mut n: u32) -> i32 {
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }

    /// https://leetcode.com/problems/hamming-distance/
    /// 两个整数之间的汉明距离指的是这两个数字对应二进制位不同的位置的数目
    /// 思路: 异或后数位1的个数
    fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }

    /// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
    fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        // arr.sort_by_cached_key(|&x| (x.count_ones, x));
        arr.sort_unstable_by(|a, b| a.count_ones().cmp(&b.count_ones()).then(a.cmp(b)));
        arr
    }
}

/**
|136|[Single Number](https://leetcode.com/problems/single-number/)
|137|[Single Number II](https://leetcode.com/problems/single-number-ii/)
|260|[Single Number III](https://leetcode.com/problems/single-number-iii/)|
|389|[Find The Difference](https://leetcode.com/problems/find-the-difference/)
*/
mod find_single_number {
    /// nums是一个长度为2n+1的的数组，有其中一个元素出现了一次，其余元素都是出现了两次
    /// 利用数学运算规律: (sum_all_other+single)*2 - (sum_all_other*2+single) = single
    fn single_number_sum_solution(nums: Vec<i32>) -> i32 {
        let nums_set: std::collections::HashSet<i32> = nums.clone().into_iter().collect();
        nums_set.into_iter().sum::<i32>() * 2 - nums.into_iter().sum::<i32>()
    }

    /// 利用异或规律: A^B^A = A^A^B = 0^B = B
    fn single_number_xor_solution(nums: Vec<i32>) -> i32 {
        // leetcode的Rust版本太低，还没有bitxor API(或者需要`use std::ops::BitXor;`)
        // nums.into_iter().fold(0, |a, b| a.bitxor(b))
        nums.into_iter().fold(0, |a, b| a ^ b)
    }

    /// single_number_2的bitwise解法要用状态机去理解，不方便背诵
    fn single_number_2_sum_solution(nums: Vec<i32>) -> i32 {
        let nums_set: std::collections::HashSet<i64> =
            nums.clone().into_iter().map(|num| num as i64).collect();
        let nums_set_sum = nums_set.into_iter().sum::<i64>();
        let nums_sum = nums.into_iter().map(|num| num as i64).sum::<i64>();
        ((nums_set_sum * 3 - nums_sum) / 2) as i32
    }

    fn single_number_3_counter_solution(nums: Vec<i32>) -> Vec<i32> {
        let mut counter = std::collections::HashMap::with_capacity(nums.len());
        for num in nums.into_iter() {
            *counter.entry(num).or_insert(0u16) += 1;
        }
        let mut ret = Vec::with_capacity(2);
        for (num, count) in counter.into_iter() {
            if count == 1 {
                ret.push(num);
                if ret.len() == 2 {
                    return ret;
                }
            }
        }
        unreachable!()
    }

    fn find_the_difference_counter_solution(s: String, t: String) -> char {
        let mut counter = [0u16; 26];
        for each in s.into_bytes().into_iter() {
            counter[(each - b'a') as usize] += 1;
        }
        for each in t.into_bytes().into_iter() {
            let idx = (each - b'a') as usize;
            match counter[idx].checked_sub(1) {
                Some(new_val) => counter[idx] = new_val,
                None => return each as char,
            }
        }
        for (i, each) in counter.iter().enumerate() {
            if each.eq(&1) {
                return (i as u8 + b'a') as char;
            }
        }
        unreachable!()
    }

    fn find_the_difference_sum_solution(s: String, t: String) -> char {
        (t.into_bytes().into_iter().sum::<u8>() - s.into_bytes().into_iter().sum::<u8>()) as char
    }

    fn find_the_difference_xor_solution(s: String, t: String) -> char {
        s.into_bytes()
            .into_iter()
            .chain(t.into_bytes().into_iter())
            .fold(0, |a, b| a ^ b)
            .into()
    }
}
