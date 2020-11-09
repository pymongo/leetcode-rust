struct Solution;

impl Solution {
    fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut res = a.into_iter().map(|x| x * x).collect::<Vec<i32>>();
        res.sort_unstable();
        res
    }

    fn two_pointers_sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut res = vec![0i32; n];
        let (mut l, mut r, mut largest_idx) = (0, n - 1, n - 1);
        loop {
            let l_square = (a[l].pow(2)) as i32;
            let r_square = (a[r].pow(2)) as i32;
            if l_square > r_square {
                res[largest_idx] = l_square;
                // skip index out of range checking
                l += 1;
            } else {
                res[largest_idx] = r_square;
                if r <= l {
                    break;
                }
                r -= 1;
            }
            largest_idx -= 1;
        }
        res
    }
}

#[test]
fn test() {
    const TEST_CASES: [&[i32]; 2] = [&[-4, -1, 0, 3, 10], &[-7, -3, 2, 3, 11]];

    for &nums in TEST_CASES.iter() {
        assert!(Solution::sorted_squares(nums.to_vec()).is_sorted());
        assert!(Solution::two_pointers_sorted_squares(nums.to_vec()).is_sorted());
    }
}
