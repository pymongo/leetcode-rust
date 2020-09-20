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
        for &(input, expected) in TEST_CASES.iter() {
            assert_eq!(i32_ascii_table(input.to_string()), expected)
        }
    });
}

#[bench]
fn bench_usize_ascii_table(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for &(input, expected) in TEST_CASES.iter() {
            assert_eq!(usize_ascii_table(input.to_string()), expected)
        }
    });
}
