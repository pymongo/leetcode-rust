/*
《数字信号处理》—— 镜像反射法生成格雷码:
0 1 2
0 0 00
  1 01
    11
    10

先将上次迭代的结果「上下镜像复制」一份，原顺序部分在左侧加上0，镜像对称部分在左侧加上1
这样能保证格雷码的要求: 从上到下仅有1bit的变化
如果懂了这些背景知识，那么这题就是easy难度了，首先左侧补0的那一半可以不做(二进制左侧加0等于不变)，因为原数组各项+0后还是原数组
*/
struct Solution;

impl Solution {
    fn gray_code(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(2usize.pow(n as u32));
        res.push(0);
        // head表示镜像反射法生成格雷码中左侧补1需要加上的数，刚好等于镜像后的下半部分(需要左边加一个1)的个数
        let mut head: i32 = 1;
        for _ in 0..n {
            // 镜像对称部分在左侧加上1
            for i in (0..head as usize).rev() {
                res.push(head + res[i]);
            }
            head <<= 1;
        }
        res
        /* 位运算的解法?
        let mut res=vec![];
        for i in 0..1<<n{
            res.push(i^i>>1);
        }
        res
        */
    }
}

#[test]
fn test() {
    assert_eq!(Solution::gray_code(2), vec![0b00, 0b01, 0b11, 0b10]);
}
