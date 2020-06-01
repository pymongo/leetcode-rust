//! https://leetcode.com/problems/two-sum/
//! 本题主要是掌握使用(B)Tree Map的数据结构达到O(n)的时间复杂度
//! bitwise_补码的解法收录在java_leetcode中
//! 不记录暴力遍历求解的方法

/*
阅读第一名的代码的收获：
1. 在函数中间return，题目只要求返回一对索引，不需要遍历完整个数组(浪费时间)
2. BTreeMap在.get()方法的性能上明显强于HashMap
   理论上红黑树的时间复杂度为O(logN)，散列的时间复杂度为O(1)
   在JDK1.8中，HashMap的长度大于8时才会转为红黑树进行存储
   在Java中TreeMap是自动排序的，因此插入/删除操作会牺牲性能
3. 函数的最后，如果测试用例没有匹配项的话，可以写unreachable!()或返回vec![]
*/

pub const TEST_CASES: [(&[i32], i32, &[i32]); 3] = [
    (&[2, 7, 9, 11], 9, &[0, 1]),
    (&[-3, 4, 3, 90], 0, &[0, 2]),
    (&[0, 4, 3, 0], 0, &[0, 3]),
];

#[test]
fn test_two_sum_bitwise() {
    for case in &TEST_CASES {
        let nums: Vec<i32> = case.0.iter().cloned().collect();
        let result = two_sum_bitwise(nums, case.1);
        let expected: Vec<i32> = case.2.iter().cloned().collect();
        assert_eq!(result, expected)
    }
}

/* Two's complement(补码)存储负数的解法
* 以4-bit(16进制)为例
*
* ## 时钟模型
* 由于1111+0001会溢出所有位清0，所以不断地+1相当于有16个刻度的时钟在顺时针转圈
* 所以-7时钟倒着拨动7格相当于 正着拨动16-7格
*
* ## 0的状态重复了
* 用时钟模型的话，0有两种表示方法，正转360°或者负转360°
* 但是在数字电路中，一个值两种状态都能表示不仅是一个UB，而且还浪费了宝贵的状态
* 所以出现了 负数补码=反码+1 也就是让负数的从-0开始全部往后挪一位
* -0变成了-1，-7变成了-8，所以4bit寄存器有符号数的范围是[-8, 7]
* */
pub fn two_sum_bitwise(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 2047是input case的最大值，确保：
    // 1. a & bit_mode = a;
    // 2. -a & bit_mode = bit_mode-a+1
    const VOLUME: usize = 2048;
    const BIT_MODE: i32 = 2047;
    // on a 32 bit x86 computer,  usize = u32,
    // while on x86_64 computers, usize = u64
    let mut sum_tracker: [usize; VOLUME] = [0; 2048];
    let mut c: usize;
    for i in 0..nums.len() {
        // & BIT_MODE防止相减后出现负数索引
        // 例如：-4 & 2047 = 4
        c = ((target - nums[i]) & BIT_MODE) as usize;
        debug_assert!(c.ge(&0));
        if sum_tracker[c as usize] != 0 {
            return vec![(sum_tracker[c] - 1) as i32, i as i32];
        }
        // 加1防止index=0时保存的记录被`result[c] != 0`拦截
        // & bitMode防止相减后出现负数索引(case twoSumBitWise2)
        sum_tracker[(nums[i] & BIT_MODE) as usize] = i + 1;
    }
    unreachable!()
}

#[test]
fn test_two_sum_btree_map() {
    for case in &TEST_CASES {
        let nums: Vec<i32> = case.0.iter().cloned().collect();
        let result = two_sum_btree_map(nums, case.1);
        let expected: Vec<i32> = case.2.iter().cloned().collect();
        assert_eq!(result, expected)
    }
}

pub fn two_sum_btree_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_tracker: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
    for (i, num) in nums.iter().enumerate() {
        if sum_tracker.contains_key(num) {
            return vec![*sum_tracker.get(num).unwrap() as i32, i as i32];
        }
        sum_tracker.insert(target - *num, i);
    }
    unreachable!()
}

#[test]
fn test_sum_hashmap() {
    for case in &TEST_CASES {
        let nums: Vec<i32> = case.0.iter().cloned().collect();
        let result = two_sum_hashmap(nums, case.1);
        let expected: Vec<i32> = case.2.iter().cloned().collect();
        assert_eq!(result, expected)
    }
}

pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_tracker: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if sum_tracker.contains_key(num) {
            return vec![*sum_tracker.get(num).unwrap() as i32, i as i32];
        }
        sum_tracker.insert(target - *num, i);
    }
    unreachable!()
    // vec![]
}
