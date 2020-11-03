struct Solution;

impl Solution {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut cur: Vec<i32> = Vec::with_capacity(k);
        let mut res: Vec<Vec<i32>> = Vec::new();
        Solution::dfs(&mut res, &mut cur, 1, n, k);
        res
    }

    fn dfs(res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, start: i32, n: i32, k: usize) {
        if cur.len() == k {
            res.push(cur.clone());
            cur.clear();
            return;
        }
        for i in start..=n {
            cur.push(i);
            Solution::dfs(res, cur, i + 1, n, k);
            cur.pop();
        }
    }
}
