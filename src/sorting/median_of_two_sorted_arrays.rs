//! 2 Solutions

#[cfg(test)]
const TEST_CASES: [(&[i32], &[i32], f64); 1] = [(&[1, 2, 3, 4], &[3, 6, 8, 9], 3.5)];

#[test]
fn test_move_divider_of_two_arrays() {
    for case in &TEST_CASES {
        let nums1: Vec<i32> = case.0.iter().cloned().collect();
        let nums2: Vec<i32> = case.1.iter().cloned().collect();
        assert_eq!(move_divider_of_two_arrays(nums1, nums2), case.2);
    }
}

/*
https://www.youtube.com/watch?v=ScCg9v921ns
数组A、B初始化时都在A、B中间设一个分割线
先不考虑奇数的情况，让代码简单点，自己尝试敲一下，不然干看答案也不理解
初始化的分割线如图
固定输入用例1：[1 2|3 4]
固定输入用例2：[3 6|8 9]
交叉比较：如果同时满足 a_mid_left(2) <= b_mid_right(8)
                && b_mid_left(6) <= a_mid_left(3)
则遍历结束，输出答案
否则移动分割线到如下位置去满足条件，利用两个数组是有序，得出上述交叉比较的遍历终止条件
移动分割线后输入用例1：[1 2 3|4]
移动分割线后输入用例2：[3|6 8 9]，刚好两个数组分割线的左半边组成了合并后中位数的左半边
*/
#[cfg(test)]
fn move_divider_of_two_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let ans: f64;
    let (a_len, b_len) = (nums1.len(), nums2.len());
    let (mut a_mid_left, mut a_mid_right) = ((a_len / 2) - 1, a_len / 2);
    let (mut b_mid_left, mut b_mid_right) = ((b_len / 2) - 1, b_len / 2);
    loop {
        if nums1[a_mid_left] > nums2[b_mid_right] {
            // a的右半边太大了！b的分割线左移一位、a的分割线右移一位
            a_mid_left -= 1;
            a_mid_right -= 1;
            b_mid_left += 1;
            b_mid_right += 1;
        } else if nums2[b_mid_left] > nums1[a_mid_right] {
            // b的右半边太大了！b的分割线左移一位、a的分割线右移一位
            b_mid_left -= 1;
            b_mid_right -= 1;
            a_mid_left += 1;
            a_mid_right += 1;
        } else {
            // calc answer
            ans = (std::cmp::max(nums1[a_mid_left], nums2[b_mid_left])
                + std::cmp::min(nums1[a_mid_right], nums2[b_mid_right])) as f64
                / 2 as f64;
            break;
        }
    }
    ans
}
