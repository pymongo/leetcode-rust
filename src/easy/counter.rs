//! 所有用到了类似Python的collections.Counter的题型

/** https://leetcode.com/problems/number-of-good-pairs/
count if nums[i] == nums[j] and i < j
思路: collection.Counter统计输入数组每个数字的出现次数，例如3出现了3次，那么就有math.comb(3,2)=3*2=6对满足i<j且nums[i]==nums[j]

除了Counter，还有已知输入全小写字母字符串需要一些记忆操作例如最长无重复子串，也建议用固定长数组，下标寻址比HashMap快
```python
return sum(map(lambda v: math.comb(v, 2), collections.Counter(nums).values()))
TODO 【待定】写个bench对比两种数据类型的counter
TODO 这题各种u8溢出，适合用来讲课
```
*/
fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    // TODO 还可以继续优化，既然nums中每个数的大小范围在0..=100，那么用counter用固定101长度的数组性能更好
    let mut counter = std::collections::HashMap::with_capacity(nums.len());
    for num in nums.into_iter() {
        *counter.entry(num).or_insert(0u8) += 1;
    }
    counter
        .into_iter()
        .map(|(_k, v)| (v as i32 - 1) * v as i32 / 2)
        .sum::<i32>()
}

#[cfg(FALSE)]
// https://leetcode.com/problems/first-unique-character-in-a-string/
fn first_unique_char(s: String) -> i32 {
    // let s = s.into_bytes();
    // let mut counter = std::collections::HashMap::with_capacity(s.len());
    // for byte in s.iter() {
    //     *counter.entry(*byte).or_insert(0u8)
    // }
    unimplemented!()
}

fn fn_1(nums: Vec<i32>) -> i32 {
    let mut counter = std::collections::HashMap::new();
    for num in nums.into_iter() {
        *counter.entry(num).or_insert(0) += 1;
    }
    let a = Some(1);
    counter.into_iter().map(|(_k, v)| (v - 1) * v / 2).sum()
}
