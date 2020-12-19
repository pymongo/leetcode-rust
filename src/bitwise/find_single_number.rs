/*!
|136|[Single Number](https://leetcode.com/problems/single-number/)
|137|[Single Number II](https://leetcode.com/problems/single-number-ii/)
|260|[Single Number III](https://leetcode.com/problems/single-number-iii/)|
|389|[Find The Difference](https://leetcode.com/problems/find-the-difference/)
*/

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
