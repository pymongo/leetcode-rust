use std::collections::VecDeque;

// 跟is_island一题过程类似，只是要返回的数据不同
fn bfs(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    if m == 0 {
        return -1;
    }
    let n = grid[0].len();
    let m_i32 = m as i32;
    let n_i32 = n as i32;

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut max_count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 1 {
                continue;
            }
            queue.push_back((i as i32, j as i32));
            let mut curr_count = 0;
            while !queue.is_empty() {
                let (curr_x, curr_y) = queue.pop_front().unwrap();
                if grid[curr_x as usize][curr_y as usize] == 2 {
                    // 如果2和3都连向4则4会被重复算一次，所以这里「也要」过滤掉已被访问过的节点
                    continue;
                }
                // 将访问过的点标记为2，避免图中有环
                grid[curr_x as usize][curr_y as usize] = 2;
                curr_count += 1;
                for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    // 注意不能用usize，因为下一对x,y可能是负数
                    let (x, y) = (curr_x + dx, curr_y + dy);
                    if x < 0 || x >= m_i32 || y < 0 || y >= n_i32 {
                        continue;
                    }
                    if grid[x as usize][y as usize] != 1 {
                        continue;
                    }
                    queue.push_back((x, y));
                }
            }
            max_count = max_count.max(curr_count);
        } // for j in 0..n
    } // for i in 0..m
    return max_count;
}

#[cfg(test)]
const TEST_CASES: [(&[&[i32]], i32); 1] = [
    (&[&[1, 1, 0, 0, 0], &[1, 1, 0, 0, 0], &[0, 0, 0, 1, 1], &[0, 0, 0, 1, 1]], 4)
];

#[test]
fn test() {
    for &(grid, max_area) in &TEST_CASES {
        // let mut grid_vec = Vec::new();
        // for &row in grid {
        //     grid_vec.push(row.to_vec());
        // }
        let grid_vec: Vec<Vec<i32>> = grid
            .iter()
            .map(|each| each.to_vec())
            .collect();
        assert_eq!(max_area, bfs(grid_vec));
    }
}
