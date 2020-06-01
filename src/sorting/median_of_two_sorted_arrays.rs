//! 2 Solutions: binary_serch_kth_min、array_devider

#[cfg(test)]
const TEST_CASES: [(&[i32], &[i32], f64); 5] = [
    (&[1, 2, 3, 4], &[3, 6, 8, 9], 3.5),
    (&[-2, -1], &[3], -1_f64),
    (&[1, 3], &[2], 2_f64),
    (&[3], &[-2, -1], -1_f64),
    (&[1,2], &[3, 4], 2.5),
];

#[test]
fn test_move_divider_of_two_arrays() {
    for case in &TEST_CASES {
        dbg!(&case);
        let nums1: Vec<i32> = case.0.iter().cloned().collect();
        let nums2: Vec<i32> = case.1.iter().cloned().collect();
        assert_eq!(find_median_sorted_arrays(nums1, nums2), case.2);
    }
}

/*
https://www.youtube.com/watch?v=ScCg9v921ns
数组A、B初始化时都在A、B中间设一个分割线
先不考虑奇数的情况，让代码简单点，自己尝试敲一下，不然干看答案也不理解
初始化的分割线如图
固定输入用例1：[1 2|3 4]
固定输入用例2：[3 6|8 9]
交叉比较：如果同时满足 a_mid_left(2) <= b_mid_right(8)
                && b_mid_left(6) <= a_mid_left(3)
则遍历结束，输出答案
否则移动分割线到如下位置去满足条件，利用两个数组是有序，得出上述交叉比较的遍历终止条件
移动分割线后输入用例1：[1 2 3|4]
移动分割线后输入用例2：[3|6 8 9]，刚好两个数组分割线的左半边组成了合并后中位数的左半边
时间复杂度O(logn)，而尾递归二分查找第k小的项的时间复杂度是O(log(m+n))

流程：移动较短数组的分割线
*/
// move_divider_of_two_arrays
#[cfg(test)]
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (len_a, len_b) = (nums1.len(), nums2.len());
    // 保证数组a的长度更短，我们遍历较短的数组a节约时间，加上二分查找后使得时间复杂度降低到O(log(min(m,n)))
    if len_a > len_b {
        return find_median_sorted_arrays(nums2, nums1);
    }

    // 如果较短数组的长度是0，统一了较长数组长度是奇数偶数的情况
    if len_a == 0 {
        return (nums2[(len_b-1)/2] as f64 + nums2[len_b/2] as f64) / 2_f64;
    }
    // 如果较短数组的长度是1，则较短数组的分割线左边或右边会没有值，这是一种特殊的边界条件
    if len_a == 1 {
        let mut nums_b = nums2;
        let insert_index = match nums_b.binary_search(&nums1[0]) {
            Ok(index) => index,
            Err(index) => index,
        };
        nums_b.insert(insert_index, nums1[0]);
        return (nums_b[len_b/2] as f64 + nums_b[(len_b+1)/2] as f64) / 2_f64;
    }

    // 往后的情况，nums1和nums2的长度至少为2
    let total_len = len_a + len_b;

    let (mut a_divider_left, mut a_divider_right) = ((len_a / 2) - 1, len_a / 2);
    let (mut b_mid_left, mut b_mid_right) = ((len_b / 2) - 1, len_b / 2);
    loop {
        if nums1[a_divider_left] > nums2[b_mid_right] {
            // a的右半边太大了！b的分割线左移一位、a的分割线右移一位
            a_divider_left -= 1;
            a_divider_right -= 1;
            b_mid_left += 1;
            b_mid_right += 1;
        } else if nums2[b_mid_left] > nums1[a_divider_right] {
            // b的右半边太大了！b的分割线左移一位、a的分割线右移一位
            b_mid_left -= 1;
            b_mid_right -= 1;
            a_divider_left += 1;
            a_divider_right += 1;
        } else {
            return (std::cmp::max(nums1[a_divider_left], nums2[b_mid_left])
                + std::cmp::min(nums1[a_divider_right], nums2[b_mid_right])) as f64
                / 2_f64;
        }
    }
}

#[test]
fn test_my_binary_search_kth() {
    for case in &TEST_CASES {
        let nums1: Vec<i32> = case.0.iter().cloned().collect();
        let nums2: Vec<i32> = case.1.iter().cloned().collect();
        assert_eq!(my_binary_search_kth(nums1, nums2), case.2);
    }
}

/// ## 二分搜索的思路
///
/// 1. 从nums1和nums2中逐步剔除元素/将元素放入中位数左半部分的候选区
/// 2. 更新k的值为k=k-k/2(这也是为什么叫二分/折半搜索的原因)
/// 3. 直到被剔除了一半数量(也就是k变成1)，否则继续循环
///
/// ### 初始条件
/// 以[1, 2, 3, 4], [3, 6, 8, 9]的测试用例为例
/// 前提条件：确保nums1的长度更小，这样遍历时更节约时间
/// len1=4, len2=4, k=4(左中位数)
///
/// ### 遍历过程
/// round 1:
/// k=4, nums1_idx=1, nums2_idx=1
/// nums1[1](2) < nums2[1](6) => nums1的1和2被剔除
/// (反之，如果是nums1的元素更大，就剔除nums2的元素)
/// 将k的值更新为k=k-k/2
///
/// round 2:
/// k=2, nums1_idx=2, nums2_idx=1
/// nums1[2](3) < nums2[1](6) => nums1的3被剔除
/// k=2-1，达成break的条件，跳出循环
///
/// 如果是偶数个元素：
/// 左中位数是nums1[nums1_idx]和nums2[nums2_idx]的最大值
/// 右中位数是nums1[nums1_idx+1]和nums2[nums2_idx+1]的最大值
/// 需要对nums1或nums2为空的情况做处理
///
/// ### TODO
///
/// 没有考虑各种边际情况，leetcode上部分测试用例不通过
/// 没有考虑nums1或nums2为空的情况，没有考虑nums1一个元素都不取和nums2一个元素都不取的情况
///
#[cfg(test)]
fn my_binary_search_kth(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (len1, len2) = (nums1.len(), nums2.len());
    // 如果是奇数，恰好是中位数，如果是偶数，则是左中位数
    let mut k = (len1 + len2 + 1) / 2;
    let mut half_k = k / 2;
    // 已经发现的/剔除的中位数左边的元素的个数
    // let mut median_left_items_count = 0;
    // nums1_index
    let mut i = half_k - 1;
    // nums2_index
    let mut j = i;
    loop {
        if nums1[i] > nums2[j] {
            // 将nums2的前half_k项收纳入median_left_items
            j += 1;
            // 由于我们 "删除" 了一些元素(这些元素都比第 k 小的元素要小)，
            // 因此需要修改 k 的值，减去删除的数的个数
            k = half_k;
            half_k = k / 2;
            i = half_k - 1;
        } else {
            // 将nums1的前half_k项收纳入median_left_items
            i += 1;
            k = half_k;
            half_k = k / 2;
            j = half_k - 1;
        }
        if k == 1 {
            break;
        }
        // median_left_items_count += half_k;
    }
    if (len1 + len2) % 2 == 0 {
        (nums1[i] + nums2[j]) as f64 / 2_f64
    } else {
        std::cmp::min(nums1[i], nums2[j]) as f64
    }
}

#[test]
fn test_my_brute_force() {
    for case in &TEST_CASES {
        let nums1: Vec<i32> = case.0.iter().cloned().collect();
        let nums2: Vec<i32> = case.1.iter().cloned().collect();
        assert_eq!(my_brute_force(nums1, nums2), case.2);
    }
}

/*
思路：遍历小的数组，二分插入到大的数组中
能跑进0ms，但是不是每次都稳进0ms
*/
#[cfg(test)]
fn my_brute_force(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len1 = nums1.len();
    let mut len2 = nums2.len();
    let total_len = len1+len2;
    let median_left_part_final_count = total_len / 2;
    let shorter_nums: Vec<i32>;
    let mut longer_nums: Vec<i32>;
    if len2 > len1 {
        longer_nums = nums2;
        shorter_nums = nums1;
        len2 = len1;
    } else {
        longer_nums = nums1;
        shorter_nums = nums2;
    }
    // 已经挑选出组成中位数的左半部分的个数
    // 二分搜索nums2的第一个元素
    // let mut median_left_part_current_count;
    let mut binary_search_index;
    for j in 0..len2 {
        binary_search_index = match longer_nums.binary_search(&shorter_nums[j]) {
            Ok(index) => index,
            Err(index) => index
        };
        longer_nums.insert(binary_search_index, shorter_nums[j]);
        // 以下写法会漏掉测试用例1的元素nums1的4
        // median_left_part_current_count = binary_search_index +j;
        // if median_left_part_current_count >= median_left_part_final_count {
        //     break;
        // }
    }
    return if total_len % 2 == 0 {
        (longer_nums[median_left_part_final_count] + longer_nums[median_left_part_final_count - 1]) as f64 / 2_f64
    } else {
        longer_nums[median_left_part_final_count] as f64
    }
}

/*
思路：遍历小的数组，冒泡排序搬的插入到大的数组中
我首次Accept该题的提交，现在看来思路很乱而且优化空间很大
这么笨的算法rust都能跑进4ms，Rust的性能太强了
执行用时 : 4 ms, 在所有 Rust 提交中击败了72.09%的用户
内存消耗 : 2 MB, 在所有 Rust 提交中击败了100.00%的用户
*/
#[cfg(not)]
pub fn my_brute_force_old(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let ans: f64;
    let mut len = nums1.len();
    let len1 = nums1.len();
    let mut len2 = nums2.len();
    let total_sum = nums1.len() + nums2.len();
    let stop_index = total_sum / 2;
    let mut i = 0;
    let mut j = 0;
    let mut arr: Vec<i32>;
    let pending_to_insert_arr: Vec<i32>;
    if len1 > len2 {
        arr = nums1;
        pending_to_insert_arr = nums2;
    } else {
        arr = nums2;
        pending_to_insert_arr = nums1;
        len = len2;
        len2 = len1;
    }
    // 奇偶数判断
    while j < len2 {
        // 既然第二个数组是有序的，我就不用二分插入了
        // while arr[std::cmp::min(i, len - 1)] < pending_to_insert_arr[j] {
        // TODO 这个while比二分查找蠢多了
        while arr[std::cmp::min(i, len - 1)] < pending_to_insert_arr[j] {
            if i < len {
                i += 1;
            } else {
                break;
            }
        }
        if i < len {
            arr.insert(i, pending_to_insert_arr[j]);
        } else {
            arr.push(pending_to_insert_arr[j]);
            i += 1;
        }
        j += 1;
        len += 1;
    }
    if total_sum % 2 == 0 {
        ans = (arr[stop_index] + arr[stop_index - 1]) as f64 / 2_f64;
    } else {
        ans = arr[stop_index] as f64;
    }
    ans
}

// 全球服第一、二分查找第k小的元素的算法
#[cfg(not)]
use std::cmp::Ordering;

#[cfg(not)]
fn search_sep_idx(n1: &Vec<i32>, n2: &Vec<i32>) -> usize {
    let (mut left, mut right) = (0, n1.len() + 1);
    while (right > left) {
        let mid = (left + right) / 2;
        match check_sep(n1, n2, mid) {
            Ordering::Equal => return mid,
            Ordering::Greater => right = mid,
            Ordering::Less => left = mid + 1,
        }
    }
    unreachable!()
}

#[cfg(not)]
fn get_min_maxs(n1: &Vec<i32>, n2: &Vec<i32>, sep_idx1: usize) -> (i32, i32, i32, i32) {
    let final_len = n1.len() + n2.len();
    let sep_idx2 = final_len / 2 - sep_idx1;
    let left_max1 = if sep_idx1 == 0 {
        std::i32::MIN
    } else {
        n1[sep_idx1 - 1]
    };
    let left_max2 = if sep_idx2 == 0 {
        std::i32::MIN
    } else {
        n2[sep_idx2 - 1]
    };

    let right_min1 = if sep_idx1 == n1.len() {
        std::i32::MAX
    } else {
        n1[sep_idx1]
    };
    let right_min2 = if sep_idx2 == n2.len() {
        std::i32::MAX
    } else {
        n2[sep_idx2]
    };

    (left_max1, left_max2, right_min1, right_min2)
}

#[cfg(not)]
fn check_sep(n1: &Vec<i32>, n2: &Vec<i32>, sep_idx1: usize) -> Ordering {
    let (left_max1, left_max2, right_min1, right_min2) = get_min_maxs(n1, n2, sep_idx1);
    let left_max = std::cmp::max(left_max1, left_max2);
    let right_min = std::cmp::min(right_min1, right_min2);
    if left_max <= right_min {
        Ordering::Equal
    } else if left_max1 > right_min {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

#[cfg(not)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        return Self::find_median_sorted_arrays(nums2, nums1);
    }
    let sep1 = search_sep_idx(&nums1, &nums2);
    let (left_max1, left_max2, right_min1, right_min2) = get_min_maxs(&nums1, &nums2, sep1);
    let left_max = std::cmp::max(left_max1, left_max2);
    let right_min = std::cmp::min(right_min1, right_min2);
    if (nums1.len() + nums2.len()) % 2 == 0 {
        // even
        (left_max + right_min) as f64 / 2.0
    } else {
        // odd
        right_min as f64
    }
}

// 国服第一、二分查找第k小的元素的算法
#[cfg(not)]
fn get_min(x: usize, y: usize) -> usize {
    if x > y {
        return y;
    } else {
        return x;
    }
}

#[cfg(not)]
fn get_kth(
    nums1: &Vec<i32>,
    start1: usize,
    end1: usize,
    nums2: &Vec<i32>,
    start2: usize,
    end2: usize,
    k: usize,
) -> i32 {
    let len1 = end1 - start1 + 1;
    let len2 = end2 - start2 + 1;
    if len1 > len2 {
        return Solution::getKth(nums2, start2, end2, nums1, start1, end1, k);
    }

    if len1 == 0 {
        return nums2[start2 + k - 1];
    }

    if k == 1 {
        let mut temp1 = nums1[start1];
        let mut temp2 = nums2[start2];
        if temp1 > temp2 {
            return temp2;
        } else {
            return temp1;
        }
    }

    let i = start1 + Solution::get_min(len1, k / 2) - 1;
    let j = start2 + Solution::get_min(len2, k / 2) - 1;
    if nums1[i] > nums2[j] {
        return Solution::getKth(
            nums1,
            start1,
            end1,
            nums2,
            j + 1,
            end2,
            k - (j - start2 + 1),
        );
    } else {
        return Solution::getKth(
            nums1,
            i + 1,
            end1,
            nums2,
            start2,
            end2,
            k - (i - start1 + 1),
        );
    }
}

#[cfg(not)]
pub fn find_median_sorted_arrays_china_best(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n = nums1.len();
    let m = nums2.len();

    let left = (n + m + 1) / 2;
    let right = (n + m + 2) / 2;

    let temp: f64 = 0.5;

    return (Solution::getKth(&nums1, 0, n - 1, &nums2, 0, m - 1, left)
        + Solution::getKth(&nums1, 0, n - 1, &nums2, 0, m - 1, right)) as f64
        * temp;
}
