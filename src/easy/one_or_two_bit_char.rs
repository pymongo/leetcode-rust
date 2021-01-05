// 以1开头的字符长度是2，以0开头的字符长度是1，问你最后一个字符属于长度为1内还是属于长度为2的字符内，直接遍历到倒数第二个字符即可
fn is_one_bit_character(bits: Vec<i32>) -> bool {
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
