struct Solution;

// 螺旋式遍历矩阵，从外圈遍历到内
impl Solution {
    pub fn spiral_order(a: Vec<Vec<i32>>) -> Vec<i32> {
        let m = a.len();
        if m == 0 {
            return Vec::new();
        }
        if m == 1 {
            return a[0].clone();
        }
        let n = a[0].len();
        if n == 1 {
            let mut res = Vec::new();
            for row in 0..m {
                res.push(a[row][0]);
            }
            return res;
        }

        let mut res = Vec::new();
        // 当前蛇形圈的边界(a[top][left], a[top][right], a[bottom][left], a[bottom][right])
        let (mut left, mut right, mut top, mut bottom) = (0, n - 1, 0, m - 1);
        while left <= right && top <= bottom {
            // Step.1: 从左上扫到右上，参考图(我的解法中底下那行扫的个数不同): https://assets.leetcode-cn.com/solution-static/54/54_fig1.png
            for col in left..=right {
                res.push(a[top][col]);
            }
            // Step.2: 从右上角扫到右下角
            for row in top + 1..=bottom - 1 {
                res.push(a[row][right]);
            }
            // Step.3: 从右下角扫到左下角
            for col in (left..=right).rev() {
                res.push(a[bottom][col]);
            }
            // Step.4: 从右下角扫到右上角
            for row in (top + 1..=bottom - 1).rev() {
                res.push(a[row][left]);
            }
            left += 1;
            top += 1;
            right -= 1;
            bottom -= 1;
        }

        // 去掉最后一趟转圈圈时会多加进去的元素(建议学leetcode官方解答中对最后一圈遍历的处理只扫第一行，我这么写不太好)
        for _ in 0..res.len() - m * n {
            res.pop();
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}
