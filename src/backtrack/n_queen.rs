use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        return entrance(n);
    }
}

fn entrance(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    // 索引表示皇后的横坐标，值表示皇后的列坐标
    let mut queens: Vec<i32> = Vec::with_capacity(n);
    // 已使用的列号
    let mut used_cols: Vec<bool> = vec![false; n];
    // 存储直线方程x+y的常系数
    let mut sum: HashSet<i32> = HashSet::with_capacity(n);
    // 存储直线方程x-y的常系数
    let mut dif: HashSet<i32> = HashSet::with_capacity(n);

    let mut res: Vec<Vec<String>> = Vec::new();
    dfs(&mut queens, &mut used_cols, &mut sum, &mut dif, n, &mut res);
    return res;
}

fn dfs(
    queens: &mut Vec<i32>,
    used_cols: &mut Vec<bool>,
    sum: &mut HashSet<i32>,
    dif: &mut HashSet<i32>,
    n: usize,
    res: &mut Vec<Vec<String>>
) {
    if queens.len() == n {
        render_solution(queens, res, n);
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
        let cur_dif = x-y_i32;
        if dif.contains(&cur_dif) {
            continue;
        }

        // 搜索下一个位置
        sum.insert(cur_sum);
        dif.insert(cur_dif);
        used_cols[y] = true;
        queens.push(y_i32);
        dfs(queens, used_cols, sum, dif, n, res);
        queens.pop().unwrap();
        sum.remove(&cur_sum);
        dif.remove(&cur_dif);
        used_cols[y] = false;
    }
}

fn render_solution(queens: &Vec<i32>, res: &mut Vec<Vec<String>>, n: usize) {
    let mut board: Vec<String> = Vec::with_capacity(n);
    for i in 0..n {
        let mut row = vec!['.'.to_string(); n];
        row[queens[i] as usize] = 'Q'.to_string();
        board.push(row.join(""))
    }
    res.push(board);
}

#[test]
fn test() {
    dbg!(entrance(4));

}
