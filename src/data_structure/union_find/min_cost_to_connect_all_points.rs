use super::UnionFind;

/// https://leetcode.com/problems/min-cost-to-connect-all-points/
/// 平面上有若干点，找出能连接所有点的最短边的总和
/// 除了并查集排除重复连边，还能用「最小生成树的模板」的Prim或Kruskal算法
fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for start in 0..n {
        for end in start + 1..n {
            edges.push((
                (points[start][0] - points[end][0]).abs()
                    + (points[start][1] - points[end][1]).abs(),
                start,
                end,
            ));
        }
    }
    edges.sort_unstable();
    let mut total_cost = 0;
    let mut union_find = UnionFind::new(n);
    let mut used_edges = 0;
    for (cost, node_a, node_b) in edges {
        let root_a = union_find.find_root(node_a);
        let root_b = union_find.find_root(node_b);
        if root_a == root_b {
            continue;
        }
        // 如果a和b不相连，则添加一条node_a连向node_b边
        union_find.parents[root_b] = root_a;
        total_cost += cost;
        used_edges += 1;
        if used_edges == n - 1 {
            break;
        }
    }
    total_cost
}

#[test]
fn test_min_cost_connect_points() {
    let test_cases = vec![
        (vec_vec![[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]], 20),
        (vec_vec![[3, 12], [-2, 5], [-4, 1]], 18),
        (vec_vec![[0, 0], [1, 1], [1, 0], [-1, 1]], 4),
        (vec_vec![[2, -3], [-17, -8], [13, 8], [-17, -15]], 53),
    ];
    for (points, min_cost) in test_cases {
        assert_eq!(min_cost_connect_points(points), min_cost);
    }
}
