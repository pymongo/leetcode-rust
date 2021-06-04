/// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
/// nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums
/// 所谓In-Place的解法过于投机取巧，难以理解，不想照抄官方解答
fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut counter = vec![false; n + 1];
    for num in nums {
        counter[num as usize] = true;
    }
    let mut ret = vec![];
    for (num, is_appeared) in counter.into_iter().skip(1).enumerate() {
        if !is_appeared {
            ret.push(num as i32 + 1);
        }
    }
    ret
}
