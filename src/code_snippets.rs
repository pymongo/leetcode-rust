/**
# 编译型语言和脚本语言在一些计算上的差异
## -1 / 10
Python/Ruby: -1
Java/Rust: 0

## -1 % 10
Python/Ruby: 9
Java/Rust: -1
*/
#[test]
fn test_partial_reverse() {
    assert_eq!(1u8.rotate_left(1), 0b0000_0010);
    let mut nums = [1, 2, 3, 4];
    nums[1..=2].rotate_left(1);
    assert_eq!(nums, [1, 3, 2, 4]);
    let mut nums = [1, 2, 3, 4];
    nums[1..=2].reverse();
    assert_eq!(nums, [1, 3, 2, 4]);
}

#[test]
fn iter_once_both_max_and_min() {
    let nums = vec![1i32, 2, 3, 4, 5];
    let (max, min) = nums.iter().fold((i32::MIN, i32::MAX), |(max, min), &x| {
        (max.max(x), min.min(x))
    });
    assert_eq!(max, *nums.iter().max().unwrap());
    assert_eq!(min, *nums.iter().min().unwrap());
}
