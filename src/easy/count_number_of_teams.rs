struct Solution;

impl Solution {
    #[allow(clippy::comparison_chain)]
    pub fn num_teams(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for mid in 1..n - 1 {
            let mid_num = nums[mid];
            let (mut left_smaller, mut left_bigger) = (0, 0);
            for i in 0..mid {
                if nums[i] < mid_num {
                    left_smaller += 1;
                } else if nums[i] > mid_num {
                    left_bigger += 1;
                }
            }
            let (mut right_smaller, mut right_bigger) = (0, 0);
            for i in mid + 1..n {
                if nums[i] < mid_num {
                    right_smaller += 1;
                } else if nums[i] > mid_num {
                    right_bigger += 1;
                }
            }
            res += left_smaller * right_bigger + left_bigger * right_smaller;
        }
        res
        /*
        let n = a.len();
        let mut res = 0;
        for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    if (a[i] < a[j] && a[j] < a[k]) || (a[i] > a[j] && a[j] > a[k]) {
                        res += 1;
                    }
                }
            }
        }
        res
        */
    }
}
