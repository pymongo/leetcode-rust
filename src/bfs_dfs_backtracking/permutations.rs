/// https://leetcode.com/problems/permutations/
/// https://leetcode.com/problems/permutations-ii/
struct PermutationsWithDedup<T> {
    cur: Vec<T>,
    ret: Vec<Vec<T>>,
    nums: Vec<T>,
    used: Vec<bool>,
    len: usize,
}

/// O(n! * n)
impl<T: Copy + Eq> PermutationsWithDedup<T> {
    fn dfs(&mut self) {
        if self.cur.len() == self.len {
            // O(n)
            self.ret.push(self.cur.clone());
            return;
        }

        for i in 0..self.len {
            if self.used[i] {
                continue;
            }

            /* 「剪枝去重」
            used[i-1]=false表示backtraing的过程中 nums[i-1] 已经被遍历过了
            例如: [a1, a2, b]
            搜索a1时一定考虑过 used[a1=true]+used[a2=false],的情况
            a2的搜索树遇到used[a2=false]时可以剪枝，因为跟搜索a1时重复了
            */
            if i > 0 && self.nums[i - 1] == self.nums[i] && !self.used[i - 1] {
                continue;
            }

            self.used[i] = true;
            self.cur.push(self.nums[i]);
            self.dfs();
            self.used[i] = false;
            self.cur.pop().unwrap();
        }
    }
}

fn permutations_ii(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let len = nums.len();
    let mut helper = PermutationsWithDedup {
        cur: Vec::new(),
        ret: Vec::new(),
        nums,
        used: vec![false; len],
        len,
    };
    helper.dfs();
    helper.ret
}

#[test]
fn test_permutations_ii() {
    let test_cases = vec![
        (vec![1, 1, 2], vec_vec![[1, 1, 2], [1, 2, 1], [2, 1, 1]]),
        (
            vec![1, 2, 3],
            vec_vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ],
        ),
    ];
    for (input, output) in test_cases {
        assert_eq!(permutations_ii(input), output);
    }
}
