//! https://leetcode-cn.com/problems/palindrome-number/
//! 输入一个i32，判断它是不是回文数

#[cfg(test)]
const TEST_CASE: [(i32, bool); 5] = [
    (121, true),
    (-121, false),
    (10, false),
    (0, true),
    (1000000001, true),
];

#[test]
fn test_reverse() {
    for &(input, expected) in TEST_CASE.iter() {
        assert_eq!(reverse(input), expected);
    }
}

/*
rust此题想要0ms以内还是有点难，学习了全球服第一的写法，确实用了很多小技巧
执行耗时：0ms
*/
#[cfg(test)]
fn reverse(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut num = x;
    let mut rev = 0;
    // 只会【遍历一半】，遍历到中间时rev和num的值就会相近
    while rev < num {
        rev = rev * 10 + num % 10;
        num = num / 10;
    }
    // dbg!((x, num, rev));
    return rev == num || num == (rev / 10);
}

#[test]
fn test_i32_to_string() {
    for &(input, expected) in TEST_CASE.iter() {
        assert_eq!(i32_to_string(input), expected);
    }
}

/*
算法：先转换为字符串再首尾比较
执行耗时：16ms
*/
#[cfg(test)]
fn i32_to_string(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let number = x.to_string();
    let chars = number.as_bytes();
    let len = chars.len();
    if len == 1 {
        return true;
    }
    let mut left = 0usize;
    let mut right = len - 1;
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

#[test]
fn test_first_try() {
    for &(input, expected) in TEST_CASE.iter() {
        assert_eq!(first_try(input), expected);
    }
}

/*
算法：模10取余，除10
Get quotient and remainder with divmod() in Python
python提供了divmod的API直接取除法结果的商和余数
执行耗时：8ms
TODO 优化：遍历一半位数即可，但是你该如何判断已经遍历了一半呢？
TODO 进一步的优化，必须用【位运算】
*/
#[cfg(test)]
fn first_try(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x <= 9 {
        return true;
    } else {
        let mut number = x;
        let mut digits: Vec<i32> = Vec::new();
        let (mut left, mut right) = (0usize, 0usize);
        while number != 0 {
            digits.push(number % 10);
            right += 1;
            number /= 10;
        }
        right -= 1;
        while left < right {
            if digits[left] != digits[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
