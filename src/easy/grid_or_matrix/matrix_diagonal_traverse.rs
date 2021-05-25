/**
(d=diagonal,i=row,j=col,len=cur_diagonal_len)
m=3,n=4:
d i j len
0 2 0 1
1 1 0 2
2 0 0 3
3 0 1 3
4 0 2 2
5 0 3 1
6 0 4 0
*/
#[test]
fn diagonal_traverse_from_bottom_left_in_bottom_right_direction() {
    let (m, n) = (3, 4);
    let m_minus_1 = m - 1;
    // bottom_right_direction traverse
    let mut mat = vec![vec![0; n]; m];
    let mut order = 1;
    for diagonal in 0..m + n {
        // current diagonal start row and col
        let row = m_minus_1.saturating_sub(diagonal);
        let col = diagonal.saturating_sub(m_minus_1);
        let cur_diagonal_len = (m - row).min(n - col);
        println!("{} {} {} {}", diagonal, row, col, cur_diagonal_len);
        for offset in 0..cur_diagonal_len {
            // row+offset表示下方向 叠加 col+offset右方向
            mat[row + offset][col + offset] = order;
            order += 1;
        }
    }
    // bottom_right_direction traverse
    for row in mat.into_iter().take(m) {
        println!("{:?}", row);
    }
}

/// https://leetcode.com/problems/sort-the-matrix-diagonally/
fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (mat.len(), mat[0].len());
    let m_minus_1 = m - 1;
    // bottom_right_direction traverse
    for diagonal in 0..m + n {
        // current diagonal start row and col
        let row = m_minus_1.saturating_sub(diagonal);
        let col = diagonal.saturating_sub(m_minus_1);
        let cur_diagonal_len = (m - row).min(n - col);
        let mut cur_diagonal = Vec::with_capacity(cur_diagonal_len);
        println!("{} {} {} {}", diagonal, row, col, cur_diagonal_len);
        for offset in 0..cur_diagonal_len {
            cur_diagonal.push(mat[row + offset][col + offset]);
        }
        cur_diagonal.sort_unstable();
        #[allow(clippy::manual_memcpy)]
        for offset in 0..cur_diagonal_len {
            mat[row + offset][col + offset] = cur_diagonal[offset];
        }
    }
    mat
}

/// https://leetcode.com/problems/toeplitz-matrix/
fn is_toeplitz_matrix(mat: Vec<Vec<i32>>) -> bool {
    let (m, n) = (mat.len(), mat[0].len());
    let m_minus_1 = m - 1;
    // bottom_right_direction traverse
    for diagonal in 0..m + n {
        // current diagonal start row and col
        let row = m_minus_1.saturating_sub(diagonal);
        let col = diagonal.saturating_sub(m_minus_1);
        let cur_diagonal_len = (m - row).min(n - col);
        for offset in 1..cur_diagonal_len {
            if mat[row + offset][col + offset] != mat[row + offset - 1][col + offset - 1] {
                return false;
            }
        }
    }
    true
}

/**
(d=diagonal,i=row,j=col,len=cur_diagonal_len)
m=3,n=3:
d i j len
0 0 0 1
1 0 1 2
2 0 2 3
3 1 2 2
4 2 2 1
*/
#[test]
fn diagonal_traverse_from_top_left_in_bottom_left_direction() {
    let (m, n) = (3, 3);
    let m_minus_1 = m - 1;
    let mut mat = vec![vec![0; n]; m];
    let mut order = 1;
    for diagonal in 0..m + n {
        let row = diagonal.saturating_sub(m_minus_1);
        let col = if diagonal <= m_minus_1 {
            diagonal
        } else {
            m_minus_1
        };
        let cur_diagonal_len = col + 1 - row;
        println!(
            "d={}: ({},{}), diagonal_len={}",
            diagonal, row, col, cur_diagonal_len
        );
        for offset in 0..cur_diagonal_len {
            mat[row + offset][col - offset] = order;
            order += 1;
        }
    }
    for row in mat {
        println!("{:?}", row);
    }
}

/// https://leetcode.com/problems/diagonal-traverse/
/// FIXME wrong answer
fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let m = mat.len();
    if m == 0 {
        return Vec::new();
    }
    if m == 1 {
        // return mat.into_iter().nth(0).unwrap();
    }
    let n = mat[0].len();
    if n == 1 {
        // return mat.into_iter().map(|row| row[0]).collect();
    }
    let m_minus_1 = m.min(n) - 1;
    let mut ret = Vec::with_capacity(m * n);
    let mut is_top_right = true;
    for diagonal in 0..m + n {
        let row = diagonal.saturating_sub(m_minus_1);
        let col = if diagonal <= m_minus_1 {
            diagonal
        } else {
            m_minus_1
        };
        let cur_diagonal_len = (col + 1).saturating_sub(row);
        println!("{}:, {},{},{}", diagonal, row, col, cur_diagonal_len);
        let mut cur_diagonal = Vec::with_capacity(cur_diagonal_len);
        for offset in 0..cur_diagonal_len {
            cur_diagonal.push(mat[row + offset][col - offset]);
        }
        if is_top_right {
            cur_diagonal.reverse();
        }
        ret.extend(cur_diagonal);
        is_top_right = !is_top_right;
    }
    ret
}

#[test]
#[should_panic]
fn test_find_diagonal_order() {
    let test_cases = vec![
        (vec_vec![[2, 5], [8, 4], [0, -1]], vec![2, 5, 8, 0, 4, -1]),
        (
            vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9],
        ),
        (vec_vec![[2, 3], [2, 3]], vec![2, 3]),
    ];
    for (input, output) in test_cases {
        assert_eq!(find_diagonal_order(input), output);
    }
}
