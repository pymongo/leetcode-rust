struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut res = 0;
        for i in 1..n {
            let dx = (points[i][0] - points[i-1][0]).abs();
            let dy = (points[i][1] - points[i-1][1]).abs();
            res += dx.max(dy);
        }
        res
    }
}
