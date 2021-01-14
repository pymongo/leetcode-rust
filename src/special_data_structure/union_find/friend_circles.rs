/// https://leetcode.com/problems/number-of-provinces/
/// https://leetcode.com/problems/friend-circles/
/// 以前这题叫「朋友圈」，不知道为什么改名为「省份数量」，其实就是数邻接矩阵中相连岛屿的数量
use super::UnionFind;

/// 入参is_connected是一个图的邻接矩阵
fn union_find(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut uf = UnionFind::new(n);
    for (i, row) in is_connected.into_iter().enumerate() {
        for (j, each) in row.into_iter().enumerate() {
            if each == 1 {
                uf.union(i, j);
            }
        }
    }

    // 数一下并查集中有几棵树
    let mut ret = 0;
    for (i, parent) in uf.parents.into_iter().enumerate() {
        // 并查集中parent==i表示节点i自己就是根节点
        if parent == i {
            ret += 1;
        }
    }
    ret
}

fn bfs(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::with_capacity(n);
    let mut ret = 0;
    for start in 0..n {
        if visited[start] {
            continue;
        }
        queue.push_back(start);
        while let Some(cur) = queue.pop_front() {
            visited[cur] = true;
            for (next, &connected) in is_connected[cur].iter().enumerate().take(n) {
                if connected == 0 || visited[next] {
                    continue;
                }
                queue.push_back(next);
            }
        }
        ret += 1;
    }
    ret
}

#[test]
fn test_friends_circle() {
    let test_cases = vec![
        (vec_vec![[1, 1, 0], [1, 1, 0], [0, 0, 1]], 2),
        (vec_vec![[1, 1, 0], [1, 1, 1], [0, 1, 1]], 1),
    ];
    for (input, output) in test_cases.into_iter() {
        assert_eq!(union_find(input.clone()), output);
        assert_eq!(bfs(input.clone()), output);
    }
}
