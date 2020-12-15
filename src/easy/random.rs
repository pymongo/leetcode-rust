

/// 考虑到测试用例中pick的调用次数较少，new的调用较多，题目也提示尽量别用额外存储空间(例如HashMap counter)，所以优化new()，不优化pick，均摊时间复杂度
struct RandomPickIndexSolution2 {
    nums: Vec<i32>,
}

impl RandomPickIndexSolution2 {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    /// 如果nums中存在多个target，则等概率地随机返回一个满足nums[i]=target的下标i
    fn pick(&mut self, target: i32) -> i32 {
        const RAND_MAX: i32 = std::i32::MAX;

        let mut count = 0i32;
        let mut ret = 0usize;
        for (i, num) in self.nums.iter().enumerate() {
            if target.ne(num) {
                continue;
            }
            if unsafe {rand()} < RAND_MAX / count {
                ret = i;
            }
            count += 1;
        }
        ret as i32
    }
}

mod temp {
    extern "C" {
        fn rand() -> i32;
    }
}
