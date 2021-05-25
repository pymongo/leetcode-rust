/// https://leetcode.com/problems/island-perimeter/
/// 逐行遍历grid中所有为1的格子，遇到一个1就往上下左右四个方向延伸，遇到边界或0就周长加一，遇到1则不加
fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut perimeter = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                continue;
            }
            // up and down
            if i == 0 || grid[i - 1][j] == 0 {
                perimeter += 1;
            }
            if i == m - 1 || grid[i + 1][j] == 0 {
                perimeter += 1;
            }
            // left and right
            if j == 0 || grid[i][j - 1] == 0 {
                perimeter += 1;
            }
            if j == n - 1 || grid[i][j + 1] == 0 {
                perimeter += 1;
            }
        }
    }
    perimeter
}

#[test]
fn test_island_perimeter() {
    /*
    "[[0, 1, 0, 0],
      [1, 1, 1, 0],
      [0, 1, 0, 0],
      [1, 1, 0, 0]]",
    */
    let test_cases = vec![(
        vec_vec![[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]],
        16,
    )];
    for (grid, perimeter) in test_cases {
        assert_eq!(island_perimeter(grid), perimeter);
    }
}
