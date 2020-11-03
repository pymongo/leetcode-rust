struct Solution;

impl Solution {
    // 以1开头的字符长度是2，以0开头的字符长度是1，问你最后一个字符是长度为1呢还是2呢，遍历到n-1个字符即可
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len();
        let last_idx = n - 1;
        let mut i = 0usize;
        while i < last_idx {
            if bits[i] == 0 {
                i += 1;
            } else {
                i += 2;
            }
        }
        i == last_idx
    }
}
