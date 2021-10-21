#![feature(test, rustc_private, asm, is_sorted, control_flow_enum)]
// cargo clippy --tests -- -Wclippy::cargo -Wclippy::nursery -Wclippy::pedantic
#![warn(clippy::nursery, clippy::cargo, clippy::pedantic, clippy::restriction)]
#![allow(
    dead_code,
    // vec_vec: use of irregular braces for `vec!` macro
    clippy::nonstandard_macro_braces,
    /* clippy::pedantic */
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::needless_pass_by_value,
    clippy::match_on_vec_items,
    clippy::non_ascii_literal,
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    /* clippy::restriction */
    clippy::blanket_clippy_restriction_lints,
    clippy::integer_division,
    clippy::integer_arithmetic,
    clippy::float_arithmetic,
    clippy::modulo_arithmetic,
    clippy::cast_sign_loss,
    clippy::as_conversions,
    clippy::default_numeric_fallback,
    clippy::pattern_type_mismatch,
    clippy::clone_on_ref_ptr,
    clippy::indexing_slicing,
    clippy::str_to_string,
    clippy::unwrap_used,
    clippy::unreachable,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::missing_docs_in_private_items,
    clippy::else_if_without_else
)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
extern crate rustc_graphviz;
extern crate rustc_lexer;
extern crate rustc_span;
extern crate test;

// Macros can only be used after they have been defined(macro_use)
#[macro_use]
mod macros;
mod bfs_dfs_backtracking;
mod binary_search;
mod binary_tree;
mod bitwise;
mod code_snippets;
mod compiler;
mod counter;
mod data_structure;
mod dp;
mod easy;
mod greedy;
mod linked_list;
mod math;
mod random;
mod two_sum_two_pointers;
mod uncategorized;
