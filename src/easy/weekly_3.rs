struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut cost_map: HashMap<(usize, usize), i32> = HashMap::with_capacity(n*(n-1)/2);
        for i in 0..n {
            for j in i + 1..n {
                cost_map.insert(
                    (i, j),
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(),
                );
            }
        }
        println!("{:?}", cost_map);

        let mut res = 0;
        let mut used = vec![false; n];
        // 类似最短路径算法，每一步都选最优解(最短)的点，然后整体就是最优的
        for i in 0..n-1 {
            if used[i] {
                continue;
            }
            let mut cur_min_cost = std::i32::MAX;
            let mut cur_min_idx = n+1;
            for j in 1..n {
                if j == i {
                    continue;
                }
                if used[j] {
                    continue;
                }
                let cur_cost = if let Some(val) = cost_map.get(&(i, j)) {
                    *val
                } else {
                    *cost_map.get(&(j, i)).unwrap()
                };
                if cur_cost < cur_min_cost {
                    cur_min_idx = j;
                    cur_min_cost = cur_cost;
                }
            }
            // 如果所有点都访问过了
            if cur_min_cost == std::i32::MAX {
                break;
            } else {
                used[i] = true;
                used[cur_min_idx] = true;
                dbg!(cur_min_cost);
                res += cur_min_cost;
            }
        }
        res
    }


}

#[test]
fn test() {
    let points1 = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    dbg!(Solution::min_cost_connect_points(points1));
}
