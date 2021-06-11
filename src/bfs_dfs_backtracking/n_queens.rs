use std::collections::HashSet;

/// https://leetcode.com/problems/n-queens-ii
fn n_queens_ii(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    // 索引表示皇后的横坐标，值表示皇后的列坐标
    let mut queens: Vec<i32> = Vec::with_capacity(n);
    // 已使用的列号
    let mut used_cols: Vec<bool> = vec![false; n];
    // 存储直线方程x+y的常系数
    let mut sum: HashSet<i32> = HashSet::with_capacity(n);
    // 存储直线方程x-y的常系数
    let mut dif: HashSet<i32> = HashSet::with_capacity(n);

    let mut ret: Vec<Vec<String>> = Vec::new();
    dfs(&mut queens, &mut used_cols, &mut sum, &mut dif, n, &mut ret);
    ret
}

fn dfs(
    queens: &mut Vec<i32>,
    used_cols: &mut Vec<bool>,
    sum: &mut HashSet<i32>,
    dif: &mut HashSet<i32>,
    n: usize,
    ret: &mut Vec<Vec<String>>,
) {
    if queens.len() == n {
        render_solution(queens, ret, n);
        return;
    }
    let x = queens.len() as i32;

    for y in 0..n {
        // 验证位置
        if used_cols[y] {
            continue;
        }
        let y_i32 = y as i32;
        let cur_sum = x + y_i32;
        if sum.contains(&cur_sum) {
            continue;
        }
        let cur_dif = x - y_i32;
        if dif.contains(&cur_dif) {
            continue;
        }

        // 搜索下一个位置
        sum.insert(cur_sum);
        dif.insert(cur_dif);
        used_cols[y] = true;
        queens.push(y_i32);
        dfs(queens, used_cols, sum, dif, n, ret);
        queens.pop().unwrap();
        sum.remove(&cur_sum);
        dif.remove(&cur_dif);
        used_cols[y] = false;
    }
}

fn render_solution(queens: &[i32], res: &mut Vec<Vec<String>>, n: usize) {
    let mut board: Vec<String> = Vec::with_capacity(n);
    for i in 0..n {
        let mut row = vec!['.'.to_string(); n];
        row[queens[i] as usize] = 'Q'.to_string();
        board.push(row.join(""));
    }
    res.push(board);
}
