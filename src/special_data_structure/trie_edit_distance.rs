/*! https://leetcode.com/problems/implement-trie-prefix-tree/
Trie的同义词: 前缀树、字典树
前缀树每个节点只有一个字符，root相当于一个dummyHead不存储字符
每个节点除了要存储字符，还要存储从根到当前节点是否构成一个单词
TODO 这题没要求实现「删除操作」，所以可以不写析构函数，较简单

## 前缀树的优点
插入和查找的时间复杂度都是O(L), L为单词长度

## 前缀树的应用
1. 搜索框自动补全或提供下拉选项的提示，例如谷歌搜索rust，下面会有很多rust开头的候选词的提示
2. 9宫格/9键手机输入时，联想候选词
3. Typo/拼写检查
4. Boggle单词游戏, 给你一个乱序的字母矩阵，从矩阵中任意一点往上下左右四个方向去搜索，能找到几个单词
5. IP 路由 (最长前缀匹配)
*/
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            is_word: false,
        }
    }
}

/// https://leetcode.com/problems/implement-trie-prefix-tree/
struct MyTrie {
    root: TrieNode,
}

impl MyTrie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let word = word.into_bytes();
        let mut curr = &mut self.root;
        for char_ in word {
            let index = (char_ - b'a') as usize;
            // like HashMap entry().or_insert()
            match curr.children[index] {
                Some(ref mut next) => {
                    curr = next;
                }
                None => {
                    // 用类似 HashMap or_insert() Entry 模式可能可以避免 unwrap
                    curr.children[index] = Some(Box::new(TrieNode::new()));
                    curr = curr.children[index].as_mut().unwrap();
                }
            }
        }
        // 单词结尾标志，避免插入apple时会把app也认为是一个单词
        curr.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let word = word.into_bytes();
        let mut curr = &self.root;
        for char_ in word {
            let index = (char_ - b'a') as usize;
            match curr.children[index] {
                Some(ref next) => {
                    curr = next;
                }
                None => {
                    return false;
                }
            }
        }
        curr.is_word
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let word = prefix.into_bytes();
        let mut curr = &self.root;
        for char_ in word {
            let index = (char_ - b'a') as usize;
            match curr.children[index] {
                Some(ref next) => {
                    curr = next;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

/**
eval leetcode testcase input

```text
输入
["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
[[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
输出
[null, null, true, false, true, null, true]
```
*/
struct TrieTestHelper(MyTrie);

impl TrieTestHelper {
    fn test(&mut self, call_func: &str, arg: &str, expected_return_val: bool) {
        dbg!(call_func, arg, expected_return_val);
        match call_func {
            "insert" => self.0.insert(arg.to_string()),
            "search" => assert_eq!(self.0.search(arg.to_string()), expected_return_val),
            "startsWith" => assert_eq!(self.0.starts_with(arg.to_string()), expected_return_val),
            _ => unreachable!(),
        };
    }
}

struct TrieTestCase<'a> {
    call_funcs: Vec<String>,
    funcs_arg: Vec<Vec<&'a str>>,
    expected_return_vals: Vec<bool>,
}

#[test]
fn test_trie() {
    let null = false;
    let test_cases = vec![TrieTestCase {
        call_funcs: vec_string![
            "insert",
            "search",
            "search",
            "startsWith",
            "insert",
            "search"
        ],
        funcs_arg: vec_vec![["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]],
        expected_return_vals: vec![null, true, false, true, null, true],
    }];
    for test_case in test_cases {
        let mut trie_test_helper = TrieTestHelper(MyTrie::new());
        for i in 0..test_case.call_funcs.len() {
            trie_test_helper.test(
                &test_case.call_funcs[i],
                test_case.funcs_arg[i][0],
                test_case.expected_return_vals[i],
            );
        }
    }
}
