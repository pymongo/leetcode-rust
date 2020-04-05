/* Problem 0001: Two Sum
Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
*/
#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut result: Vec<i32> = Vec::new();
  let mut map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
  for (index, num) in nums.iter().enumerate() {
    if map.contains_key(num) {
      result.push(*map.get(num).unwrap() as i32);
      result.push(index as i32);
    } else {
      map.insert(target - *num, index);
    }
  }
  // Review: 别人的解答都是在代码中间return就行了，好像确实是只需要返回第一个结果
  return result;
}

#[allow(dead_code)]
fn two_sum_fastest_btreemap(nums: Vec<i32>, target: i32) -> Vec<i32> {
  /*
  阅读第一名的代码的收获：
  1. 在函数中间return，题目只要求返回一对索引，不需要遍历完整个数组(浪费时间)
  2. BTreeMap在.get()方法的性能上明显强于HashMap
     理论上红黑树的时间复杂度为O(logN)，散列的时间复杂度为O(1)
     在JDK1.8中，HashMap的长度大于8时才会转为红黑树进行存储
     在Java中TreeMap是自动排序的，因此插入/删除操作会牺牲性能
  */
  let mut map: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
  for (index, num) in nums.iter().enumerate() {
    if map.contains_key(&num) {
      return vec![*map.get(num).unwrap() as i32, index as i32];
    }
    map.insert(target - *num, index);
  }
  vec![-1, -1]
}

#[allow(dead_code)]
pub fn run() {
  // 我曾在这个测试用例上挂了：[-3, 4, 3, 90] - 0
  // [OK]: let result = two_sum(vec![2, 7, 11, 15], 9);
  let result = two_sum(vec![-3, 4, 3, 90], 0);
  for each in &result {
    println!("{}", each);
  }
}

