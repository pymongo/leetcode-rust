struct Solution;

impl Solution {
    fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .filter(|(&start, &end)| start <= query_time && query_time <= end)
            .count() as i32
    }
}

#[test]
fn test_busy_student() {
    assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
}
