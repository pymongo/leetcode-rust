#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<Self>),
}

struct NestedIterator {
    cursor: usize,
    len: usize,
    nums: Vec<i32>,
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
        .map(|item| match item {
            NestedInteger::Int(int) => vec![int],
            NestedInteger::List(list) => flatten_dfs(list),
        })
        .flatten()
        .collect()
}

#[test]
fn feature() {
    let a = vec![Some(1i32), None, Some(2)];
    let mut b = a.into_iter().flatten();
    dbg!(b.next().unwrap());
    dbg!(b.next().unwrap());
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
