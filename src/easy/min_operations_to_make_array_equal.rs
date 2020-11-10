/*
nums = [1,3,5,7,9,11]
avg = 6
operations = (6-1)+(6-3)+(6-5)
*/
struct Solution;

impl Solution {
    fn min_operations(n: i32) -> i32 {
        let mut res = 0;
        if n % 2 == 1 {
            for i in 0..=n / 2 {
                res += n - (2 * i + 1);
            }
        } else {
            for i in 0..n / 2 {
                res += n - (2 * i + 1);
            }
        }
        res
        // return n * n /4;
    }
}
