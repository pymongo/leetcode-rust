//! https://leetcode-cn.com/problems/palindrome-number/
//! 输入一个i32，判断它是不是回文数

#[cfg(test)]
const TEST_CASE: [(i32, bool); 3] = [
    (121, true),
    (-121, false),
    (10, false)
];

#[test]
fn test_first_try() {
    for case in &TEST_CASE {
        dbg!(case);
        assert_eq!(first_try(case.0), case.1);
    }
}

#[cfg(test)]
fn first_try(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x <=9 {
        return true;
    } else {
        false
    }
}
