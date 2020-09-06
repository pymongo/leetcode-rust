struct Solution;

// 如果矩阵的某个元素为0，则将0所在行和所在列的全部元素置0，尽量避免重复操作而且要遍历完矩阵才能设0，避免污染
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        // 已经设成全为0的行和列
        let (mut zero_row, mut zero_col) = (vec![false; m], vec![false; n]);
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    zero_row[i] = true;
                    zero_col[j] = true;
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if zero_row[i] || zero_col[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
