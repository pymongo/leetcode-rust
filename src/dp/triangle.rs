/// https://leetcode.com/problems/triangle/
/// 求从顶部到底层的最短路径，注意函数名的top_bottom并不是递归/分治法的top_down/bottom_up的含义，函数名仅仅表示遍历输入二维数组的方向
fn from_top_to_bottom(mut triangle: Vec<Vec<i32>>) -> i32 {
    let depth = triangle.len();
    if depth < 2 {
        return triangle[0][0];
    }
    for i in 1..depth {
        // 每层的最左节点只能从上一层的最左节点过来，当前层的最右节点也是只能从上层最右过来
        triangle[i][0] += triangle[i - 1][0];
        triangle[i][i] += triangle[i - 1][i - 1];

        for j in 1..i {
            triangle[i][j] += triangle[i - 1][j - 1].min(triangle[i - 1][j]);
        }
    }
    *triangle.last().unwrap().iter().min().unwrap()
}

/**
```python
def dp_bottom_to_top(triangle: List[List[int]]) -> int:
    for i in range(len(triangle) - 2, -1, -1):
        for j in range(i + 1):
            triangle[i][j] = triangle[i][j] + min(triangle[i + 1][j], triangle[i + 1][j + 1])
    return triangle[0][0]
```
*/
fn from_bottom_to_top(mut triangle: Vec<Vec<i32>>) -> i32 {
    for i in (0..triangle.len() - 1).rev() {
        for j in 0..=i {
            // 第i层最后一个下标是i
            triangle[i][j] += triangle[i + 1][j].min(triangle[i + 1][j + 1]);
        }
    }
    triangle[0][0]
}

#[test]
fn test() {
    let test_cases = vec![(vec_vec![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]], 11)];
    for (input, output) in test_cases.into_iter() {
        assert_eq!(from_top_to_bottom(input.clone()), output);
        assert_eq!(from_bottom_to_top(input), output);
    }
}
