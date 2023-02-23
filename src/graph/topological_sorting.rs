/**
Detect Cycle in a Directed Graph methods:
1. topological sorting, if has circle, sort_result.len() < nodes.len()
2. DFS with memorize/visited
3. Union set
*/
/// https://leetcode.cn/problems/course-schedule/
/// https://leetcode.cn/problems/course-schedule-ii/
/// course_schedule: return course_schedule_ii().is_empty()
/// 108ms solutions, slow than python 52ms solutions
fn course_schedule_ii(num_nodes: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let num_nodes = num_nodes as usize;
    // step.1 build graph(node_id -> node's indegree) adjacency
    let mut nodes = vec![std::collections::HashSet::new(); num_nodes];
    for edge in edges {
        let edge_src = edge[1] as usize;
        let edge_dst = edge[0] as usize;
        nodes[edge_dst].insert(edge_src);
    }

    // step.2 iter/traverse init state
    //   directed graph: find node indegree==0 put to queue
    // undirected graph: find node indegree<=1 put to queue
    let mut queue = std::collections::VecDeque::new();
    for (node_id, node) in nodes.iter().enumerate() {
        if node.is_empty() {
            queue.push_back(node_id);
        }
    }

    let mut topological_order = Vec::new();
    while let Some(node_id) = queue.pop_front() {
        for (each_node_id, each_node) in nodes.iter_mut().enumerate() {
            // 删掉节点 node_id 以及其相连的边，如果删除后发现入度为零的点，则扔到队列尾部
            if each_node.remove(&node_id) && each_node.is_empty() {
                queue.push_back(each_node_id);
            }
        }
        topological_order.push(node_id as i32);
    }
    if topological_order.len() < nodes.len() {
        Vec::new()
    } else {
        topological_order
    }
}

#[test]
fn test_course_schedule_ii() {
    for (num_courses, prerequisites, topological_order) in [
        (2, vec_vec![[1, 0], [0, 1]], vec![]),
        (
            4,
            vec_vec![[1, 0], [2, 0], [3, 1], [3, 2]],
            vec![0, 1, 2, 3],
        ),
    ] {
        assert_eq!(
            course_schedule_ii(num_courses, prerequisites),
            topological_order
        );
    }
}

/// 0ms
fn topological_order_optimize_for_oj(num_nodes: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let num_nodes = num_nodes as usize;
    let mut nodes_indegree = vec![0; num_nodes];
    let mut outdegree = vec![Vec::new(); num_nodes];
    for edge in edges {
        let edge_src = edge[1] as usize;
        let edge_dst = edge[0] as usize;

        nodes_indegree[edge_dst] += 1;
        outdegree[edge_src].push(edge_dst);
    }

    let mut zero_indegree_queue = std::collections::VecDeque::new();
    for (node_id, indegree) in nodes_indegree.iter().enumerate() {
        if *indegree == 0 {
            zero_indegree_queue.push_back(node_id);
        }
    }

    let mut topological_order = Vec::new();
    while let Some(node_id) = zero_indegree_queue.pop_front() {
        for &each in &outdegree[node_id] {
            nodes_indegree[each] -= 1;
            if nodes_indegree[each] == 0 {
                zero_indegree_queue.push_back(each);
            }
        }
        topological_order.push(node_id as i32);
    }

    if topological_order.len() < nodes_indegree.len() {
        Vec::new()
    } else {
        topological_order
    }
}
