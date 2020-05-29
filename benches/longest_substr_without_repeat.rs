#![feature(test)]
extern crate test;
// cargo +nightly bench --bench longest_substr_without_repeat

extern crate leetcode;
use leetcode::string::longest_non_repeated_substr::{
    i32_ascii_table, usize_ascii_table, TEST_CASES,
};

#[bench]
fn bench_i32_ascii_table(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for case in &TEST_CASES {
            assert_eq!(i32_ascii_table(case.0.to_string()), case.1)
        }
    });
}

#[bench]
fn bench_usize_ascii_table(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for case in &TEST_CASES {
            assert_eq!(usize_ascii_table(case.0.to_string()), case.1)
        }
    });
}
