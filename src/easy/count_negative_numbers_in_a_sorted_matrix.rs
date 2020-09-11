/*
 4  3  2 -1
 3  2  1 -1
 1  1 -1 -2
-1 -1 -2 -3
这题跟剑指Offer的search in 2D array用的是同样的数据结构
*/
struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut i, mut j) = (m - 1, 0);
        let mut cnt = 0;
        while j < n {
            let cur = grid[i][j];
            if cur >= 0 {
                j += 1;
            } else {
                // 既然是有序的，那就可以批量数该行的负数个数，然后上移
                cnt += n - j;
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }
        return cnt as i32;
    }
}
