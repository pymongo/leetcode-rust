/// https://leetcode.com/problems/number-of-1-bits/
mod hamming_weight {
    /// number_of_1_bits: n.count_ones(); // Java: Integer.bitCount(n)
    fn hamming_weight_count_ones_solution(n: u32) -> i32 {
        n.count_ones() as i32
    }

    /// 用汇编的popcnt指令去统计正整数里有几个1
    /// __builtin_popcount in c++
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

    /// count_ones最容易理解和记忆的位运算解法
    fn hamming_weight_bitwise_solution_1(n: u32) -> i32 {
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
    fn hamming_weight_bitwise_solution_2(mut n: u32) -> i32 {
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}

/// https://leetcode.com/problems/hamming-distance/
/// 汉明距离: 两个整数的二进制表示不同位置的个数
/// 思路: 异或后(不同位得1) count_ones
fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

/// https://leetcode.com/problems/total-hamming-distance/
/// O(n^2) 的遍历数组中的两两组合在 leetcode 用大部分语言提交都会超时
fn total_hamming_distance_brute_force(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let len = nums.len();
    for i in 0..len {
        for j in i + 1..len {
            ret += (nums[i] ^ nums[j]).count_ones();
        }
    }
    ret as i32
}

/// 大约 O(n) 时间复杂度
fn total_hamming_distance_counter(nums: Vec<i32>) -> i32 {
    let len = nums.len() as i32;
    // bit_one_counter[i]=k 表示nums数组中第i位为1的有k个，第i位为0的有len-k个
    // 这是种垂直方向一次性计数数组每个元素第i位的解法
    // 例如 数组长度位8, 有3个数字的第0位是1, 那么会有5个数字的第0位是0
    // 第0位对总的汉明距离的影响是5*3或3*5(因为两两成对计入，所以一个1和5个0凑对和1个0和3个1凑对是一样的)
    let mut bit_one_counter = [0u16; 32];
    for mut num in nums {
        let mut bit_index = 1;
        while num > 0 {
            // num & 0x01 取 num 最右的位
            if num & 0x01 == 1 {
                bit_one_counter[bit_index] += 1;
            }
            // num 右移1位，更新 num 的最后一位
            num >>= 1;
            bit_index += 1;
        }
    }
    let mut ret = 0;
    for count in std::array::IntoIter::new(bit_one_counter) {
        let ones = count as i32;
        let zeros = len - ones;
        ret += ones * zeros;
    }
    ret
}

#[test]
fn test_total_hamming_distance() {
    const TEST_CASES: [(&[i32], i32); 1] = [(&[4, 14, 2], 6)];
    for (input, output) in std::array::IntoIter::new(TEST_CASES) {
        assert_eq!(total_hamming_distance_brute_force(input.to_vec()), output);
        assert_eq!(total_hamming_distance_counter(input.to_vec()), output);
    }
}

/// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
    // arr.sort_by_cached_key(|&x| (x.count_ones, x));
    arr.sort_unstable_by(|a, b| a.count_ones().cmp(&b.count_ones()).then(a.cmp(b)));
    arr
}
