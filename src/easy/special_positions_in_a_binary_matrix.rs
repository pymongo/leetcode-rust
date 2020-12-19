#[allow(clippy::needless_range_loop)]
fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (mat.len(), mat[0].len());
    let mut ret = 0;
    for i in 0..m {
        'for_j: for j in 0..n {
            if mat[i][j] == 1 {
                for row in 0..m {
                    if row == i {
                        continue; // for row in 0..m
                    }
                    if mat[row][j] == 1 {
                        continue 'for_j;
                    }
                }
                for col in 0..n {
                    if col == j {
                        continue;
                    }
                    if mat[i][col] == 1 {
                        continue 'for_j;
                    }
                }
                ret += 1;
            }
        }
    }
    ret
}

#[test]
fn test_num_special() {
    let test_cases = vec![(
        vec![
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ],
        2,
    )];
    for (points, min_cost) in test_cases {
        assert_eq!(num_special(points), min_cost);
    }
}
