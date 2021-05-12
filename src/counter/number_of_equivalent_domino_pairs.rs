/// https://leetcode.com/problems/number-of-equivalent-domino-pairs/
/// 既然nums[i][0]和nums[i][1]的取值范围都在0-9，那么可以聚合成一对2位十进制数，为了方便比较我们规定让较小的数放在十位
fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut counter = [0_u16; 100];
    let mut ret = 0;
    for each in dominoes {
        let index = if each[1] < each[0] {
            each[1] * 10 + each[0]
        } else {
            each[0] * 10 + each[1]
        } as usize;
        // 这里是C(n,2)的计数方式
        ret += counter[index] as i32;
        counter[index] += 1;
    }
    ret
}

#[test]
fn test_num_equiv_domino_pairs() {
    let test_cases = vec![(vec_vec![[1, 2], [2, 1], [3, 4], [5, 6]], 1)];
    for (input, output) in test_cases {
        assert_eq!(num_equiv_domino_pairs(input), output);
    }
}

/// 超时解答
#[cfg(not)]
fn num_equiv_domino_pairs_brute_force(dominoes: Vec<Vec<i32>>) -> i32 {
    let n = dominoes.len();
    let mut ret = 0;
    for i in 0..n {
        let (a, b) = (dominoes[i][0], dominoes[i][1]);
        let sum_a_b = a + b;
        for j in i + 1..n {
            let (c, d) = (dominoes[j][0], dominoes[j][1]);
            if c + d == sum_a_b && (a == c || a == d) {
                ret += 1;
            }
        }
    }
    ret
}
