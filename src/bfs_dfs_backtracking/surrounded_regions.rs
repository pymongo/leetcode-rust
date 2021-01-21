struct BFSHelper {
    max_x: usize,
    max_y: usize,
}

impl BFSHelper {
    fn bfs(&self, i: usize, j: usize, board: &mut Vec<Vec<char>>) {
        let mut stack = vec![(i, j)];
        while let Some((x, y)) = stack.pop() {
            board[x][y] = 'F';
            if x < self.max_x && board[x + 1][y] == 'O' {
                stack.push((x + 1, y));
            }
            if x > 0 && board[x - 1][y] == 'O' {
                stack.push((x - 1, y));
            }
            if y < self.max_y && board[x][y + 1] == 'O' {
                stack.push((x, y + 1));
            }
            if y > 0 && board[x][y - 1] == 'O' {
                stack.push((x, y - 1));
            }
        }
    }
}

/// https://leetcode.com/problems/surrounded-regions/
/// 从边界出发吧，先把边界上和O连通点找到，把这些变成F
/// 最后遍历整个 board 把 O 变成 X, 把 F 变成 O
fn surrounded_regions(board: &mut Vec<Vec<char>>) {
    if board.is_empty() {
        return;
    }
    let (m, n) = (board.len(), board[0].len());
    let bfs_helper = BFSHelper {
        max_x: m - 1,
        max_y: n - 1,
    };

    // 从边界出发吧，先把边界上和O连通点找到，标记为不合格的F
    for j in 0..n {
        // 第一行
        if board[0][j] == 'O' {
            bfs_helper.bfs(0, j, board);
        }
        // 最后一行
        if board[m - 1][j] == 'O' {
            bfs_helper.bfs(m - 1, j, board);
        }
    }
    for i in 1..m - 1 {
        // 第一列
        if board[i][0] == 'O' {
            bfs_helper.bfs(i, 0, board);
        }
        // 最后一列
        if board[i][n - 1] == 'O' {
            bfs_helper.bfs(i, n - 1, board);
        }
    }

    for row in board.iter_mut().take(m) {
        for ch in row.iter_mut().take(n) {
            if *ch == 'O' {
                *ch = 'X';
            } else if *ch == 'F' {
                *ch = 'O';
            }
        }
    }
}

#[test]
fn test_surrounded_regions() {
    let test_cases = vec![
        (
            vec_vec![
                ['X', 'X', 'X', 'X'],
                ['X', 'O', 'O', 'X'],
                ['X', 'X', 'O', 'X'],
                ['X', 'O', 'X', 'X']
            ],
            vec_vec![
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'O', 'X', 'X']
            ],
        ),
        (
            vec_vec![['X', 'O', 'X'], ['X', 'O', 'X'], ['X', 'O', 'X'],],
            vec_vec![['X', 'O', 'X'], ['X', 'O', 'X'], ['X', 'O', 'X'],],
        ),
        (
            vec_vec![
                ['O', 'O', 'O', 'O', 'X', 'X'],
                ['O', 'O', 'O', 'O', 'O', 'O'],
                ['O', 'X', 'O', 'X', 'O', 'O'],
                ['O', 'X', 'O', 'O', 'X', 'O'],
                ['O', 'X', 'O', 'X', 'O', 'O'],
                ['O', 'X', 'O', 'O', 'O', 'O']
            ],
            vec_vec![
                ['O', 'O', 'O', 'O', 'X', 'X'],
                ['O', 'O', 'O', 'O', 'O', 'O'],
                ['O', 'X', 'O', 'X', 'O', 'O'],
                ['O', 'X', 'O', 'O', 'X', 'O'],
                ['O', 'X', 'O', 'X', 'O', 'O'],
                ['O', 'X', 'O', 'O', 'O', 'O']
            ],
        ),
    ];
    for (mut input, output) in test_cases {
        surrounded_regions(&mut input);
        assert_eq!(input, output);
    }
}

/// 解题思路没问题但是最后一个测试用例超时
#[cfg(not)]
fn solve(board: &mut Vec<Vec<char>>) {
    if board.is_empty() {
        return;
    }
    let (m, n) = (board.len(), board[0].len());
    let mut queue = std::collections::VecDeque::new();
    let mut need_to_replace = std::collections::HashSet::new();
    for i in 1..m - 1 {
        'next_start_point: for j in 1..n - 1 {
            if board[i][j] != 'O' {
                continue;
            }
            if need_to_replace.contains(&(i, j)) {
                continue;
            }
            let mut seen = std::collections::HashSet::new();
            queue.push_back((i, j));
            while let Some((x, y)) = queue.pop_front() {
                seen.insert((x, y));
                for &(next_x, next_y) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                    if next_x == 0 || next_x == m - 1 || next_y == 0 || next_y == n - 1 {
                        if board[next_x][next_y] == 'O' {
                            // 如果由O组成的岛屿一直延伸到边界，则不能被围棋X吃掉
                            queue.clear();
                            continue 'next_start_point;
                        }
                        continue;
                    }
                    if seen.contains(&(next_x, next_y)) {
                        continue;
                    }
                    if board[next_x][next_y] == 'O' {
                        queue.push_back((next_x, next_y));
                    }
                }
            }
            need_to_replace.extend(seen.into_iter());
        }
    }
    for (i, j) in need_to_replace.into_iter() {
        board[i][j] = 'X';
    }
}
