/// https://leetcode.com/problems/fair-candy-swap/
/// 其实就是two_sum两数之差的变种题
fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let a_sum: i32 = a.iter().sum();
    let b_sum: i32 = b.iter().sum();
    let avg_sum = (a_sum + b_sum) / 2;
    // a_sum + b - a = avg_sum
    // b - a = target
    let target = avg_sum - a_sum;
    // 现在就转为两数之差(two sum)类解法了
    let mut map = std::collections::HashMap::with_capacity(b.len());
    for b in b {
        map.insert(b - target, b);
    }
    for a in a {
        if let Some(b) = map.get(&a) {
            return vec![a, *b];
        }
    }
    unreachable!()
}

#[test]
fn test_fair_candy_swap() {
    const TEST_CASES: [(&[i32], &[i32], &[i32]); 4] = [
        (&[1, 1], &[2, 2], &[1, 2]),
        (&[1, 2], &[2, 3], &[1, 2]),
        (&[2], &[1, 3], &[2, 3]),
        (&[1, 2, 5], &[2, 4], &[5, 4]),
    ];
    for &(a, b, output) in &TEST_CASES {
        assert_eq!(fair_candy_swap(a.into(), b.into()), output.to_vec());
    }
}
