/// https://leetcode.com/problems/4sum-ii/
fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut pairs = std::collections::HashMap::new();
    for num_a in a {
        for num_b in &b {
            *pairs.entry(num_a + num_b).or_default() += 1;
        }
    }
    let mut count = 0;
    for num_c in c {
        for num_d in &d {
            count += pairs.get(&(-num_c - num_d)).unwrap_or(&0);
        }
    }
    count
}
