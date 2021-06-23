/** https://leetcode.com/problems/subsets
# subsets-i: 输入数组无重复项(简单版)
Input:  vec![1, 2, 3]
Output: vec_vec![[], [1], [2], [3], [1, 2], [1, 3], [2, 3], [1, 2, 3]]

## 三种解决思路
1. 组合数: 将问题的规模缩小为从C(n,0)+C(n,1)，也就是从3个选0个,1个,2个,3个集合的和
2. BFS迭代: curr=curr.append(curr.each+nums[i])
3. DFS决策: 遍历1,2,3时每个都有选或不选的决策

## C(n,k)解法: 如何缩小问题的规模？数学公式?
将返回值的二维数组按长度分组，不难发现以下规律
C(n,0): [ ]
C(n,1): [1] [2] [3]
C(n,2): [1,2] [1,3] [2,3]
C(n,3): [1,2,3]
所以可以将问题简化为编写一个 穷举从长度为n的数组中取k个的函数的不同组合

## subsets可能的搜索树?
注: 尖括号表示剪枝(不要的重复项)
```text
                [ ]
    [1]         [2]         [3]
[1,2] [1,3] <2,1> [2,3] <3,1> <3,2>
```
*/
fn subsets_combine(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    for k in 0..=nums.len() {
        combine_n_k(&nums, k, &mut ret);
    }
    ret
}

/// https://leetcode.com/problems/combinations/
/// itertools.combinations(nums, k)
/// 非递归的combine算法用到了二进制的字典序，过于难
fn combine_n_k(nums: &[i32], k: usize, ret: &mut Vec<Vec<i32>>) {
    /// i: unused/un_choose num index in nums
    fn helper(
        unused_start: usize,
        cur: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
        nums: &[i32],
        k: usize,
    ) {
        if cur.len() == k {
            ret.push(cur.clone());
            return;
        }
        for unused_idx in unused_start..nums.len() {
            cur.push(nums[unused_idx]);
            helper(unused_idx + 1, cur, ret, nums, k);
            cur.pop().unwrap();
        }
    }
    let mut cur = Vec::with_capacity(k);
    helper(0, &mut cur, ret, nums, k);
}

/// https://leetcode.com/problems/combinations/
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let nums: Vec<i32> = (1..).take(n as usize).collect();
    combine_n_k(&nums, k as usize, &mut ret);
    ret
}

/**
```python
output = [[]]
for num in nums:
    output += [curr + [num] for curr in output]
return output
```

## ⭐subsets BFS搜索树
```text
[]
[] [1]
[] [1] [2] [1,2]
[] [1] [2] [1,2] [3] [1,3] [2,3] [1,2,3]
```

check DFS search tree on problem sum-of-all-subset-xor-totals

数据结构上用不断遍历上一次队列长度进行出队处理后再入队，或者用sentinel_node的队列也可以
但是在内存利用率上远不如两个新旧数组间互相迭代(例如二叉树的层级遍历)

根据每个num的选或不选组成二叉树
*/
fn subsets_bfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut last: Vec<Vec<i32>> = Vec::with_capacity(2_usize.pow(nums.len() as u32));
    last.push(Vec::with_capacity(0));
    for num in nums {
        let mut curr = last.clone();
        for each_curr in &mut curr {
            each_curr.push(num);
        }
        last.append(&mut curr);
    }
    last.sort_by_cached_key(Vec::len);
    last
}

struct SubsetsDfs {
    nums: Vec<i32>,
    len: usize,
    cur: Vec<i32>,
    ret: Vec<Vec<i32>>,
}

/**
## ⭐subsets DFS搜索树
left_child : not select
right_child:     select
```text
        []
1:   []     [1]
2: [] [2] [1] [2]
```
*/
impl SubsetsDfs {
    fn dfs(&mut self, index: usize) {
        if index == self.len {
            self.ret.push(self.cur.clone());
            return;
        }

        // select current num
        self.cur.push(self.nums[index]);
        self.dfs(index + 1);
        self.cur.pop();

        // doesn't select current num
        self.dfs(index + 1);
    }
}

fn subsets_dfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut helper = SubsetsDfs {
        nums,
        len,
        cur: Vec::new(),
        ret: Vec::new(),
    };
    helper.dfs(0);
    helper.ret.sort_by_cached_key(Vec::len);
    helper.ret
}

#[test]
fn test_subsets() {
    let test_cases = vec![(
        vec![1, 2, 3],
        vec_vec![[], [1], [2], [3], [1, 2], [1, 3], [2, 3], [1, 2, 3]],
    )];
    for (input, output) in test_cases {
        assert_eq!(subsets_combine(input.clone()), output);
        assert_eq!(subsets_bfs(input.clone()), output);
        assert_eq!(subsets_dfs(input), output);
    }
}

/// https://leetcode.com/problems/sum-of-all-subset-xor-totals/
fn subsets_xor_sum(nums: Vec<i32>) -> i32 {
    let mut last_subsets = vec![vec![]];
    for num in nums {
        let mut curr_subsets = last_subsets.clone();
        for each_subset in &mut curr_subsets {
            each_subset.push(num);
        }
        last_subsets.append(&mut curr_subsets);
    }

    last_subsets
        .into_iter()
        .map(|subset| subset.into_iter().fold(0, |a, b| a ^ b))
        .sum()
}

/**
## DFS搜索树
left_child : not select current num
right_child:     select current num
```text
        []
1:   []    [1]
```
*/
fn subsets_xor_sum_dfs(nums: Vec<i32>) -> i32 {
    struct Dfs {
        nums: Vec<i32>,
        len: usize,
        total_xor_sum: i32,
    }
    impl Dfs {
        fn dfs(&mut self, index: usize, curr_xor_sum: i32) {
            if index == self.len {
                self.total_xor_sum += curr_xor_sum;
                return;
            }
            // select current num
            self.dfs(index + 1, curr_xor_sum ^ self.nums[index]);
            // doesn't select current num
            self.dfs(index + 1, curr_xor_sum);
        }
    }
    let len = nums.len();
    let mut helper = Dfs {
        nums,
        len,
        total_xor_sum: 0,
    };
    helper.dfs(0, 0);
    helper.total_xor_sum
}

#[test]
fn test_subsets_xor_sum() {
    const TEST_CASES: [(&[i32], i32); 1] = [(&[5, 1, 6], 28)];
    for (nums, output) in TEST_CASES {
        assert_eq!(subsets_xor_sum(nums.to_owned()), output);
        assert_eq!(subsets_xor_sum_dfs(nums.to_owned()), output);
    }
}
