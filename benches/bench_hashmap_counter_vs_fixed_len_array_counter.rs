/*! 测试 number-of-good-pairs 一题用固定长数组作为counter和HashMap作为counter的性能
number-of-good-pairs一题数值的范围在1..=100，而且长度范围也在1..=100
所谓counter值得就是类似Python collections.Counter行为的操作

除了Counter，还有已知输入全小写字母字符串需要一些记忆操作例如最长无重复子串，也建议用固定长数组，下标寻址比HashMap快

test bench_fixed_len_array_counter ... bench:          51 ns/iter (+/- 10)
test bench_hashmap_counter         ... bench:       1,772 ns/iter (+/- 158)
*/
#![feature(test, array_map)]
extern crate test;

const NUMS: [i32; 97] = [
    3, 2, 3, 5, 4, 4, 2, 3, 5, 1, 4, 3, 2, 2, 5, 3, 1, 5, 4, 5, 3, 2, 5, 2, 3, 3, 2, 1, 4, 5, 3, 3,
    5, 5, 5, 2, 5, 5, 2, 1, 4, 4, 2, 2, 3, 4, 2, 1, 5, 3, 2, 2, 2, 1, 1, 1, 3, 4, 5, 4, 5, 4, 3, 4,
    3, 4, 3, 4, 5, 1, 5, 5, 1, 2, 2, 3, 5, 2, 3, 1, 3, 5, 1, 5, 1, 4, 2, 1, 5, 5, 1, 3, 3, 2, 5, 3,
    1,
];

#[bench]
fn bench_fixed_len_array_counter(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut counter = [0u8; 101];
        for num in std::array::IntoIter::new(NUMS) {
            counter[num as usize] += 1;
        }
        // array暂不支持into_iter, issue#66145
        let _ret = counter
            .iter()
            .map(|&v| (v as i32 - 1) * v as i32 / 2)
            .sum::<i32>();
    });
}

#[bench]
fn bench_hashmap_counter(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut counter = std::collections::HashMap::with_capacity(NUMS.len());
        for num in std::array::IntoIter::new(NUMS) {
            *counter.entry(num).or_insert(0u8) += 1;
        }
        let _ret = counter
            .into_iter()
            .map(|(_k, v)| (v as i32 - 1) * v as i32 / 2)
            .sum::<i32>();
    });
}
