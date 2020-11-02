struct Solution;

impl Solution {
    fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut res = 0;
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
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test_num_special() {
    let testcases = vec![(
        vec![
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ],
        2,
    )];
    for (points, min_cost) in testcases {
        assert_eq!(Solution::num_special(points), min_cost);
    }
}
