#![feature(test)]
extern crate test;
/*
cargo +nightly bench --bench two_sum_hashmap_vs_btree_map
*/

extern crate leetcode;
use leetcode::collections::btree_map_two_sum::*;

/*
test bench_test_two_sum_btree_map ... bench:       1,244 ns/iter (+/- 181)
test bench_two_sum_bitwise        ... bench:       1,113 ns/iter (+/- 142)
test bench_two_sum_hashmap        ... bench:       1,628 ns/iter (+/- 402)
*/
#[bench]
fn bench_test_two_sum_btree_map(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for case in &TEST_CASES {
            let nums: Vec<i32> = case.0.iter().cloned().collect();
            let result = two_sum_btree_map(nums, case.1);
            let expected: Vec<i32> = case.2.iter().cloned().collect();
            assert_eq!(result, expected);
        }
    });
}

#[bench]
fn bench_two_sum_hashmap(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for case in &TEST_CASES {
            let nums: Vec<i32> = case.0.iter().cloned().collect();
            let result = two_sum_hashmap(nums, case.1);
            let expected: Vec<i32> = case.2.iter().cloned().collect();
            assert_eq!(result, expected);
        }
    });
}

#[bench]
fn bench_two_sum_bitwise(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for case in &TEST_CASES {
            let nums: Vec<i32> = case.0.iter().cloned().collect();
            let result = two_sum_bitwise(nums, case.1);
            let expected: Vec<i32> = case.2.iter().cloned().collect();
            assert_eq!(result, expected);
        }
    });
}
