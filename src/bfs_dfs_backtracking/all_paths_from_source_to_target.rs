/// https://leetcode.com/problems/all-paths-from-source-to-target/

struct AllPathsSourceTargetHelper {
    graph: *const Vec<Vec<i32>>,
    dest: i32,
    cur: Vec<i32>,
    paths: Vec<Vec<i32>>,
}

impl AllPathsSourceTargetHelper {
    fn dfs(&mut self, cur: i32) {
        if cur == self.dest {
            self.paths.push(self.cur.clone());
            return;
        }
        for &next in &unsafe { &*self.graph }[cur as usize] {
            self.cur.push(next);
            self.dfs(next);
            self.cur.pop().unwrap();
        }
    }
}

/// https://leetcode.com/problems/all-paths-from-source-to-target/
fn all_paths_source_target_my_best(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let dest = graph.len() as i32 - 1;
    let mut helper = AllPathsSourceTargetHelper {
        graph: &graph as *const _,
        dest,
        cur: vec![0],
        paths: Vec::new(),
    };
    helper.dfs(0);
    helper.paths
}

/// 由于Rust闭包不能或者很难递归调用，不用闭包又不能捕获外部作用域的变量，不能像python那样方便的内层定义的方法能随意读写外层的变量
/// 导致Rust写回溯算法的题非常困难，dfs函数入参太多，一定要头脑清醒从一开始就想清楚宏观的需要几个入参，哪些入参要回溯
/// 如果用结构体去重构超多入参的dfs函数，会导致不能以更细粒度的状态进行回溯，结构体不能更细粒度和高性能的记住上一个状态
fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let graph: Vec<Vec<usize>> = graph
        .into_iter()
        .map(|each| each.into_iter().map(|x| x as usize).collect())
        .collect();
    let n = graph.len();
    let mut curr = Vec::with_capacity(n);
    let mut ret = Vec::new();
    let mut visited = vec![false; n];
    // for start in 0..n {
    curr.push(0);
    visited[0] = true;
    dfs(&mut curr, &mut visited, &mut ret, &graph, n - 1);
    //     curr.pop().unwrap();
    //     visited[start] = false;
    // }
    ret.into_iter()
        .map(|each| each.into_iter().map(|x| x as i32).collect())
        .collect()
}

fn dfs(
    curr: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    ret: &mut Vec<Vec<usize>>,
    graph: &[Vec<usize>],
    dest: usize,
) {
    for &next in &graph[*curr.last().unwrap()] {
        if visited[next] {
            continue;
        }
        curr.push(next);
        if next == dest {
            ret.push(curr.clone());
            curr.pop().unwrap();
            continue;
        }
        visited[next] = true;
        dfs(curr, visited, ret, graph, dest);
        curr.pop().unwrap();
        visited[next] = false;
    }
}

#[test]
fn test_all_paths_source_target() {
    // 入参graph的数据格式是邻接表，graph[0]表示节点0的连向节点1和节点2
    let test_cases = vec![(
        vec_vec![[1, 2], [3], [3], []],
        vec_vec![[0, 1, 3], [0, 2, 3]],
    )];
    for (input, output) in test_cases {
        assert_eq!(all_paths_source_target(input), output);
    }
}
