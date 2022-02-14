/// https://leetcode.com/problems/design-add-and-search-words-data-structure/
#[derive(Default)]
struct WordDictionary {
    children: [Option<Box<Self>>; 26],
    is_word: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;
        for letter in word.into_bytes().into_iter().map(|ch| (ch - b'a') as usize) {
            node = node.children[letter].get_or_insert_with(|| Box::new(Self::default()));
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self);
        for letter in word.into_bytes() {
            if letter == b'.' {
                for _ in 0..queue.len() {
                    let cur = queue.pop_front().unwrap();
                    for next in cur.children.iter().flatten() {
                        queue.push_back(next);
                    }
                }
            } else {
                let index = usize::from(letter - b'a');
                for _ in 0..queue.len() {
                    if let Some(ref next) = queue.pop_front().unwrap().children[index] {
                        queue.push_back(next);
                    }
                }
            }
            if queue.is_empty() {
                return false;
            }
        }

        queue.into_iter().any(|x| x.is_word)
    }
}

#[cfg(FALSE)]
impl std::ops::Index<usize> for WordDictionary {
    type Output = Option<Box<Self>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}

/** https://leetcode.com/problems/implement-trie-prefix-tree/
Trie的同义词: 前缀树、字典树
前缀树每个节点只有一个字符，root相当于一个dummyHead不存储字符
每个节点除了要存储字符，还要存储从根到当前节点是否构成一个单词

## 前缀树的优点
插入和查找的时间复杂度都是O(L), L为单词长度

## 前缀树的应用
1. 搜索框自动补全或提供下拉选项的提示，例如谷歌搜索rust，下面会有很多rust开头的候选词的提示
2. 9宫格/9键手机输入时，联想候选词
3. Typo/拼写检查
4. Boggle单词游戏, 给你一个乱序的字母矩阵，从矩阵中任意一点往上下左右四个方向去搜索，能找到几个单词
5. IP 路由 (最长前缀匹配)
*/
/// 生产环境下的Trie会更复杂，需要类似并查集那种路径压缩，否则一个`aaaaa`这样的单词会创建高度为5的树，所以会有一些trie算法让这颗树更扁
#[derive(Default)]
struct TrieLeetcode {
    /// 为了简便，我们的Trie仅支持「小写字母」
    children: [Option<Box<Self>>; 26],
    /// 如果用 HashMap 去存children，则会插入一个('$', Trie::new())表示该节点是个单词的结尾
    is_word: bool,
}

impl TrieLeetcode {
    fn new() -> Self {
        Self::default()
    }

    /**
    ## not ergonomics code example
    ```text
    fn insert(&mut self, word: String) {
        let word = word.into_bytes();
        let mut curr = &mut self.root;
        for letter in word {
            let index = (letter - b'a') as usize;
            match curr.children[index] {
                Some(ref mut next) => {
                    curr = next;
                }
                None => {
                    curr.children[index] = Some(Box::new(MyTrieNode::new()));
                    curr = curr.children[index].as_mut().unwrap();
                }
            }
        }
        curr.is_word = true;
    }
    ```
    */
    fn insert(&mut self, word: String) {
        // 将所有大写字母转为小写字母，leetcode提交不需要转，读操作系统`/usr/share/dicts/words`需要全转小写
        // let word = word.to_ascii_lowercase().into_bytes();
        let mut node = self;
        for letter in word.into_bytes().into_iter().map(|ch| (ch - b'a') as usize) {
            node = node.children[letter].get_or_insert_with(|| Box::new(Self::default()));
        }
        // 单词结尾标志，避免插入apple时会把app也认为是一个单词
        node.is_word = true;
    }

    fn _find_node(&self, word: &str) -> Option<&Self> {
        let mut node = self;
        for letter in word.as_bytes().iter().map(|ch| (ch - b'a') as usize) {
            node = node.children[letter].as_ref()?;
        }
        Some(node)
    }

    fn _find_node_mut(&mut self, word: &str) -> Option<&mut Self> {
        let mut node = self;
        for letter in word.as_bytes().iter().map(|ch| (ch - b'a') as usize) {
            node = node.children[letter].as_mut()?;
        }
        Some(node)
    }

    /**
    ## not ergonomics code example
    ```text
    fn search(&self, word: String) -> bool {
        let word = word.into_bytes();
        let mut curr = &self.root;
        for letter in word {
            let index = (letter - b'a') as usize;
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
    ```
    */
    fn search(&self, word: String) -> bool {
        self._find_node(&word).map_or(false, |node| node.is_word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self._find_node(&prefix).is_some()
    }

    fn delete_word(&mut self, word: String) -> bool {
        let node_opt = self._find_node_mut(&word);
        match node_opt {
            Some(node) => {
                node.is_word = false;
                true
            }
            None => false,
        }
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
struct TrieTestHelper(TrieLeetcode);

impl TrieTestHelper {
    fn test(&mut self, call_func: &str, arg: &str, expected_return_val: bool) {
        match call_func {
            "insert" => self.0.insert(arg.to_string()),
            "search" => assert_eq!(self.0.search(arg.to_string()), expected_return_val),
            "startsWith" => assert_eq!(self.0.starts_with(arg.to_string()), expected_return_val),
            _ => unreachable!(),
        };
    }
}

#[allow(clippy::single_char_lifetime_names)]
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
        let mut trie_test_helper = TrieTestHelper(TrieLeetcode::new());
        for i in 0..test_case.call_funcs.len() {
            trie_test_helper.test(
                &test_case.call_funcs[i],
                test_case.funcs_arg[i][0],
                test_case.expected_return_vals[i],
            );
        }
    }
}
