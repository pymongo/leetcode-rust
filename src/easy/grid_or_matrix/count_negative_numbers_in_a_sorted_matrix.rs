/// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
/// 这题跟剑指Offer的`search in 2D array`一题用的是同样的数据结构(排序二维矩阵)
fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
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
            }
            i -= 1;
        }
    }
    cnt as i32
}

#[test]
fn test_count_negatives() {
    assert_eq!(
        count_negatives(vec_vec![
            [4, 3, 2, -1],
            [3, 2, 1, -1],
            [1, 1, -1, -2],
            [-1, -1, -2, -3]
        ]),
        8
    );
}
