#[cfg(test)]
const TEST_CASE: [(i32, i32); 5] = [
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 5),
    (5, 8)
];

#[test]
fn test_tail_recursion() {
    for case in &TEST_CASE {
        assert_eq!(tail_recursion_helper(case.0), case.1);
    }
}

#[cfg(test)]
fn tail_recursion_helper(n: i32) -> i32 {
   tail_recursion(n as u32, 1, 1) as i32
}

/* 耗时0ms，内存2MB，leetcode登顶解法 */
#[cfg(test)]
fn tail_recursion(n: u32, a: u32, b: u32) -> u32 {
    if n == 1 {
        b
    } else {
        tail_recursion(n-1, b, a+b)
    }
}
