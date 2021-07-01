//! https://leetcode-cn.com/problems/chuan-di-xin-xi/
fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut graph = [[false; 10]; 10];
    for each in relation {
        graph[each[0] as usize][each[1] as usize] = true;
    }

    let n = n as usize;
    let mut dfs_state = DfsState {
        ans: 0,
        n,
        destination: n - 1,
        max_steps: k as u8,
        graph,
    };

    dfs_state.dfs(0, 0);
    dfs_state.ans
}

struct DfsState {
    ans: i32,
    n: usize,
    /// n-1
    destination: usize,
    max_steps: u8,
    graph: [[bool; 10]; 10],
}

impl DfsState {
    fn dfs(&mut self, pos: usize, step: u8) {
        if step == self.max_steps {
            if pos == self.destination {
                self.ans += 1;
            }
            return;
        }
        for next_pos in 0..self.n {
            if self.graph[pos][next_pos] {
                self.dfs(next_pos, step + 1);
            }
        }
    }
}

#[test]
fn test_num_ways() {
    let test_cases = vec![
        (
            5,
            vec_vec![[0, 2], [2, 1], [3, 4], [2, 3], [1, 4], [2, 0], [0, 4]],
            3,
            3,
        ),
        (3, vec_vec![[0, 2], [2, 1]], 2, 0),
    ];
    for (n, relation, k, ans) in test_cases {
        assert_eq!(num_ways(n, relation, k), ans);
    }
}
