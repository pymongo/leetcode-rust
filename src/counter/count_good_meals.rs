/// https://leetcode.com/problems/count-good-meals/
fn count_pairs_permutation_solution(nums: Vec<i32>) -> i32 {
    const fn is_power_of_2(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        n & (n - 1) == 0
    }
    let mut counter_map = std::collections::BTreeMap::new();
    for num in nums {
        *counter_map.entry(num).or_insert(0) += 1;
    }
    let n = counter_map.keys().len();
    let mut unique = Vec::with_capacity(n);
    let mut counter = Vec::with_capacity(n);
    for (key, value) in counter_map {
        unique.push(key);
        counter.push(i64::from(value));
    }
    // unique.sort_unstable(); // 由于nums是有序的，插入counter时也是有序的，所以不用排序
    let n = unique.len();
    let mut ret = 0_i64;
    for i in 0..n {
        for j in i..n {
            if is_power_of_2(unique[i] + unique[j]) {
                if i == j {
                    // math.comb(count, 2)
                    ret += (counter[i] - 1) * counter[i] / 2;
                } else {
                    ret += counter[i] * counter[j];
                }
            }
        }
    }
    (ret % (10_i64.pow(9) + 7)) as i32
}

/// 由于 0<=nums[i]<=2^20，所以nums[i]+nums[i]只可能是2^0..=2^21，最小是0+1，最大是2^20+2^20
/// leetcode.com版本太低不支持const fn内while loop
const fn gen_power_of_2() -> [i32; 22] {
    let mut ret = [0_i32; 22];
    let mut i = 0;
    while i < 22 {
        ret[i] = 1 << i;
        i += 1;
    }
    ret
}

/// 生成从2^0到2^N次方的等比数列
const fn gen_twos_geometric_series<const N: usize>() -> [i32; N] {
    let mut ret = [0_i32; N];
    let mut i = 0_usize;
    while i < N {
        ret[i] = 2_i32.pow(i as u32);
        i += 1;
    }
    ret
}

#[test]
fn test_gen_twos_geometric_series() {
    println!("{:?}", gen_twos_geometric_series::<22>());
}

/// 这个解法也就是把时间复杂度从O(n^2)降低到O(22n）
fn count_pairs_two_sum_solution(nums: Vec<i32>) -> i32 {
    // 照顾下leetcode.com的const fn不支持while loop
    // const TWO_SUMS: [i32; 22] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152];
    const TWO_SUMS: [i32; 22] = gen_twos_geometric_series::<22>();
    let mut counter = std::collections::BTreeMap::new();
    let mut ret = 0_i64;
    for num in nums {
        for &two_sum in &TWO_SUMS {
            let target = two_sum - num;
            if target < 0 {
                continue;
            }
            if let Some(&count) = counter.get(&target) {
                ret += i64::from(count);
            }
        }
        *counter.entry(num).or_insert(0_i32) += 1;
    }
    (ret % (10_i64.pow(9) + 7)) as i32
}

#[test]
fn test_count_pairs() {
    const TEST_CASES: [(&[i32], i32); 4] = [
        (&[1, 1, 3, 7, 15, 31, 63, 127, 255, 511], 17),
        (
            &[
                149, 107, 1, 63, 0, 1, 6867, 1325, 5611, 2581, 39, 89, 46, 18, 12, 20, 22, 234,
            ],
            12,
        ),
        (&[1, 3, 5, 7, 9], 4),
        (&[1, 1, 1, 3, 3, 3, 7], 15),
    ];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(count_pairs_permutation_solution(input.into()), output);
        assert_eq!(count_pairs_two_sum_solution(input.into()), output);
    }
}
