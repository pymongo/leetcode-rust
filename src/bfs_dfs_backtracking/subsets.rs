/** https://leetcode.com/problems/subsets
# subsets-输入数组无重复项(简单版)

## subsets DFS搜索树
注: 尖括号表示剪枝(不要的重复项)
```text
                [ ]
    [1]         [2]         [3]
[1,2] [1,3] <2,1> [2,3] <3,1> <3,2>
[1,2,3]
```
## 减枝去重
TODO

## 如何缩小问题的规模？数学公式?
将返回值的二维数组按长度分组，不难发现以下规律
C(n,0): [ ]
C(n,1): [1] [2] [3]
C(n,2): [1,2] [1,3] [2,3]
C(n,3): [1,2,3]
所以可以将问题简化为编写一个 穷举从长度为n的数组中取k个的函数的不同组合
*/
fn subsets_dfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
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

## subsets BFS搜索树
```text
[]
[] [1]
[] [1] [2] [1,2]
[] [1] [2] [1,2] [3] [1,3] [2,3] [1,2,3]
```

数据结构上用不断遍历上一次队列长度进行出队处理后再入队，或者用sentinel_node的队列也可以
但是在内存利用率上远不如两个新旧数组间互相迭代(例如二叉树的层级遍历)

根据每个num的选或不选组成二叉树
*/
fn subsets_bfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut last: Vec<Vec<i32>> = Vec::with_capacity(2_usize.pow(nums.len() as u32));
    last.push(Vec::with_capacity(0));
    for num in nums {
        let mut curr = last.clone();
        for each_curr in curr.iter_mut() {
            each_curr.push(num);
        }
        last.append(&mut curr);
    }
    last
}

#[test]
fn test_subsets() {
    let test_cases = vec![(
        vec![1, 2, 3],
        vec_vec![[], [1], [2], [3], [1, 2], [1, 3], [2, 3], [1, 2, 3]],
    )];
    for (input, output) in test_cases {
        assert_eq!(subsets_dfs(input.clone()), output);
        let mut bfs_output = subsets_bfs(input);
        bfs_output.sort_by_cached_key(|arr| arr.len());
        assert_eq!(bfs_output, output);
    }
}
