/// https://leetcode.com/problems/gas-station/
fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let (mut start, mut oil_remain) = (0, 0);
    let (mut total_gas, mut total_cost) = (0, 0);
    for i in 0..gas.len() {
        total_gas += gas[i];
        total_cost += cost[i];
        oil_remain += gas[i] - cost[i];
        /* 从起点start=a出发，到不了c站，说明从a到c的任意一个站b作为出发点，也到不了c，证明如下
        假设a,b,c相连，则有关系式: gas[a]-cost[a] + gas[b]-cost[b] + gas[c]-cost[c] < 0
        由于a可以作为出发点: gas[b]-cost[b] + gas[c]-cost[c] < 0
        由于b可以作为出发点，所以gas[b]-cost[b]也是大于0，但是仍然不足以走到c
        所以a出发到不了c这就排除掉从a到c所有点作为出发点的可能性，可以将出发点假设成c的后一个点
        */
        if oil_remain < 0 {
            start = i as i32 + 1;
            oil_remain = 0;
        }
    }
    if total_gas < total_cost {
        return -1;
    }
    start
}

#[test]
fn test_can_complete_circuit() {
    const TEST_CASES: [(&[i32], &[i32], i32); 2] = [
        (&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2], 3),
        (&[2, 3, 4], &[3, 4, 3], -1),
    ];

    for (gas, cost, start_index) in TEST_CASES {
        assert_eq!(
            can_complete_circuit(gas.to_vec(), cost.to_vec()),
            start_index
        );
    }
}
