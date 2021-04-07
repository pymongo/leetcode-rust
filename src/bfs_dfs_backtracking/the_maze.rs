/// https://leetcode.com/problems/the-maze/
fn has_path(mut maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
    let (m, n) = (maze.len() as i32, maze[0].len() as i32);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start[0], start[1]));
    while let Some((x, y)) = queue.pop_front() {
        if x == destination[0] && y == destination[1] {
            return true;
        }
        // mark (x, y) as visited
        maze[x as usize][y as usize] = 2;
        for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (mut x2, mut y2) = (x + dx, y + dy);
            // 必须沿着同一个方向走直到碰到障碍物或边界才能停下换方向，注意不能用`maze[x2][y2] == 0`
            while x2 >= 0 && x2 < m && y2 > 0 && y2 < n && maze[x2 as usize][y2 as usize] != 1 {
                // destructuring assignments are unstable
                // (x2, y2) = (x2 + dx, y2 + dy);
                x2 += dx;
                y2 += dy;
            }
            // 如果当前位置已经是边界或障碍物，则回退一格
            x2 -= dx;
            y2 -= dy;
            if maze[x2 as usize][y2 as usize] == 0 {
                // 如果停下来的位置没有走过
                queue.push_back((x2, y2));
            }
        }
    }
    false
}

#[test]
fn test_has_path() {
    let test_cases = vec![
        (
            vec_vec![
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 0, 1, 1],
                [0, 0, 0, 0, 0]
            ],
            vec![0, 4],
            vec![4, 4],
            true,
        ),
        (
            vec_vec![
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 0, 1, 1],
                [0, 0, 0, 0, 0]
            ],
            vec![0, 4],
            vec![3, 2],
            false,
        ),
    ];
    for (maze, start, destination, can_reach) in test_cases {
        assert_eq!(has_path(maze, start, destination), can_reach);
    }
}
