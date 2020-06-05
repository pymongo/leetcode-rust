//! https://leetcode.com/problems/zigzag-conversion/

#[cfg(test)]
const TEST_CASES: [(&str, i32, &str); 2] = [
    ("LEETCODEISHIRING", 3, "LCIRETOESIIGEDHN"),
    ("LEETCODEISHIRING", 4, "LDREOEIIECIHNTSG"),
];

#[test]
fn test_zigzag() {
    for case in &TEST_CASES {
        assert_eq!(zigzag(case.0.to_owned(), case.1), case.2.to_owned());
    }
}

/*
思路1：有多少行就定义几个数组，按N遍历时挨个写入数组，最后将数组的每个char join到一起
*/
#[cfg(test)]
fn zigzag(s: String, num_rows: i32) -> String {
    let a: Vec<char>;
    let rows = num_rows as usize;
    let len = s.len();
    let mut arr: Vec<Vec<char>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        arr.push(Vec::with_capacity(len));
    }

    unimplemented!()
}

/*
将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：

L   C   I   R
E T O E S I I G
E   D   H   N
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。

请你实现这个将字符串进行指定行数变换的函数：

string convert(string s, int numRows);
示例 1:

输入: s = "LEETCODEISHIRING", numRows = 3
输出: "LCIRETOESIIGEDHN"
示例 2:

输入: s = "LEETCODEISHIRING", numRows = 4
输出: "LDREOEIIECIHNTSG"
解释:

L     D     R
E   O E   I I
E C   I H   N
T     S     G
*/
