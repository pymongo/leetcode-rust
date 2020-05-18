#![feature(test)]
extern crate test;
// cargo +nightly bench --bench longest_substr_without_repeat

extern crate leetcode;
use leetcode::string::longest_substr_without_repeat::{two_sum_btreemap, two_sum_hashmap};

#[bench]
fn bench_two_sum_btreemap(bencher: &mut test::Bencher) {
    bencher.iter(|| two_sum_btreemap(vec![-3, 4, 3, 90], 0));
}

/*
test bench_two_sum_btreemap ... bench:         345 ns/iter (+/- 33)
test bench_two_sum_hashmap  ... bench:         540 ns/iter (+/- 53)
*/
#[bench]
fn bench_two_sum_hashmap(bencher: &mut test::Bencher) {
    bencher.iter(|| two_sum_hashmap(vec![-3, 4, 3, 90], 0));
}
