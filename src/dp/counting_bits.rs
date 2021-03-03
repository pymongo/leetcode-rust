/// https://leetcode.com/problems/counting-bits/
fn count_bits(num: i32) -> Vec<i32> {
    let num = num as usize;
    let mut ret = Vec::with_capacity(num + 1);
    for n in 0..=num {
        ret.push(n.count_ones() as i32);
    }
    ret
}

fn count_bits_dp(num: i32) -> Vec<i32> {
    let num = (num + 1) as usize;
    let mut ret = vec![0; num];
    for n in 1..num {
        ret[n] = ret[(n - 1) & n] + 1;
    }
    ret
}
