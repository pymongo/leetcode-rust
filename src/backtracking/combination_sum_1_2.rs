struct Solution;

impl Solution {
    fn combination_sum2(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // .sort()调用归并排序，需要额外空间，建议使用sort_unstable()速度更快而且无需额外空间()
        nums.sort_unstable();
        let n = nums.len();
        let mut cur = Vec::with_capacity(n);
        let mut res = Vec::new();
        Self::helper(0, target, &mut cur, &mut res, &nums, n);
        res
    }

    fn helper(
        start: usize,
        target: i32,
        cur: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        nums: &Vec<i32>,
        n: usize,
    ) {
        if target == 0 {
            res.push(cur.clone());
        }
        for i in start..n {
            // 剪枝: 由于数组是有序的，如果当前的值比target大，则往后的值也是比target大
            if nums[i] > target {
                return;
            }
            // 剪枝: 答案集去重，避免重复解
            if i > start && nums[i] == nums[i-1] {
                continue;
            }
            cur.push(nums[i]);
            // combination_sum_1: 允许nums中元素重复使用，所以start_index会是i
            // Self::helper(i, target-nums[i], cur, res, &nums, n);
            // combination_sum_2: 不允许nums中元素重复使用，所以start_index会是i+1
            Self::helper(i+1, target-nums[i], cur, res, &nums, n);
            cur.pop();
        }
    }
}

#[test]
fn test() {
    let res = Solution::combination_sum2(vec![2,5,2,1,2], 5);
    for row in res {
        println!("{:?}", row);
    }
}