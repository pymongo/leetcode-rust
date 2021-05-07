#![feature(is_sorted, asm)]
#![allow(dead_code)]
//#![warn(clippy::pedantic,clippy::nursery,clippy::cargo,clippy::restriction)]
#![warn(clippy::nursery, clippy::cargo)]
// cargo clippy --all -- -Wclippy::restriction -Wclippy::cargo -Wclippy::nursery -Wclippy::pedantic
#![doc(html_playground_url = "https://play.rust-lang.org/")]
// 如果mod backtracking写在mod macros上面，则mod backtracking无法使用macros内的所有宏
// Macros can only be used after they have been defined(macro_use)
#[macro_use]
mod macros;
mod bfs_dfs_backtracking;
mod binary_search;
mod binary_tree;
mod bitwise;
pub mod code_snippets;
mod counter;
mod data_structure;
mod dp;
mod easy;
mod eval_math_expression;
mod greedy;
mod linked_list;
mod math;
mod random;
mod two_sum_two_pointers;
mod uncategorized;
