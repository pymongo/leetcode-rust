mod count_negative_numbers_in_a_sorted_matrix;
mod island_perimeter;
mod matrix_diagonal_traverse;
mod rotate_matrix;
mod spiral_matrix;

/// https://leetcode.com/problems/set-matrix-zeroes/
/// 需求: 如果矩阵的某个元素为0，则将0所在行和所在列的全部元素置0
/// 注意: 要先遍历一次矩阵，发现哪些坐标是0，然后再将相应行和列置零，不能一边遍历一边置零否则会污染后面的元素
fn set_matrix_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());
    // 已经设成全为0的行和列
    let (mut zero_row, mut zero_col) = (vec![false; m], vec![false; n]);
    for (i, row) in matrix.iter().take(m).enumerate() {
        for (j, col) in row.iter().take(n).enumerate() {
            if *col == 0 {
                zero_row[i] = true;
                zero_col[j] = true;
            }
        }
    }
    for (i, row) in matrix.iter_mut().take(m).enumerate() {
        for (j, col) in row.iter_mut().take(n).enumerate() {
            if zero_row[i] || zero_col[j] {
                *col = 0;
            }
        }
    }
}

/// https://leetcode.com/problems/matrix-diagonal-sum/
/// 本题仅要求算两条主对角线，既↘和↙两个方向的最长对角线
fn matrix_diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let mut ret = 0;

    for j in 0..n {
        // 累加左上-右下对角线
        ret += mat[n - j - 1][j];
        // 累加左下-右上对角线
        ret += mat[j][j];
    }

    // 如果是矩阵长度为奇数，则中间元素会被重复计算，需要去掉
    if n % 2 == 1 {
        ret -= mat[n / 2][n / 2];
    }

    ret
}

#[test]
fn test_matrix_diagonal_sum() {
    const TEST_CASES: [(&[&[i32]], i32); 2] = [
        (
            &[&[1, 1, 1, 1], &[1, 1, 1, 1], &[1, 1, 1, 1], &[1, 1, 1, 1]],
            8,
        ),
        (&[&[5]], 5),
    ];
    for &(mat, res) in &TEST_CASES {
        let n = mat.len();
        let mut mat_vec = Vec::with_capacity(n);
        for &row in mat {
            mat_vec.push(row.to_vec());
        }
        assert_eq!(matrix_diagonal_sum(mat_vec), res);
    }
}
