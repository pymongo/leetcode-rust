struct Solution;

impl Solution {
    // 从1-9中找出k个不重复的组合，使得它们的和等于target
    fn combination_sum3(k: i32, target: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut cur = Vec::with_capacity(k);
        let mut res = Vec::new();
        Self::helper(1, target, &mut cur, &mut res, k);
        res
    }

    fn helper(
        start: i32,
        target: i32,
        cur: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        k: usize,
    ) {
        if cur.len() == k {
            if target == 0 {
                res.push(cur.clone());
            }
            return;
        }
        for num in start..=9 {
            if num > target {
                return;
            }
            cur.push(num);
            Self::helper(num+1, target-num, cur, res, k);
            cur.pop();
        }
    }
}

#[test]
fn test() {
    let res = Solution::combination_sum3(3, 7);
    for row in res {
        println!("{:?}", row);
    }
}