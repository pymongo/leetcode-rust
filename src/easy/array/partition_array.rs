/// https://leetcode.com/problems/move-zeroes/
/// 还有一种做法是快慢双指针，慢指针指向最后一个非0元素的下一个元素，快指针往后遇到0则交换到慢指针的位置，然后慢指针前移
/// (如果快慢指针是赋值而不是交换，则最后将慢指针往后的所有元素置0)
/// move_zeros是sort_colors的简化版
fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_non_zero_next_idx = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, last_non_zero_next_idx);
            last_non_zero_next_idx += 1;
        }
    }
}

/// https://leetcode.com/problems/remove-element/
/// 这题跟move_zeros完全一样，或者说move_zeros是remove_element一题target=0的情况
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut last_unique_next_idx = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums.swap(i, last_unique_next_idx);
            last_unique_next_idx += 1;
        }
    }
    last_unique_next_idx as i32
}

#[test]
fn test_remove_duplicates() {
    assert_eq!(remove_duplicates_from_sorted_array(&mut Vec::new()), 0);
    assert_eq!(remove_duplicates_from_sorted_array(&mut vec![1, 1, 2]), 2);
}

/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
/// 用Rust的dedup API两行搞定:
/// nums.dedup();
/// nums.len() as i32
/// 用标准库的dedup性能更好，因为`Avoid bounds checks by using raw pointers`
fn remove_duplicates_from_sorted_array(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut last_unique = 0usize;
    for i in 1..n {
        if nums[i] != nums[last_unique] {
            // 先前移到下个空位(slow)，再把unique的数字换过来
            last_unique += 1;
            nums.swap(last_unique, i);
        }
    }
    last_unique as i32 + 1
}

/// https://leetcode.com/problems/sort-array-by-parity/
/// 重排数组，所有偶数元素连续的出现在前面，之后跟着所有奇数元素
/// 这题算是sort_colors的简单版，只有两种情况，而sort_colors有0,1,2三种情况
fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
    // iter_partition_in_place is unstable
    // a.iter_mut().partition_in_place(|x| x % 2 == 0);
    fn partition_solution(a: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd): (Vec<i32>, Vec<i32>) = a.into_iter().partition(|x| x % 2 == 0);
        even.append(&mut odd);
        even
    }

    fn iter_chain_solution(a: Vec<i32>) -> Vec<i32> {
        a.iter()
            .filter(|&x| x % 2 == 0)
            .chain(a.iter().filter(|&x| x % 2 == 1))
            .copied() // .map(|x| *x)
            .collect::<Vec<i32>>()
    }

    fn sort_by_mod_solution(mut a: Vec<i32>) -> Vec<i32> {
        a.sort_unstable_by_key(|x| x % 2);
        a
    }

    /// fastest in-place solution
    fn two_pointers_partition_solution(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let (mut l, mut r) = (0, n - 1);
        while l < r {
            while l < r && a[l] % 2 == 0 {
                l += 1;
            }
            while l < r && a[r] % 2 == 1 {
                r -= 1;
            }
            if l < r {
                a.swap(l, r);
            }
        }
        a
    }

    fn two_pointers_sort_color_solution(mut a: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd) = (0, a.len() - 1);
        let mut cur = 0;
        while even < odd {
            if a[cur] % 2 == 0 {
                even += 1;
                cur += 1;
            } else {
                // 参考sort_color一题，换完之后，cur指针不会前移，要判断换过来的新值是否复合偶数条件
                a.swap(cur, odd);
                odd -= 1;
            }
        }
        a
    }

    two_pointers_partition_solution(a)
}

/// https://leetcode.com/problems/sort-array-by-parity-ii/
/// In-place重排数组，使得奇数值在奇数下标，偶数值在偶数下标
fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
    fn official_solution(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut odd = 1;
        for even in (0..n).step_by(2) {
            if a[even] % 2 == 1 {
                while a[odd] % 2 == 1 {
                    odd += 2;
                }
                a.swap(even, odd);
            }
        }
        a
    }
    let n = a.len();
    let (mut even, mut odd) = (0, 1);
    while even < n && odd < n {
        while even < n && a[even] % 2 == 0 {
            even += 2;
        }
        while odd < n && a[odd] % 2 == 1 {
            odd += 2;
        }
        if even < n && odd < n {
            a.swap(even, odd);
        }
    }
    a
}
