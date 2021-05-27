#![feature(test, asm, is_sorted)]
// cargo clippy --tests -- -Wclippy::cargo -Wclippy::nursery -Wclippy::pedantic
#![warn(clippy::nursery, clippy::cargo, clippy::pedantic)]
//#![warn(clippy::restriction)]
#![allow(
    dead_code,
    clippy::blanket_clippy_restriction_lints,
    clippy::doc_markdown,
    clippy::non_ascii_literal,
    clippy::indexing_slicing,
    clippy::match_on_vec_items,
    clippy::default_numeric_fallback,
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::unreachable,
    clippy::unwrap_used,
    clippy::cast_sign_loss,
    clippy::shadow_reuse
)]
#![allow(clippy::needless_pass_by_value)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
extern crate test;

// 如果mod backtracking写在mod macros上面，则mod backtracking无法使用macros内的所有宏
// Macros can only be used after they have been defined(macro_use)
#[macro_use]
mod macros;
mod bfs_dfs_backtracking;
mod binary_search;
mod binary_tree;
mod bitwise;
mod code_snippets;
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
