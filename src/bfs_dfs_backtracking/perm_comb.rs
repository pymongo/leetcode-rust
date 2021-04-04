//! TODO
/// impl python itertools.combinations
fn comb<T>(items: Vec<T>, comb_len: usize) -> Vec<Vec<T>> {
    // let mut ret;

    // let mut curr = Vec::with_capacity(comb_len);

    // ret
    todo!()
}

#[test]
fn test_comb() {
    const TEST_CASES: [(&[i32], usize, &[&[i32]]); 1] =
        [(&[1, 2, 3], 2, &[&[1, 2], &[2, 3], &[1, 3]])];
    for &(input_nums, comb_len, output) in TEST_CASES.iter() {
        let output: Vec<Vec<i32>> = output.iter().map(|&each| each.to_vec()).collect();
        assert_eq!(comb(input_nums.to_vec(), comb_len), output);
    }
}
