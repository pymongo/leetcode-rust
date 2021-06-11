/*!
# Rust RFC 2169-euclidean-modulo

## -1 / 10
- Python/Ruby: -1
- C/C++/Java/Rust: 0

## -1 % 10
Python/Ruby: 9
C/C++/Java/Rust: -1

test:
ruby -e "p -1 / 10"
root -e "-1 % 10"
*/

/// https://github.com/rust-lang/rust/issues/75726
/// https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
/// rem_euclid can use in leetcode problem circular-array-loop
#[test]
#[allow(clippy::eq_op)]
fn test_rem_euclid() {
    assert_eq!(-1 / 10, 0);
    // `div_euclid` is same as python's `//`
    assert_eq!((-1_i32).div_euclid(10), -1);
    assert_eq!(-1 % 10, -1);
    // `rem_euclid` is same as python's `%`
    assert_eq!((-1_i32).rem_euclid(10), 9);
}
