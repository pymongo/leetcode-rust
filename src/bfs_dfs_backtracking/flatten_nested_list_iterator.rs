//! https://leetcode.com/problems/flatten-nested-list-iterator/
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<Self>),
}

/**
借助队列或栈模拟递归的解法
```python
def flatten(lists):
    ret = []
    q = collections.deque(lists)
    while q:
        item = q.popleft()
        if isinstance(item, list):
            # 保证重新扔回队列头部时是按照数组原有顺序
            for each in reversed(item):
                q.appendleft(each)
            continue
        ret.append(item)
    return ret
```
*/
fn flatten_dfs(list: Vec<NestedInteger>) -> Vec<i32> {
    list.into_iter()
        .flat_map(|item| match item {
            NestedInteger::Int(int) => vec![int],
            NestedInteger::List(list) => flatten_dfs(list),
        })
        .collect()
}

struct NestedIterator {
    cursor: usize,
    len: usize,
    nums: Vec<i32>,
}

impl NestedIterator {
    fn new(list: Vec<NestedInteger>) -> Self {
        // use flattern to merge multi arrays?
        let nums = flatten_dfs(list);
        Self {
            cursor: 0,
            len: nums.len(),
            nums,
        }
    }

    fn next(&mut self) -> i32 {
        let ret = self.nums[self.cursor];
        self.cursor += 1;
        ret
    }

    fn has_next(&mut self) -> bool {
        self.cursor < self.len
    }
}
