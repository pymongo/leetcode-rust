/// https://leetcode.com/problems/4sum-ii/
fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut pairs = std::collections::HashMap::new();
    for num_a in a.into_iter() {
        for num_b in b.iter() {
            *pairs.entry(num_a + num_b).or_default() += 1;
        }
    }
    let mut count = 0;
    for num_c in c.into_iter() {
        for num_d in d.iter() {
            count += pairs.get(&(-num_c - num_d)).unwrap_or(&0);
        }
    }
    count
}
