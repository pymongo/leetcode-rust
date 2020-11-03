/** https://leetcode.com/problems/reverse-integer/ & https://leetcode.com/problems/palindrome-number/

```text
https://twitter.com/ospopen/status/1322127786618748928
好喜欢Rust这种checked前缀的命名和设计，优雅解决了反转i32的溢出问题
不用像官方解答那样冗长的上限下限溢出判断，还有一堆数学公式推导出7和-8这两个晦涩难懂的临界值

与之相反的unchecked前缀的API都是unsafe的，例如unchecked_sub溢出就会运行时panicked at 'attempt to subtract with overflow'
```
*/
struct Solution;

/**
注意Python/Ruby/js等脚本语言不能这么写(abs())，因为这三种语言-1整除10都等于-1，会陷入死循环，而C/Java/Rust则不会

```python
I32_MIN = -2 ** 31
I32_MAX = 2 ** 31 - 1

def reverse_number(n: int) -> int:
    is_negative: bool = False
    if n < 0:
        is_negative = True
        n = -n
    reverse: int = 0
    while n != 0:
        reverse = reverse * 10 + n % 10
        n //= 10
    if is_negative:
        return 0 if -reverse < I32_MIN else -reverse
    if not is_negative and reverse > I32_MAX:
        return 0
    return reverse
```
*/
impl Solution {
    fn reverse(x: i32) -> i32 {
        fn helper(mut n: i32) -> Option<i32> {
            let mut res = 0i32;
            while n.abs() != 0 {
                // or n.abs().is_positive()
                res = res.checked_mul(10)?.checked_add(n % 10)?;
                n /= 10;
            }
            Some(res)
        }
        helper(x).unwrap_or_default()
    }

    /// Beware of overflow when you reverse the integer
    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        x == Self::reverse(x)
    }

    fn is_palindrome_half_traverse(x: i32) -> bool {
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
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test_is_palindrome() {
        const TESTCASES: [(i32, bool); 5] = [
            (121, true),
            (-121, false),
            (10, false),
            (0, true),
            (1000000001, true),
        ];
        for &(input, expected) in TESTCASES.iter() {
            assert_eq!(Solution::is_palindrome(input), expected);
            assert_eq!(Solution::is_palindrome_half_traverse(input), expected);
        }
    }
}
