/** https://leetcode.com/problems/sqrtx/
## 牛顿连续均值求根法
```python
def sqrt_newton_iterative(num: int) -> int:
    if num == 0:
        return 0
    # 牛顿法的初始值是num/2
    last_n, n = num, num / 2
    # 由于牛顿迭代法，next_n一定会比当前的n小，所以不需要加abs
    while last_n - n > 1e-3:
        last_n = n
        n = (last_n + num / last_n) / 2
    return int(n)
```
*/
fn my_sqrt(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    // 牛顿法迭代序列的第一项是x，第二项(也就初始值)是x/2，要用f64确保不会因为精度引起误差
    let num = f64::from(num);
    let (mut last_n, mut n) = (num, num / 2.0);
    while last_n - n > 1e-3 {
        last_n = n;
        n = (last_n + num / last_n) / 2.0;
    }
    n as i32
}

#[test]
fn test_my_sqrt() {
    for _ in 0..10_u32.pow(6) {
        let random_i32 = crate::code_snippets::random_i32::random_i32();
        assert_eq!(f64::from(random_i32).sqrt() as i32, my_sqrt(random_i32));
    }
}

/// https://leetcode.com/problems/valid-perfect-square/
fn is_perfect_square(num: i32) -> bool {
    my_sqrt(num).pow(2) == num
}
