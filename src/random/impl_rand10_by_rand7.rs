/// gen range 1..=7
fn rand7() -> i32 {
    crate::code_snippets::random_i32::rand_range(1, 7)
}

/** https://leetcode.com/problems/implement-rand10-using-rand7/
## Rejection Sampling
可以理解成将 2位7进制数 转为 1位10进制
因为至少要两位7进制数才能表达一位10进制
b-1\a 1 2 3 4 5 6 7
0     1 2 3 4 5 6 7
1     8 9 101 2 3 4
2     5 6 7 8 9 101
3     2 3 4 5 6 7 8
4     9 101 2 3 4 5
5     6 7 8 9 10* *
6     * * * * * * *

*表示拒绝掉的抽样，有9/49的概率被拒绝(41..=49)，所以叫拒绝抽样
所以两次rand7()得到的2位7进制数能模拟1..=40的十进制范围，
*/
fn rand10() -> i32 {
    loop {
        let (a, b) = (rand7(), rand7());
        // b是十位，(b-1)是为了偏移一下，从[1,7]偏移到[0,6]，因为想10位的系数从0开始算
        let decimal = a + (b - 1) * 7;
        if decimal <= 40 {
            break decimal % 10 + 1;
        }
    }
}
