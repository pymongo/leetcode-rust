/// https://leetcode.com/problems/edit-distance
#[cfg(feature = "rustc_private")]
fn edit_distance_using_rustc_span(a: String, b: String) -> usize {
    rustc_span::lev_distance::lev_distance(&a, &b)
}

/** https://leetcode.com/problems/edit-distance
- rustc_span::lev_distance::lev_distance used in rustc
- strsim::levenshtein used in rustup/darling

edit_distance 的同义词: Levenshtein distance, Damerau-Levenshtein distance

## wikipedia
- [Edit distance](https://en.wikipedia.org/wiki/Edit_distance)
- [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
- [Damerau-Levenshtein distance](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)

---

## rustc 的实现 和 strsim库的实现
首先 rustc 为了自举也为了代码质量所以编译时尽量不要依赖第三方库，目前来看也就依赖了 libc, cc 这种 C 语言相关的库
所以 rustc 需要自己实现一个高性能的 edit_distance 算法
strsim 作为库，为了通用性把 edit_distance 的入参做成泛型

rustc 的动态规划实现的空间复杂度是 O(n)，而 strsim 的动态规划实现的空间复杂度是 O(n^2)
在 leetcode 上的运行 strsim 的实现耗时 4ms，而 rustc 的实现耗时 0ms

---

## tweet: 我对 rustc 和 strsim 关于 edit_distance 的评论
https://twitter.com/ospopen/status/1380091878440271872

```text
这个算法用于typo检查和拼写错误时的候选词的建议

rust源码动态规划空间复杂度是O(n)优于strsim的O(n^2)
所以leetcode上strsim跑这题耗时4ms会慢一些

## Trie 前缀树
虽然 Trie 跟 dp 没关系，但是 Trie 跟 edit_distance 一起用能实现高性能的拼写错误检查
而且我还想实现一个基于Trie数据结构的edit_distance算法
所以我把 Trie数据结构 和 edit_distance算法放到一起
```
*/
fn lev_distance(a: &str, b: &str) -> usize {
    // cases which don't require further computation
    if a.is_empty() {
        return b.chars().count();
    } else if b.is_empty() {
        return a.chars().count();
    }

    let mut dcol: Vec<_> = (0..=b.len()).collect();
    let mut t_last = 0;

    for (i, sc) in a.chars().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, tc) in b.chars().enumerate() {
            let next = dcol[j + 1];
            if sc == tc {
                dcol[j + 1] = current;
            } else {
                dcol[j + 1] = std::cmp::min(current, next);
                dcol[j + 1] = std::cmp::min(dcol[j + 1], dcol[j]) + 1;
            }
            current = next;
            t_last = j;
        }
    }
    dcol[t_last + 1]
}

/// edit from rustc_ast::lev_distance
fn lev_distance_optimize(word1: String, word2: String) -> i32 {
    let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
    let (word1_len, word2_len) = (word1.len(), word2.len());

    // word1_len or word2_len == 0, warning: usize mul usize overflow
    if word1_len * word2_len == 0 {
        return (word1_len + word2_len) as i32;
    }
    let mut dcol: Vec<_> = (0..=word2_len).collect();
    let mut t_last = 0;

    for (i, sc) in word1.into_iter().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, &tc) in word2.iter().enumerate() {
            let next = dcol[j + 1];
            if sc == tc {
                dcol[j + 1] = current;
            } else {
                dcol[j + 1] = current.min(next);
                dcol[j + 1] = dcol[j + 1].min(dcol[j]) + 1;
            }
            current = next;
            t_last = j;
        }
    }
    dcol[t_last + 1] as i32
}

struct EditDistanceRecursive {
    word1: Vec<u8>,
    word2: Vec<u8>,
}

impl EditDistanceRecursive {
    fn solution_entrance(word1: String, word2: String) -> i32 {
        let helper = Self {
            word1: word1.into_bytes(),
            word2: word2.into_bytes(),
        };
        helper.edit_distance(helper.word1.len(), helper.word2.len()) as i32
    }

    /**
    ```python
    def best_solution_tail_recursive(word1: str, word2: str) -> int:
        # 如果cache用的不好，还不如禁用LRU cache，因为这算尾递归不需要memo让递归间共享变量
        @lru_cache(None)
        def dp(n, m):
            if n == 0:
                return m
            if m == 0:
                return n
            if word1[n - 1] == word2[m - 1]:
                return dp(n - 1, m - 1)
            # 为何看上去没有memo也能这么快？同理用栈模拟尾递归也是最快的解法
            return min(
                dp(n, m - 1),  # insert
                dp(n - 1, m),  # remove
                dp(n - 1, m - 1)  # replace
            ) + 1
        return dp(len(word1), len(word2))
    ```
    思路，两个指针分别从后往前扫两个字符串
    */
    fn edit_distance(&self, index1: usize, index2: usize) -> usize {
        // 如果从后往前扫其中一个指针遍历完了，则剩余的编辑距离就是另一个指针未遍历的剩余长度
        if index1 == 0 {
            return index2;
        }
        if index2 == 0 {
            return index1;
        }
        // NOTE 注意为了避免0_usize-1越界，所以index1会从 word_len1..1
        // 所以当前遍历的下标应该分别是 index1-1 和 index2-1
        if self.word1[index1 - 1] == self.word2[index2 - 1] {
            return self.edit_distance(index1 - 1, index2 - 1);
        }
        self.edit_distance(index1, index2 - 1) // insert: word2[..=index2-2] push word1[index1-1] to make word1[..=index1-1] same as word2[..=index2-1]
            .min(self.edit_distance(index1 - 1, index2)) // delete: word2[..=index2-2] delete last char
            .min(self.edit_distance(index1 - 1, index2 - 1)) // replace
            + 1
    }
}

/// 从字符串word1修改成word2至少需要多少次操作(replace/insert/delete)
/// follow_up: 实现一个语料库是trie而不是Vec<String>的edit_distance算法
#[allow(clippy::needless_range_loop)]
fn edit_distance_dp(word1: String, word2: String) -> i32 {
    let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
    let (word1_len, word2_len) = (word1.len(), word2.len());
    // # dp[i][j]表示word1[..i]至少需要多少次操作(replace/insert/delete)替换成B[..j]
    // 很容易想到的其中一种状态转移的情况: 如果word1[i]==word2[j]，那么dp[i][j]==dp[i-1][j-1]
    let mut dp = vec![vec![0; word2_len + 1]; word1_len + 1];
    for i in 0..=word1_len {
        // 需要i次删除操作才能让word1[..i]修改成空的字符串word2[..0]
        dp[i][0] = i;
    }
    for j in 0..=word2_len {
        // 需要j次插入操作才能让空字符串word1[..0]修改成word2[..j]
        dp[0][j] = j;
    }
    for i in 1..=word1_len {
        for j in 1..=word2_len {
            if word1[i - 1] == word2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                // dp[i-1][j-1] + 1: word1[i-1]和word2[i-2]不同，所以替换次数+1,
                //                   如果dp的决策层选择replace操作，dp[i][j]总共操作数等于dp[i-1][j-1]+1
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]) + 1;
            }
        }
    }
    dp[word1_len][word2_len] as i32
}

#[test]
fn test_edit_distance() {
    const TEST_CASES: [(&str, &str, i32); 2] = [("horse", "ros", 3), ("intention", "execution", 5)];
    for &(word1, word2, edit_distance) in &TEST_CASES {
        assert_eq!(
            lev_distance_optimize(word1.to_string(), word2.to_string()),
            edit_distance
        );
        assert_eq!(
            EditDistanceRecursive::solution_entrance(word1.to_string(), word2.to_string()),
            edit_distance
        );
        assert_eq!(
            edit_distance_dp(word1.to_string(), word2.to_string()),
            edit_distance
        );
    }
}
