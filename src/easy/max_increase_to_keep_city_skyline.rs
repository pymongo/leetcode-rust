struct Solution;

impl Solution {
    // 大意: 先算出旧矩阵每行每列的最大值，然后遍历矩阵看看当前值最大能加到什么，然后累加最大能增加的量
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut max_row: Vec<i32> = Vec::with_capacity(m);
        let mut max_col: Vec<i32> = vec![std::i32::MIN; n];
        for i in 0..m {
            max_row.push(*grid[i].iter().max().unwrap());
        }
        for j in 0..n {
            for i in 0..m {
                max_col[j] = max_col[j].max(grid[i][j]);
            }
        }

        let mut res = 0;
        for i in 0..m {
            let curr_max_row = max_row[i];
            for j in 0..n {
                // 最大能增长的高度等于行列最大值二者的最小值
                res += std::cmp::min(curr_max_row, max_col[j]) - grid[i][j];
            }
        }
        return res;
    }
}
