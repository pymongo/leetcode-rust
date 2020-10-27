struct Solution;

impl Solution {
    /** 针对nums每项都是[0,100]之间，可以用「前缀和数组」
     */
    fn prefix_sum_solution(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_sum = [0i32; 101];
        for &num in &nums {
            prefix_sum[num as usize] += 1;
        }
        for i in 1..=100 {
            prefix_sum[i] += prefix_sum[i - 1];
        }

        let mut res = Vec::with_capacity(nums.len());
        for num in nums {
            if num == 0 {
                res.push(0);
            } else {
                res.push(prefix_sum[(num - 1) as usize]);
            }
        }
        res
    }

    fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = Vec::with_capacity(n);
        for i in 0..n {
            let num = nums[i];
            let mut less_count = 0;
            for j in 0..n {
                if i == j {
                    continue;
                }
                if nums[j] < num {
                    less_count += 1;
                }
            }
            res.push(less_count);
        }
        res
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], &[i32]); 1] = [(&[8, 1, 2, 2, 3], &[4, 0, 1, 1, 3])];

#[test]
fn test_smaller_numbers_than_current() {
    for &(nums, output) in &TEST_CASES {
        assert_eq!(
            Solution::smaller_numbers_than_current(nums.to_vec()),
            output.to_vec()
        );
        assert_eq!(
            Solution::prefix_sum_solution(nums.to_vec()),
            output.to_vec()
        );
    }
}
