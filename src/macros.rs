/// https://docs.rs/rustgym-util/0.2.4/src/rustgym_util/data.rs.html#2-6
macro_rules! vec_vec {
    [$($token_tree:tt),* $(,)?] => {
        vec![$(vec!$token_tree),*]
    };
}

#[test]
fn test_vec_vec_i32() {
    let expected = vec![vec![17, 2], vec![-31], vec![3], vec![]];
    assert_eq!(vec_vec![[17, 2], [-31], [3], []], expected);
    assert_eq!(vec_vec!([17, 2], [-31], [3], [],), expected);
    assert_eq!(vec_vec! {[17,2],[-31],[3],[],}, expected);
}
