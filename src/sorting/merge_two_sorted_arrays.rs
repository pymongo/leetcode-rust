//! https://leetcode.com/problems/merge-sorted-array/

#[cfg(test)]
const TEST_CASES: [(&[i32], i32, &[i32], i32, &[i32]); 3] = [
    (&[1, 2, 3, 0, 0, 0], 3, &[2, 5, 6], 3, &[1, 2, 2, 3, 5, 6]),
    (&[2, 0], 1, &[1], 1, &[1, 2]),
    (&[0], 0, &[1], 1, &[1]),
];

#[test]
fn test_my_first_solution() {
    for case in &TEST_CASES {
        let mut nums1: Vec<i32> = case.0.iter().cloned().collect();
        let mut nums2: Vec<i32> = case.2.iter().cloned().collect();
        let result: Vec<i32> = case.4.iter().cloned().collect();
        my_first_solution(&mut nums1, case.1, &mut nums2, case.3);
        assert_eq!(nums1, result);
    }
}

/// # 解题思路
/// ## 从后往前遍历
/// 参考一道面试题，如何将占据内存地址[0:10]的数组复制到内存地址[5:15]上
/// 首先顺序复制的话，复制到第6个时会把第1个给覆盖掉
/// 如果使用倒序复制的方法，新旧数组的指针都从后往前遍历，那就能避免重复
/// 这道题数组nums1的后半部分预留了全是0的存储空间，所以从后往前遍历时既能更新nums1又不用担心nums1出现重叠导致覆盖的问题
#[cfg(test)]
fn my_first_solution(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    }
    let mut nums1_read_index: usize = 0_i32.max(m - 1) as usize;
    let mut nums2_read_index: usize = (n - 1) as usize;
    let mut nums1_write_index: usize = (m + n - 1) as usize;
    let mut is_nums2_remain = true;
    // 利用nums1和nums2是有序的特点，从后往前遍历
    loop {
        if nums1[nums1_read_index] > nums2[nums2_read_index] {
            nums1[nums1_write_index] = nums1[nums1_read_index];
            if nums1_read_index == 0 {
                break;
            }
            nums1_read_index -= 1;
        } else {
            nums1[nums1_write_index] = nums2[nums2_read_index];
            if nums2_read_index == 0 {
                is_nums2_remain = false;
                break;
            }
            nums2_read_index -= 1;
        }
        nums1_write_index -= 1;
    }
    // 如果nums1从后往前遍历完的时候，nums2还有剩余项
    // 则把nums2[0:nums2_read_index]赋值/复制到nums1[0:nums2_read_index]中
    if is_nums2_remain {
        for i in 0..=nums2_read_index {
            nums1[i] = nums2[i]
        }
    }
}

/* Java
// while there are still elements to compare
while ((p1 >= 0) && (p2 >= 0))
  // compare two elements from nums1 and nums2
  // and add the largest one in nums1
  nums1[p--] = (nums1[p1] < nums2[p2]) ? nums2[p2--] : nums1[p1--];

// add missing elements from nums2
System.arraycopy(nums2, 0, nums1, 0, p2 + 1);
*/
