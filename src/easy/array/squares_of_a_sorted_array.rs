//! https://leetcode.com/problems/squares-of-a-sorted-array/

fn sorted_squares_brute_force(a: Vec<i32>) -> Vec<i32> {
    let mut res = a.into_iter().map(|x| x * x).collect::<Vec<i32>>();
    res.sort_unstable();
    res
}

/// a is sorted, left-> ... <-right, a[left] or a[right] must has one is biggest
fn sorted_squares_two_pointers_solution(a: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut ret = vec![0_i32; n];
    let (mut l, mut r, mut largest_idx) = (0, n - 1, n - 1);
    loop {
        let l_square = a[l] * a[l];
        let r_square = a[r] * a[r];
        if l_square > r_square {
            ret[largest_idx] = l_square;
            // skip index out of range checking
            l += 1;
        } else {
            ret[largest_idx] = r_square;
            if r <= l {
                break;
            }
            r -= 1;
        }
        largest_idx -= 1;
    }
    ret
}

#[test]
fn test() {
    const TEST_CASES: [&[i32]; 2] = [&[-4, -1, 0, 3, 10], &[-7, -3, 2, 3, 11]];

    for nums in TEST_CASES {
        assert!(sorted_squares_brute_force(nums.to_vec()).is_sorted());
        assert!(sorted_squares_two_pointers_solution(nums.to_vec()).is_sorted());
    }
}
