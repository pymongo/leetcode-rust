#![feature(is_sorted, asm)]
//#![warn(clippy::pedantic,clippy::nursery,clippy::cargo,clippy::restriction)]
#![warn(clippy::nursery, clippy::cargo)]
//#![warn(clippy::restriction)]
#![warn(clippy::pedantic)]
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
    clippy::cast_possible_wrap
)]
#![allow(clippy::needless_pass_by_value)]
// cargo clippy --tests
// cargo clippy --all -- -Wclippy::cargo -Wclippy::nursery -Wclippy::pedantic
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
pub mod data_structure;
mod dp;
mod easy;
mod eval_math_expression;
mod greedy;
mod linked_list;
mod math;
mod random;
mod two_sum_two_pointers;
mod uncategorized;

/**
## transmute获取结构体的.0字段
由于Rust的孤儿规则，经常遇到 pub struct A(B); 这样包一层的结构体定义，例如mongodb::bson::DateTime就是用Unnamed/Tuple struct包了chrono::DateTime
以前我只在FFI调用openssl的md5时用过transmute
发现transmute的一种用法是将(T,)转为T
例如批量将Vec<(i32,)>转Vec<i32>而不用into_iter().map(|x| x.0)
但是transmute和map(|x| x.0)两种写法「生成的汇编代码一样」
体现了Rust的零成本抽象(同一行为不同写法编译结果一样)

pic.twitter.com/wiXrWolisN
```text
let vec_bson_datetime = vec![mongodb::bson::DateTime::from(
    "2021-05-17T00:00:00Z".parse::<chrono::DateTime<chrono::Utc>>()?,
)];

let vec_chrono_datetime: Vec<chrono::DateTime<chrono::Utc>> =
    unsafe { std::mem::transmute(vec_bson_datetime) };

assert_eq!(
    vec_chrono_datetime[0].date().naive_utc(),
    chrono::NaiveDate::from_ymd(2021, 5, 17)
);
```
## transmute获取私有字段(私有变共有)
除了transmute,指针操作也能达到类似效果
*/
#[test]
fn test_transmute() {
    dbg!(std::mem::align_of::<(i32,)>());
    dbg!(std::mem::size_of::<(i32,)>());
    let a = vec![(1,), (2,), (3,)];
    let b: Vec<i32> = unsafe { std::mem::transmute(a) };
    assert_eq!(b, vec![1, 2, 3]);
    println!("{:?}", b);
}
