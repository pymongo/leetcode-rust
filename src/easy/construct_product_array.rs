/// https://leetcode.com/problems/gou-jian-cheng-ji-shu-zu-lcof
#[allow(clippy::needless_range_loop)]
fn construct_product_array(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![1; n];
    // 右上三角区域的乘积运算
    for i in 0..n {
        for j in i + 1..n {
            // dbg!((i, j, nums[j]));
            res[i] *= nums[j];
        }
    }

    // dbg!("bottom-left corner");
    // 左下三角区域的乘积运算
    for i in 0..n {
        for j in 0..i {
            res[i] *= nums[j];
        }
    }
    res
}

/*
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
        assert_eq!(construct_product_array(input), output.to_vec())
    }
}
