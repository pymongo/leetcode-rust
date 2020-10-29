struct Solution;

impl Solution {
    #[allow(clippy::many_single_char_names)]
    pub fn count_good_triplets(a: Vec<i32>, q: i32, w: i32, e: i32) -> i32 {
        let mut res = 0;
        let n = a.len();
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if (a[i] - a[j]).abs() <= q
                        && (a[j] - a[k]).abs() <= w
                        && (a[i] - a[k]).abs() <= e
                    {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}
