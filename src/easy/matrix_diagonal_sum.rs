// https://leetcode-cn.com/contest/biweekly-contest-34/problems/matrix-diagonal-sum/
// 34周双周赛第一题

struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut res = 0;


        for j in 0..n {
            // 累加左上-右下对角线
            res += mat[n-j-1][j];
            // 累加左下-右上对角线
            res += mat[j][j];
        }

        // 如果是矩阵长度为奇数，则中间元素会被重复计算，需要去掉
        if n % 2 == 1 {
            res -= mat[n / 2][n / 2];
        }

        res
    }
}

const TEST_CASES: [(&[&[i32]], i32); 2] = [
    (&[
        &[1, 1, 1, 1],
        &[1, 1, 1, 1],
        &[1, 1, 1, 1],
        &[1, 1, 1, 1]
    ], 8),
    (&[&[5]], 5),
];

#[test]
fn test_diagonal_sum() {
    for &(mat, res) in &TEST_CASES {
        let n = mat.len();
        let mut mat_vec = Vec::with_capacity(n);
        for &row in mat {
            mat_vec.push(row.to_vec());
        }
        assert_eq!(Solution::diagonal_sum(mat_vec), res);
    }
}
