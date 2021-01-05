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

/**
# LazyStatic
single_thread_immutable lazy_static stable rust implement, doesn't support no_std

## 适用于leetcode题型

int_to_roman, roman_to_int(但是这两题用python更合适，需要HashMap有序)

## 另一种泛型约束写法`pub struct LazyStaticCell<T, F: FnOnce() -> T> {`的缺点

- 缺点1: 每个impl都要写老长<T, F: FnOnce() -> T>
- 缺点2: 使用closure的语句都要多写闭包的函数签名，`static A: LazyStaticCell<i32, fn() -> i32> = LazyStaticCell::new(|| 0i32);`，可读性不好
*/
pub struct LazyStatic<T, F = fn() -> T> {
    init_once: std::sync::Once,
    /// 用Option为了让初次调用init_function初始化时能把F闭包给move掉
    init_function: std::cell::Cell<Option<F>>,
    /// 为了支持leetcode stable的rustc编译器，UnsafeCell内层不能用更好的MaybeUninit，因为MaybeUninit的write方法是unstable feature
    /// 所以这也是lazy_static源码中有个字段内是Option加上TODO注释将来会换成MaybeUninit
    data: std::cell::UnsafeCell<Option<T>>,
    /// make our struct impl auto drop?
    _marker: std::marker::PhantomData<T>,
}

unsafe impl<T> Send for LazyStatic<T> {}
unsafe impl<T> Sync for LazyStatic<T> {}

impl<T, F> LazyStatic<T, F> {
    pub const fn new(f: F) -> Self {
        Self {
            init_once: std::sync::Once::new(),
            init_function: std::cell::Cell::new(Some(f)),
            data: std::cell::UnsafeCell::new(None),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, F: FnOnce() -> T> LazyStatic<T, F> {
    /// 为了解决错误`call expression requires function`，为了能让init_function字段能call，必须改变trait bound所以不能用原来泛型约束的self
    /// cast `F = fn() -> T` to `F: FnOnce() -> T` inorder to call self.init_function
    fn get(this: &LazyStatic<T, F>) -> &T {
        this.init_once.call_once(|| {
            let data = this.init_function.take().unwrap()();
            unsafe {
                *this.data.get() = Some(data);
            }
        });
        unsafe { (*this.data.get()).as_ref().unwrap() }
    }
}

#[cfg(test)]
mod test_lazy_static {
    use super::LazyStatic;
    use std::collections::HashMap;

    static GLOBAL_MAP: LazyStatic<HashMap<i32, i32>> = LazyStatic::new(|| {
        let mut map = HashMap::new();
        map.insert(1, 1);
        map
    });

    #[test]
    fn test_my_lazy_cell() {
        let map = LazyStatic::get(&GLOBAL_MAP);
        assert_eq!(map.get(&1), Some(&1));
    }
}
