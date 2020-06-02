//! https://leetcode-cn.com/problems/palindrome-number/
//! 输入一个i32，判断它是不是回文数

#[cfg(test)]
const TEST_CASE: [(i32, bool); 4] = [
    (121, true),
    (-121, false),
    (10, false),
    (0, true)
];

#[test]
fn test_first_try() {
    for case in &TEST_CASE {
        dbg!(case);
        assert_eq!(first_try(case.0), case.1);
    }
}

/*
算法：模10取余，除10
TODO Get quotient and remainder with divmod() in Python
python提供了divmod的API直接取除法结果的商和余数
执行耗时：8ms
*/
#[cfg(test)]
fn first_try(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x <=9 {
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
