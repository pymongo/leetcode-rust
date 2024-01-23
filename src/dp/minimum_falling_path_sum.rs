//! https://leetcode.cn/problems/minimum-falling-path-sum
fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut min = i32::MAX;
    for j in 0..n {
        min = min.min(search(0, j, m, n, 0, &matrix));
    }

    min
}

fn search(row: usize, col:usize, m: usize, n: usize, mut sum: i32, matrix: &Vec<Vec<i32>>) -> i32 {
    if row == m {
        return sum;
    }
    let cur_val = matrix[row][col];
    sum += cur_val;
    let mut min = search(row+1,col, m, n, sum, matrix);
    if col >= 1 {
        min = min.min(search(row+1,col-1, m, n, sum, matrix));
    }
    if col < n-1 {
        min = min.min(search(row+1,col+1, m, n, sum, matrix));
    }
    min
}

#[test]
fn test() {
    for (matrix, min_path_sum) in [
        (
            vec_vec![
                [-19,57],[-40,-5]
            ],
            -59
        ),
        (
            vec_vec![
                [2,1,3],[6,5,4],[7,8,9]
            ],
            13
        )
    ] {
        assert_eq!(min_falling_path_sum(matrix), min_path_sum);
    }
}
