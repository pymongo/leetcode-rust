//! https://leetcode.com/problems/two-sum/
//! 本题主要是掌握使用(B)Tree Map的数据结构达到O(n)的时间复杂度
//! bitwise_补码的解法收录在java_leetcode中
//! 不记录暴力遍历求解的方法

/*
阅读第一名的代码的收获：
1. 在函数中间return，题目只要求返回一对索引，不需要遍历完整个数组(浪费时间)
2. BTreeMap在.get()方法的性能上明显强于HashMap
   理论上红黑树的时间复杂度为O(logN)，散列的时间复杂度为O(1)
   在JDK1.8中，HashMap的长度大于8时才会转为红黑树进行存储
   在Java中TreeMap是自动排序的，因此插入/删除操作会牺牲性能
3. 函数的最后，如果测试用例没有匹配项的话，可以写unreachable!()或返回vec![]
*/

// benches/two_sum_hashmap_vs_treemap记录了hashmap和treemap的性能比较
pub fn two_sum_btreemap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_tracker: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
    for i in 0..nums.len() {
        if sum_tracker.contains_key(&nums[i]) {
            return vec![
                *sum_tracker.get(&nums[i]).unwrap() as i32,
                i as i32
            ];
        }
        sum_tracker.insert(target - nums[i], i);
    }
    vec![]
}

#[test]
fn test_two_sum_btreemap() {
    assert_eq!(two_sum_btreemap(vec![-3, 4, 3, 90], 0), vec![0, 2]);
}

pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_tracker: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for i in 0..nums.len() {
        if sum_tracker.contains_key(&nums[i]) {
            return vec![
                *sum_tracker.get(&nums[i]).unwrap() as i32,
                i as i32
            ];
        }
        sum_tracker.insert(target - nums[i], i);
    }
    vec![]
}

#[test]
fn test_two_sum_hashmap() {
    assert_eq!(two_sum_btreemap(vec![-3, 4, 3, 90], 0), vec![0, 2]);
}
