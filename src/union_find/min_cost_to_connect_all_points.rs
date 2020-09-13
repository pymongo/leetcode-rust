struct Solution;

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parents = Vec::with_capacity(n);
        for i in 0..n {
            parents.push(i);
        }
        UnionFind { parents }
    }

    fn find_root(&self, node: usize) -> usize {
        let mut curr_node = node;
        while self.parents[curr_node] != curr_node {
            curr_node = self.parents[curr_node];
        }
        return self.parents[curr_node];
    }

    // 添加一条node_a连向node_b边
    fn union(&mut self, node_a: usize, node_b: usize) {
        // 路径压缩: 不要直接将b连到a上，而是将b的祖先连向a的祖先，以此压缩路径减少连边
        let root_a = Self::find_root(self, node_a);
        let root_b = Self::find_root(self, node_b);
        if root_a != root_b {
            // 将b的祖先挂载到a的祖先下
            self.parents[root_b] = root_a;
        }
    }
}

impl Solution {
    fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in i + 1..n {
                edges.push(((points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(), i, j));
            }
        }
        edges.sort_unstable();
        let mut total_cost = 0;
        let mut union_find = UnionFind::new(n);
        let mut used_edges = 0;
        for (val, node_a, node_b) in edges {
            let root_a = union_find.find_root(node_a);
            let root_b = union_find.find_root(node_b);
            if root_a == root_b {
                continue;
            }
            total_cost += val;
            union_find.parents[root_b] = root_a;
            used_edges += 1;
            if used_edges == n-1 {
                break;
            }
        }
        return total_cost;
    }
}

#[cfg(test)]
fn test_cases() -> Vec<(Vec<Vec<i32>>, i32)> {
    vec![
        (
            vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]],
            20,
        ),
        (
            vec![vec![2, -3], vec![-17, -8], vec![13, 8], vec![-17, -15]],
            53,
        ),
        (vec![vec![3, 12], vec![-2, 5], vec![-4, 1]], 18),
        (vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]], 4),
    ]
}

#[test]
fn test() {
    for (points, min_cost) in test_cases() {
        assert_eq!(Solution::min_cost_connect_points(points), min_cost);
    }
}
