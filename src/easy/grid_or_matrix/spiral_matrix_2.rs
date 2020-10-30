struct Solution;

/// 按螺旋遍历生成n*n的矩阵
impl Solution {
    #[allow(clippy::many_single_char_names)]
    fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            // 避免r=0时r--发生usize溢出
            return vec![vec![1]];
        }
        let target = n * n;
        let n = n as usize;
        let mut m = vec![vec![0; n]; n];
        // 跟spiral_matrix_1一样，l/r/t/b分别表示left/right/top/bottom
        let (mut l, mut r, mut t, mut b) = (0, n - 1, 0, n - 1);
        let mut num = 1;
        while num <= target {
            for j in l..=r {
                m[t][j] = num;
                num += 1;
            }
            t += 1;
            for i in t..=b {
                m[i][r] = num;
                num += 1;
            }
            r -= 1;
            for j in (l..=r).rev() {
                m[b][j] = num;
                num += 1;
            }
            b -= 1;
            for i in (t..=b).rev() {
                m[i][l] = num;
                num += 1;
            }
            l += 1;
        }
        return m;
    }
}

#[test]
fn test() {
    let n: usize = 5;
    let res = Solution::generate_matrix(n as i32);
    for i in 0..n {
        println!("{:?}", res[i]);
    }
}
