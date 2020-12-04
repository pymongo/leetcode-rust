//! https://leetcode.com/problems/merge-sorted-array/

/// # 解题思路
/// ## 从后往前遍历
/// 参考一道面试题，如何将占据内存地址[0:10]的数组复制到内存地址[5:15]上
/// 首先顺序复制的话，复制到第6个时会把第1个给覆盖掉
/// 如果使用倒序复制的方法，新旧数组的指针都从后往前遍历，那就能避免重复
/// 这道题数组nums1的后半部分预留了全是0的存储空间，所以从后往前遍历时既能更新nums1又不用担心nums1出现重叠导致覆盖的问题
fn my_first_solution(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (m, n) = (m as usize, n as usize);
    let (mut p1, mut p2, mut p) = (m - 1, n.wrapping_sub(1), m + n - 1);
    while p1 != std::usize::MAX && p2 != std::usize::MAX {
        if nums1[p1] > nums2[p2] {
            nums1[p] = nums1[p1];
            p1 = p1.wrapping_sub(1);
        } else {
            nums1[p] = nums2[p2];
            p2 = p2.wrapping_sub(1);
        }
        p -= 1;
    }
    while p1 != std::usize::MAX {
        nums1[p] = nums1[p1];
        p = p.wrapping_sub(1);
        p1 = p1.wrapping_sub(1);
    }
    while p2 != std::usize::MAX {
        nums1[p] = nums2[p2];
        p = p.wrapping_sub(1);
        p2 = p2.wrapping_sub(1);
    }
}

#[test]
fn test_my_first_solution() {
    const TEST_CASES: [(&[i32], i32, &[i32], i32, &[i32]); 2] = [
        (&[1, 2, 3, 0, 0, 0], 3, &[2, 5, 6], 3, &[1, 2, 2, 3, 5, 6]),
        (&[2, 0], 1, &[1], 1, &[1, 2]),
    ];
    for &(nums1, m, nums2, n, expected) in TEST_CASES.iter() {
        let mut nums1 = nums1.to_vec();
        my_first_solution(&mut nums1, m, &mut nums2.to_vec(), n);
        assert_eq!(nums1, expected.to_vec());
    }
}
