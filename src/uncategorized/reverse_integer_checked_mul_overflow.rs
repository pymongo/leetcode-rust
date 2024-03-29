/*! https://leetcode.com/problems/reverse-integer/
& https://leetcode.com/problems/palindrome-number/

```text
https://twitter.com/ospopen/status/1322127786618748928
好喜欢Rust这种checked前缀的命名和设计，优雅解决了反转i32的溢出问题
不用像官方解答那样冗长的上限下限溢出判断，还有一堆数学公式推导出7和-8这两个晦涩难懂的临界值

与之相反的unchecked前缀的API都是unsafe的，例如unchecked_sub溢出就会运行时panicked at 'attempt to subtract with overflow'
```
*/

/// 注意Python/Ruby不能这么写(abs())，因为 Python -1整除10都等于-1(div_euclid)，会陷入死循环
/// reverse_integer(-2147483648) may panic on debug build(because overflow-checks=true profile), but work good on release build
fn reverse_integer(x: i32) -> i32 {
    /*
    || -> Option<i32> {
        let mut ret = 0_i32;
        while x.abs() != 0 {
            ret = ret.checked_mul(10)?.checked_add(x%10)?;
            x /= 10;
        }
        Some(ret)
    }().unwrap_or(0)
    */
    fn helper(mut n: i32) -> Option<i32> {
        let mut ret = 0_i32;
        while n.abs() != 0 {
            ret = ret.checked_mul(10)?.checked_add(n % 10)?;
            n /= 10;
        }
        Some(ret)
    }
    helper(x).unwrap_or_default()
}

// use std::ops::ControlFlow;
// fn reverse_integer_control_flow(mut n: i32) -> ControlFlow<i32, i32> {
//     let mut ret = 0_i32;
//     while n.abs() != 0 {
//         ret = ret.checked_mul(10)?.checked_add(n % 10)?;
//         n /= 10;
//     }
//     Controlo
// }

/// Beware of overflow when you reverse the integer
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    x == reverse_integer(x)
}

const fn is_palindrome_half_traverse(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut num = x;
    let mut rev = 0;
    // 只会【遍历一半】，遍历到中间时rev和num的值就会相近
    while rev < num {
        rev = rev * 10 + num % 10;
        num /= 10;
    }
    // dbg!((x, num, rev));
    rev == num || num == (rev / 10)
}

#[test]
fn test_reverse() {
    assert_eq!(reverse_integer(-123), -321);
}

#[test]
fn test_is_palindrome() {
    const TEST_CASES: [(i32, bool); 5] = [
        (121, true),
        (-121, false),
        (10, false),
        (0, true),
        (1_000_000_001, true),
    ];
    for (input, expected) in TEST_CASES {
        assert_eq!(is_palindrome(input), expected);
        assert_eq!(is_palindrome_half_traverse(input), expected);
    }
}
