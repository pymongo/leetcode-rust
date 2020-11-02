struct Solution;

impl Solution {
    fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut nums: Vec<i32> = (1..=m).collect();
        let mut res = Vec::with_capacity(queries.len());
        for target in queries {
            let index = nums.iter().position(|&num| num == target).unwrap();
            // nums.swap(0, index);
            nums.remove(index);
            nums.insert(0, target);

            res.push(index as i32);
        }
        res
    }
}

#[cfg(test)]
const TESTCASES: [(&[i32], i32, &[i32]); 1] = [(&[3, 1, 2, 1], 5, &[2, 1, 2, 1])];

#[test]
fn test_process_queries() {
    for &(queries, m, expected) in &TESTCASES {
        assert_eq!(
            Solution::process_queries(queries.to_vec(), m),
            expected.to_vec()
        );
    }
}
