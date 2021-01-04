#![feature(is_sorted, asm)]
#![allow(dead_code)]
// #![deny(warnings)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
mod backtracking;
mod bfs;
mod binary_search;
mod binary_tree;
mod bitwise;
mod code_snippets;
mod dp;
mod easy;
mod greedy;
mod linked_list;
mod random;
mod special_data_structure;
mod string;
mod union_find;

pub fn parse_2d_array(s: &str) -> Vec<Vec<i32>> {
    let len = s.len();
    let s = &s[1..len - 1];
    let mut ret = vec![];
    for nums_str in s.split("],") {
        let nums_str = nums_str.trim().replace('[', "").replace(']', "");
        let mut nums = vec![];
        for num in nums_str.split(',') {
            if num.is_empty() {
                continue;
            }
            nums.push(num.trim().parse::<i32>().unwrap());
        }
        ret.push(nums);
    }
    ret
}

#[test]
fn test_parse_2d_array() {
    assert_eq!(
        parse_2d_array("[[17,2],[-31],[3],[]]"),
        vec![vec![17, 2], vec![-31], vec![3], vec![]]
    );
}
