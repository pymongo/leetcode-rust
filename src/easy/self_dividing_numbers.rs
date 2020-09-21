struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = Vec::new();
        'outer: for num in left..=right {
            let mut n = num;
            while n != 0 {
                let digit = n % 10;
                // not allowed to contain the digit zero.
                // panicked at 'attempt to calculate the remainder with a divisor of zero'
                if digit == 0 || num % digit != 0 {
                    continue 'outer;
                }
                n /= 10;
            }
            res.push(num);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::self_dividing_numbers(1, 22), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]);
}
