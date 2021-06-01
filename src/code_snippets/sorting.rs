/**
# 冒泡排序
思路: 第一轮从0开始前后两两比较，把较大者换到右侧，所以第一轮能把最大值放到nums[len-1]，以此类推第二轮把 第二大的放到nums[len-2]
时间复杂度:
    平均O(n^2)
    最好O(n): 需要用flag判断当前轮次是否发生交换(
    最坏O(n^2): 输入数组是逆序的
是否稳定排序: 稳定排序
*/
fn bubble_sort(nums: &mut [i32]) {
    let len = nums.len();
    for i in 0..len - 1 {
        let mut has_swap_flag = false;
        for j in 0..len - i - 1 {
            // 两两挨个比较，如果左侧较大则交换
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                has_swap_flag = true;
            }
        }
        // 如果当前轮次没有发生交换，说明数组已经有序
        if !has_swap_flag {
            return;
        }
    }
}

/**
# 选择排序
思路: 第一轮找出最小的值换到nums[0]，第二轮找到第二小的值换到nums[1]...
优化: 选择排序思路的进阶版是堆排序
时间复杂度: 平均/最好/最坏O(n^2)
是否稳定排序: 稳定排序
## 冒泡排序和选择排序
一般来说选择比冒泡效率高(benchmark结果显示选择排序一般比冒泡快3倍)，因为每轮只要交换一次
冒泡排序的好处是第一轮排序可能交换次数多，但往后的轮次会交换次数少，因为第二轮能利用上一轮的交换让数组更有序的收益
冒泡有个很大的优点就是它可以检测整个数组是否已经有序，当某次遍历没有发生任何交换的时候你就可以提前终止了
选择排序比冒泡差在，当前遍历没有利用上次遍历的结果，而冒泡排序遍历时不断将更大的数换到后面，所以冒泡排序最后的几次遍历耗时很短
*/
fn selection_sort(nums: &mut [i32]) {
    let len = nums.len();
    for i in 0..len - 1 {
        let mut min_index = i;
        for j in i + 1..len {
            if nums[j] < nums[min_index] {
                min_index = j;
            }
        }
        nums.swap(i, min_index);
    }
}

fn my_quick_sort(start: usize, end: usize, nums: &mut [i32]) {
    if start >= end {
        return;
    }
    let pivot = nums[start + (end - start) / 2];
    let mut left = start;
    let mut right = end;
    while left <= right {
        while left <= right && nums[left] < pivot {
            left += 1;
        }
        while left <= right && nums[right] > pivot {
            right -= 1;
        }
        if left <= right {
            nums.swap(left, right);
            left += 1;
            // right -= 1; // maybe overflow
            right = right.saturating_sub(1);
        }
    }
    my_quick_sort(start, right, nums);
    my_quick_sort(left, end, nums);
}

fn random_numbers_test_case() -> Vec<i32> {
    use super::random_i32::random_i32;
    use std::sync::Once;
    const NUMS_LEN: usize = 1000;
    static mut NUMS: [i32; NUMS_LEN] = [0; NUMS_LEN];
    static IS_NUMS_INIT: Once = Once::new();
    IS_NUMS_INIT.call_once(|| unsafe {
        for num in NUMS.iter_mut().take(NUMS_LEN) {
            *num = random_i32();
        }
    });
    /*
    let a = random_numbers_test_case();
    let mut b = random_numbers_test_case();
    b.sort_unstable();
    a和b会是两个独立的堆内存，b排序不会影响a
    */
    unsafe { NUMS }.to_vec()
}

#[cfg(test)]
mod tests {
    use super::{bubble_sort, my_quick_sort, random_numbers_test_case, selection_sort};

    #[test]
    fn test_bubble_sort() {
        let mut nums = random_numbers_test_case();
        bubble_sort(&mut nums);
        assert!(nums.is_sorted());
    }

    #[test]
    fn test_selection_sort() {
        let mut nums = random_numbers_test_case();
        selection_sort(&mut nums);
        assert!(nums.is_sorted());
    }

    #[test]
    fn test_my_quick_sort() {
        let mut nums = random_numbers_test_case();
        dbg!(nums.len());
        my_quick_sort(0, nums.len() - 1, &mut nums);
        assert!(nums.is_sorted());
    }

    #[test]
    fn test_my_heap() {
        use crate::code_snippets::sorting::random_numbers_test_case;
        use crate::data_structure::heap::my_max_heap::MyMaxHeap;
        let test_cases = random_numbers_test_case();
        let heap = test_cases.into_iter().collect::<MyMaxHeap<_>>();
        assert!(heap.into_sorted_vec().is_sorted());
    }
}

// cargo bench -- code_snippets::sorting::benches
#[cfg(test)]
mod benches {
    use super::{bubble_sort, my_quick_sort, random_numbers_test_case, selection_sort};

    #[bench]
    fn bench_bubble_sort(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let mut nums = random_numbers_test_case();
            bubble_sort(&mut nums);
            //assert!(nums.is_sorted());
        });
    }

    #[bench]
    fn bench_selection_sort(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let mut nums = random_numbers_test_case();
            selection_sort(&mut nums);
        });
    }

    #[bench]
    fn bench_my_heap_sort(bencher: &mut test::Bencher) {
        use crate::data_structure::heap::my_max_heap::MyMaxHeap;
        bencher.iter(|| {
            let nums = random_numbers_test_case();
            #[allow(clippy::let_underscore_drop)]
            let _ = nums.into_iter().collect::<MyMaxHeap<_>>().into_sorted_vec();
        });
    }

    #[bench]
    fn bench_my_quick_sort(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let mut nums = random_numbers_test_case();
            my_quick_sort(0, nums.len() - 1, &mut nums);
        });
    }

    #[bench]
    fn bench_std_heap_sort(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let nums = random_numbers_test_case();
            nums.into_iter()
                .collect::<std::collections::BinaryHeap<_>>()
                .into_sorted_vec();
        });
    }

    #[bench]
    fn bench_std_merge_sort(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let mut nums = random_numbers_test_case();
            #[allow(clippy::stable_sort_primitive)]
            nums.sort();
        });
    }

    #[bench]
    fn bench_std_quick_sort(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            let mut nums = random_numbers_test_case();
            nums.sort_unstable();
        });
    }
}
