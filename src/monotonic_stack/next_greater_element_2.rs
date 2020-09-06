struct Solution;

// https://mp.weixin.qq.com/s?__biz=MzAxODQxMDM0Mw==&mid=2247484525&idx=1&sn=3d2e63694607fec72455a52d9b15d4e5&chksm=9bd7fa65aca073734df90b45054448e09c14e6e35ad7b778bff62f9bd6c2b4f6e1ca7bc4f844&scene=21#wechat_redirect
impl Solution {
    // 由于入参是一个循环数组，按照旋转排序数组[rotate string一题查找某个字符串能否通过s循环移位后得到]的经验
    // 是原数组翻倍，也就是后面再接上一个原数组
    fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return Vec::with_capacity(0);
        }
        let mut res = vec![-1; n];
        let mut stack: Vec<i32> = Vec::with_capacity(n);
        for i in (0..(2 * n - 1)).rev() {
            while !stack.is_empty() && stack.last().unwrap().le(&nums[i % n]) {
                stack.pop();
            }
            if let Some(peek) = stack.last() {
                res[i % n] = *peek;
            }
            stack.push(nums[i % n]);
        }
        res
    }
}

#[cfg(test)]
const TEST_CASES: [(&[i32], &[i32]); 1] = [(&[1, 2, 1], &[2, -1, 2])];

#[test]
fn test() {
    for &(nums, output) in TEST_CASES.iter() {
        assert_eq!(
            Solution::next_greater_elements(nums.to_vec()),
            output.to_vec()
        );
    }
}
