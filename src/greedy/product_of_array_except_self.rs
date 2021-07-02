/// https://leetcode.com/problems/product-of-array-except-self
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // 右上三角，从下到上，从右到左累乘的结果
    let mut top_right = 1;
    // 左下三角，从上到下，从左到右累乘的结果
    let mut bottom_left = 1;

    let n = nums.len();
    let mut ret = vec![1; n];
    for i in (0..n).rev() {
        ret[i] *= top_right;
        top_right *= nums[i];
    }
    for i in 0..n {
        ret[i] *= bottom_left;
        bottom_left *= nums[i];
    }
    ret
}

/// https://leetcode.com/problems/gou-jian-cheng-ji-shu-zu-lcof
/// 这题不看答案我真没想到累积「上下角」和累积「下三角」
/// 注意这是超时解答
#[allow(clippy::needless_range_loop)]
fn construct_product_array(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut ret = vec![1; n];
    // 右上三角区域的乘积运算
    for i in 0..n {
        for j in i + 1..n {
            // dbg!((i, j, nums[j]));
            ret[i] *= nums[j];
        }
    }

    // dbg!("bottom-left corner");
    // 左下三角区域的乘积运算
    for i in 0..n {
        for j in 0..i {
            ret[i] *= nums[j];
        }
    }
    ret
}

/** T表示一行中需要累乘的元素
  1 2 3 4
1   T T T
2 T   T T
3 T T   T
4 T T T
*/
#[test]
fn test_construct_product_array() {
    const TEST_CASES: [(&[i32], &[i32]); 3] = [
        (&[1, 2, 3, 4], &[24, 12, 8, 6]),
        (&[1, 0, 3, 4], &[0, 12, 0, 0]),
        (&[1, 0, 3, 0], &[0, 0, 0, 0]),
    ];

    for &(input, output) in &TEST_CASES {
        assert_eq!(construct_product_array(input), output.to_vec());
    }
}
