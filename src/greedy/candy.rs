/// https://leetcode.com/problems/candy/
fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    // 首先每个孩子至少要给1个糖果
    let mut nums = vec![1; n];
    // 先看每个孩子的左边情况，如果当前孩子比左边的成绩更好，则理应比左边的多一个糖果
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            nums[i] = nums[i - 1] + 1;
        }
    }
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            nums[i] = nums[i].max(nums[i + 1] + 1);
        }
    }
    nums.into_iter().sum()
}
