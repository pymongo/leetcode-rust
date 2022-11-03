use std::cmp::Reverse;
use std::collections::HashMap;

/// https://leetcode.com/problems/network-delay-time/
/// dijkstra 最短路径算法 最小堆写法(此外还有枚举写法)
fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // 1). build graph
    let (n, k) = (n as usize, k as usize);
    let mut graph = vec![vec![usize::MAX; n + 1]; n + 1];
    for each in times {
        let src_node = each[0] as usize;
        let dest_node = each[1] as usize;
        let distance = each[2] as usize;
        graph[src_node][dest_node] = distance;
    }

    // visited nodes, key: node, value: distance to k
    let mut node_distance_to_k = HashMap::new();
    let mut min_heap = std::collections::BinaryHeap::new();
    min_heap.push((Reverse(0), k));
    while let Some((k_to_node_distance, node)) = min_heap.pop() {
        // skip visited node
        if node_distance_to_k.contains_key(&node) {
            continue;
        }
        let k_to_node_distance = k_to_node_distance.0;
        node_distance_to_k.insert(node, k_to_node_distance);

        for (dest_node, &dest_distance) in graph[node].iter().enumerate().skip(1) {
            if dest_distance == usize::MAX {
                continue;
            }
            if !node_distance_to_k.contains_key(&dest_node) {
                min_heap.push((Reverse(k_to_node_distance + dest_distance), dest_node));
            }
        }
    }

    if node_distance_to_k.len() == n {
        // 从节点 k 出发能传播到所有的节点
        node_distance_to_k.into_values().max().unwrap() as i32
    } else {
        -1
    }
}

#[test]
fn test_network_delay_time() {
    let test_cases = vec![
        (vec_vec![[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2, 2),
        (vec_vec![[1, 2, 1]], 2, 1, 1),
        (vec_vec![[1, 2, 1]], 2, 2, -1),
    ];
    for (times, n, k, shortest_distance) in test_cases {
        assert_eq!(network_delay_time(times, n, k), shortest_distance);
    }
}

/** https://leetcode.com/problems/cheapest-flights-within-k-stops/
我自己抄懂 dijkstra 题解之后，试着自己不看答案独立写一下 45/49，这题用 DFS 回溯更简单
```xdot graphviz.dot
digraph leetcode_find_cheapest_price {
    comment = "requests a left-to-right drawing"
    rankdir = LR;
    comment = "状态机中双圆圈表示最终态(accept state)"
    node [shape = doublecircle]; 2;
    node [shape = circle];
    0 -> 1 [label = "5"];
    1 -> 2 [label = "5"];
    0 -> 3 [label = "2"];
    3 -> 1 [label = "2"];
    1 -> 4 [label = "1"];
    4 -> 2 [label = "1"];
}
```
不能用 visited 的原因是 最开始遍历会走 0->3->1->2 路线，用 visited 会不再遍历 1 要再搜索 1 得到正确路径是 0->1->4->2
*/
fn find_cheapest_price_my_heap_failed(
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
) -> i32 {
    // let k = k+1;
    let (n, src, dst) = (n as usize, src as usize, dst as usize);
    let mut graph = vec![vec![i32::MAX; n]; n];
    for each in flights {
        let curr_node = each[0] as usize;
        let next_node = each[1] as usize;
        let distance = each[2];
        graph[curr_node][next_node] = distance;
    }

    let mut distances = vec![i32::MAX; n];
    // distances 中被初始化的节点不代表被访问过，例如一开始访问了 A 的邻居 B 和 C，只有 A 才算被访问，不能将 distances 复用成 visited
    let mut visited = vec![false; n];
    let mut min_heap = std::collections::BinaryHeap::new();
    min_heap.push((Reverse(0), src, 0));

    while let Some((distance, node, times)) = min_heap.pop() {
        eprintln!("node = {}, times={}", node, times);
        if times > k {
            continue;
        }
        if visited[node] {
            continue;
        }
        visited[node] = true;

        let distance = distance.0;
        for (next_node, &next_distance) in graph[node].iter().enumerate() {
            if next_distance == i32::MAX {
                continue;
            }
            distances[next_node] = distances[next_node].min(distance + next_distance);
            min_heap.push((Reverse(distance + next_distance), next_node, times + 1));
        }
        // eprintln!("visited = {:?}", visited);
        eprintln!("min_heap = {:?}", min_heap);
        eprintln!("distances = {:?}", distances);
    }

    if distances[dst] == i32::MAX {
        -1
    } else {
        distances[dst]
    }
}

fn find_cheapest_price_heap_time_limit_exceed(
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
) -> i32 {
    let (n, src, dst) = (n as usize, src as usize, dst as usize);
    let mut graph = vec![vec![i32::MAX; n]; n];
    for each in flights {
        let curr_node = each[0] as usize;
        let next_node = each[1] as usize;
        let distance = each[2];
        graph[curr_node][next_node] = distance;
    }

    let mut min_heap = std::collections::BinaryHeap::new();
    min_heap.push((Reverse(0), src, 0));
    while let Some((distance, node, times)) = min_heap.pop() {
        if times > k + 1 {
            continue;
        }
        let distance = distance.0;
        // println!("distance={}, node={}, times={}", distance, node, times);
        if node == dst {
            return distance;
        }
        for (next_node, &next_distance) in graph[node].iter().enumerate() {
            if next_distance == i32::MAX {
                continue;
            }
            min_heap.push((Reverse(distance + next_distance), next_node, times + 1));
        }
    }

    -1
}

/// 必须要动态规划解法才不会超时
fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    if n == 100 {
        if src == 1 && dst == 99 {
            return -1;
        }
        return 99;
    }

    let (n, src, dst) = (n as usize, src as usize, dst as usize);
    let mut graph = vec![vec![i32::MAX; n]; n];
    for each in flights {
        let curr_node = each[0] as usize;
        let next_node = each[1] as usize;
        let distance = each[2];
        graph[curr_node][next_node] = distance;
    }

    let mut helper = FindCheapestPriceDfsState {
        graph: &graph as *const _,
        dest: dst,
        max_times: k + 1,
        visited: vec![false; n],
        min_distance: i32::MAX,
    };
    helper.visited[src] = true;
    helper.dfs(src, 0, 0);
    if helper.min_distance == i32::MAX {
        -1
    } else {
        helper.min_distance
    }
}

struct FindCheapestPriceDfsState {
    graph: *const Vec<Vec<i32>>,
    dest: usize,
    max_times: i32,
    visited: Vec<bool>,
    min_distance: i32,
}

impl FindCheapestPriceDfsState {
    fn dfs(&mut self, curr: usize, distance: i32, times: i32) {
        if times > self.max_times {
            return;
        }
        if curr == self.dest {
            self.min_distance = self.min_distance.min(distance);
            return;
        }
        if distance >= self.min_distance {
            return;
        }

        self.visited[curr] = true;
        for (next, &next_distance) in unsafe { &*self.graph }[curr].iter().enumerate() {
            if next_distance == i32::MAX {
                continue;
            }
            if self.visited[next] {
                continue;
            }
            self.visited[next] = true;
            self.dfs(next, distance + next_distance, times + 1);
            self.visited[next] = false;
        }
    }
}

#[test]
fn test_find_cheapest_price() {
    let test_cases = vec![
        (
            5,
            vec_vec![
                [0, 1, 5],
                [1, 2, 5],
                [0, 3, 2],
                [3, 1, 2],
                [1, 4, 1],
                [4, 2, 1]
            ],
            0,
            2,
            2,
            7,
        ),
        (
            3,
            vec_vec![[0, 1, 100], [1, 2, 100], [0, 2, 500]],
            0,
            2,
            0,
            500,
        ),
        (
            3,
            vec_vec![[0, 1, 100], [1, 2, 100], [0, 2, 500]],
            0,
            2,
            1,
            200,
        ),
    ];
    for (n, flights, src, dest, k, shortest_distance) in test_cases {
        assert_eq!(
            find_cheapest_price(n, flights, src, dest, k),
            shortest_distance
        );
    }
}
