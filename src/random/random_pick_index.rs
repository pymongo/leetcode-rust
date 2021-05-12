//! https://leetcode.com/problems/random-pick-index/
//! 相似题目: random_pick_index_with_weight, random_pick_index_with_blacklist, linked_list_random_node
//! 如果nums中存在多个target，则等概率地随机返回一个满足nums[i]=target的下标i
//! 题目给出的输入数据像是online_data/stream_data，数据流很大
//! 关键是`new`的调用次数会比`pick`多，所以应当均摊时间复杂度，优化new而不优化pick
//! 所以将输入的nums做成一个counter对于这题的测试用例还说是最慢的解法

#[allow(non_camel_case_types)]
type time_t = isize;

struct RandomPickIndex {
    nums: Vec<i32>,
}

impl RandomPickIndex {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn pick(&mut self, target: i32) -> i32 {
        extern "C" {
            fn rand() -> i32;
        }
        let mut count = 0_i32;
        let mut ret = 0_usize;
        for (i, num) in self.nums.iter().enumerate() {
            if target.ne(num) {
                continue;
            }
            count += 1;
            // 蓄水池抽样: 以1/n的概率更新return_value(留下当前的数据) 或 (n-1)/n不更新return_value(继续用之前的ret的值)
            // n为数据流(online_data,stream_data)中过去数据里值等于target的个数(也就是count变量)
            if unsafe { rand() } % count == 0 {
                ret = i;
            }
        }
        ret as i32
    }
}

struct RandomPickIndexCounterSolution {
    nums_index: std::collections::HashMap<i32, Vec<i32>>,
}

impl RandomPickIndexCounterSolution {
    fn new(nums: Vec<i32>) -> Self {
        let mut nums_index = std::collections::HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            nums_index
                .entry(num)
                .or_insert_with(Vec::new)
                .push(i as i32);
        }
        // leetcode对随机数的检查不严格，即便不用当前时间戳做随机数种子(srand,time)去提高随机性，依然能AC
        Self { nums_index }
    }

    /// 如果nums中存在多个target，则等概率地随机返回一个满足nums[i]=target的下标i
    fn pick(&mut self, target: i32) -> i32 {
        extern "C" {
            fn rand() -> i32;
        }
        let candidates = self.nums_index.get(&target).unwrap();
        let random_number = unsafe { rand() };
        candidates[random_number as usize % (candidates.len() + 1)]
    }
}
