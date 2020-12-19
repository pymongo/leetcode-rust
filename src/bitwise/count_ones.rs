/// https://leetcode.com/problems/number-of-1-bits/
/// number_of_1_bits: n.count_ones(); // Java: Integer.bitCount(n)
fn hamming_weight_count_ones_solution(n: u32) -> i32 {
    n.count_ones() as i32
}

/// 用汇编的popcnt指令去统计正整数里有几个1
fn hamming_weight_asm_popcnt_solution(n: u32) -> i32 {
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
