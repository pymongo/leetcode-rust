/*!
https://leetcode.com/problems/max-area-of-island
VecDeque双端队列内部通过ring buffer环形数组实现
在rust的1.21版本之前以下函数是错的
fn is_full(&self) -> bool {
    self.cap() - self.len() == 1
}
因为环状数组，tail在head的前一位索引时表示已满，所以is_full方法内需要对cap()逻辑容量进行-1后再跟物理容量len去比较
*/
struct Solution;

impl Solution {
    /// 跟`Number of Islands`一题中判断is_island的过程类似，只是要返回的数据不同
    fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = std::collections::VecDeque::new();
        let mut max_island_area = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    continue;
                }
                queue.push_back((i, j));
                let mut curr_island_area_count = 0;
                while let Some((x, y)) = queue.pop_front() {
                    if grid[x][y] == 2 {
                        // 如果(0,1)和(1,0)都连向(1,1) 则(1,1)会被重复算一次，所以这里「也要」过滤掉已被访问过的节点
                        continue;
                    }
                    // 将访问过的点标记为2，避免图中有环导致重复遍历陷入死循环
                    grid[x][y] = 2;
                    curr_island_area_count += 1;
                    // up and down
                    if x > 0 && grid[x - 1][y] == 1 {
                        queue.push_back((x - 1, y));
                    }
                    if x < m - 1 && grid[x + 1][y] == 1 {
                        queue.push_back((x + 1, y));
                    }
                    // left and right
                    if y > 0 && grid[x][y - 1] == 1 {
                        queue.push_back((x, y - 1));
                    }
                    if y < n - 1 && grid[x][y + 1] == 1 {
                        queue.push_back((x, y + 1));
                    }
                }
                max_island_area = max_island_area.max(curr_island_area_count);
            }
        }
        max_island_area
    }
}

#[cfg(test)]
const TESTCASES: [(&[&[i32]], i32); 1] = [(
    &[
        &[1, 1, 0, 0, 0],
        &[1, 1, 0, 0, 0],
        &[0, 0, 0, 1, 1],
        &[0, 0, 0, 1, 1],
    ],
    4,
)];

#[test]
fn test_max_area_of_island() {
    for &(grid, max_area) in &TESTCASES {
        let grid: Vec<Vec<i32>> = grid.iter().map(|each| each.to_vec()).collect();
        assert_eq!(Solution::max_area_of_island(grid), max_area);
    }
}
