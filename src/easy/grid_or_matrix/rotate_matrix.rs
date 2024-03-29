/*! https://leetcode.com/problems/rotate-image/

需求: 矩阵顺时针旋转90°(90 degrees (clockwise))

## 解法一: 先上下颠倒再转置矩阵 或 先transpose再reverse rows

matrix[::] = zip(*matrix[::-1])

之所以要用matrix[::]是为了避免Python shadowing重写绑定一个新的matrix局部变量
*/

/**
1 2 3
4 5 6
7 8 9
i=0,j=1,m[i][j]=2:
let top_left = 2;
m[0][1] = m[1][0];
*/
fn rotate(m: &mut Vec<Vec<i32>>) {
    let n = m.len();
    // 四个角落的元素原地转圈圈交换
    for i in 0..n / 2 {
        // 需要让i或j其中一个遍历到(n+1)/2去保证奇数情况下例如5*5最外圈也能遍历3次
        for j in 0..(n + 1) / 2 {
            let top_left = m[i][j];
            // 左上角 <= 左下角: 左上和左下关于⟍对角线镜像对称，所以(i,j)=>(n-1-j,i)
            m[i][j] = m[n - j - 1][i];
            // 左下角 <= 右下角: 左上和右下中心对称，所以(i,j)=>(n-1-i, n-1-j)
            m[n - j - 1][i] = m[n - 1 - i][n - 1 - j];
            // 右下角 <= 右上角: 左上和右上关于↗️对角线镜像对称，所以(i,j)=>(j, n-1-i)
            m[n - 1 - i][n - 1 - j] = m[j][n - 1 - i];
            // 右上角 <= 左上角
            m[j][n - 1 - i] = top_left;
        }
    }
}

#[test]
fn test_rotate() {
    #[rustfmt::skip]
    let test_cases = vec![(
        vec_vec![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ],
        vec_vec![
            [7, 4, 1],
            [8, 5, 2],
            [9, 6, 3]
        ],
    )];
    for (mut input, output) in test_cases {
        rotate(&mut input);
        assert_eq!(input, output);
    }
}
