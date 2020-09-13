struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    let mut is_match = true;
                    for row in 0..m {
                        if row == i {
                            continue; // for row in 0..m
                        }
                        if mat[row][j] == 1 {
                            is_match = false;
                            break; // for row in 0..m
                        }
                    }
                    if !is_match {
                        continue; // for j in 0..n
                    }
                    for col in 0..n {
                        if col == j {
                            continue; // for col in 0..n
                        }
                        if mat[i][col] == 1 {
                            is_match = false;
                            break; // for col in 0..n
                        }
                    }
                    if !is_match {
                        continue; // for j in 0..n
                    }
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let mat1 = vec![
        vec![0, 0, 0, 1],
        vec![1, 0, 0, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0],
    ];
    dbg!(Solution::num_special(mat1));
}
