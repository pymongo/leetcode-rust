#![feature(test)]
extern crate test;
// cargo +nightly bench --bench longest_substr_without_repeat

extern crate leetcode;
use leetcode::string::longest_substr_without_repeat::{
    i32_ascii_table, sliding_window_ascii_table2, usize_ascii_table, TEST_CASES,
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

#[bench]
fn bench_sliding_window_ascii_table2(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for case in &TEST_CASES {
            assert_eq!(usize_ascii_table(case.0.to_string()), case.1)
        }
    });
}
