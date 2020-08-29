struct Solution;

impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (a.len(), a[0].len());
        let mut res = Vec::with_capacity(n);
        for j in 0..n {
            let mut row = Vec::with_capacity(m);
            for i in 0..m {
                row.push(a[i][j]);
            }
            res.push(row);
        }
        res
    }
}
