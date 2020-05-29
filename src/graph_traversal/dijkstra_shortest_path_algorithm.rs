//! https://leetcode.com/problems/network-delay-time/
//! ## 题目描述
//! 从一个有向图的某个节点发出一个信号，需要多长时间才能让所有能收到信号的节点接收到信号
//! 这个过程有点像物理上的扩散现象，耗时等于最后一个收到信息的节点的耗时
//! 可以将该问题转化为：出发点K到其他各点的最短距离 中的最大值，就是最晚一个收到信号的节点

#[cfg(test)]
const TEST_CASES: [(&[&[i32]], i32, i32, i32); 1] =
    [(&[&[2, 1, 1], &[2, 3, 1], &[3, 4, 1]], 4, 2, 2)];

#[test]
fn test_network_delay_time() {
    for case in &TEST_CASES {
        let times: Vec<Vec<i32>> = case
            .0
            .iter()
            .map(|each| each.iter().cloned().collect())
            .collect();
        assert_eq!(network_delay_time(times, case.1, case.2), case.3);
    }
}

#[cfg(test)]
fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // unimplemented!()
    0_i32
}
