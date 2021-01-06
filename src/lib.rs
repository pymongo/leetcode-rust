#![feature(is_sorted, asm)]
#![allow(dead_code, unused_macros)]
// #![deny(warnings)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
// 如果mod backtracking写在mod macros上面，则mod backtracking无法使用macros内的所有宏
// Macros can only be used after they have been defined(macro_use)
#[macro_use]
mod macros;

mod backtracking;
mod bfs;
mod binary_search;
mod binary_tree;
mod bitwise;
mod code_snippets;
mod counter;
mod dp;
mod easy;
mod greedy;
mod linked_list;
mod math;
mod random;
mod special_data_structure;
mod uncategorized;
