//! https://leetcode.com/problems/median-of-two-sorted-arrays/
//! TODO 这题可读性最好最容易记的应该是find_kth递归的解法，我写的Rust解法太长了可读性并不好，建议看我写的Python解法版本

struct Solution;

impl Solution {
    /// [4ms, O(n*logn)]即便用了两个数组合并后排序的完全没利用上两个数组已经有序的笨方法，Rust的性能还是能跑进4ms
    fn my_brute_force(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() < nums2.len() {
            return Self::my_brute_force(nums2, nums1);
        }
        nums1.append(&mut nums2);
        nums1.sort_unstable();
        let len = nums1.len();
        if len % 2 == 0 {
            (nums1[len / 2 - 1] + nums1[len / 2]) as f64 / 2f64
        } else {
            nums1[len / 2] as f64
        }
    }

    /// [0ms, O(n)]既然两个数组已经有序，那么可以用归并排序的归并操作去合并数组提升性能，使用二分法能达到更快的logn时间复杂度
    fn merge_sort_solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n, mut i, mut j) = (nums1.len(), nums2.len(), 0usize, 0usize);
        let len = m + n;
        let mut merged = Vec::with_capacity(len);
        while i < m && j < n {
            if nums1[i] <= nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }
        while i < m {
            merged.push(nums1[i]);
            i += 1;
        }
        while j < n {
            merged.push(nums2[j]);
            j += 1;
        }
        if len % 2 == 0 {
            (merged[len / 2 - 1] + merged[len / 2]) as f64 / 2f64
        } else {
            merged[len / 2] as f64
        }
    }
}

#[cfg(test)]
mod test_find_median_sorted_arrays {
    use super::Solution;

    const TESTCASES: [(&[i32], &[i32], f64); 17] = [
        (&[1, 2, 3, 4], &[3, 6, 8, 9], 3.5),
        (&[1, 5, 6, 7], &[2, 3, 4, 8], 4.5),
        (&[1, 2], &[3, 4, 5, 6, 7], 4f64),
        (&[4, 5, 6], &[1, 2, 3], 3.5),
        (&[4, 5], &[1, 2, 3, 6], 3.5),
        (&[1, 3], &[2, 4, 5, 6], 3.5),
        (&[1, 2], &[3, 4, 5, 6], 3.5),
        (&[1, 3], &[2, 4, 5], 3f64),
        (&[1, 2, 3], &[4, 5], 3f64),
        (&[3, 4], &[1, 2, 5], 3f64),
        (&[-2, -1], &[3], -1f64),
        (&[1, 3], &[2], 2f64),
        (&[3], &[-2, -1], -1f64),
        (&[1, 2], &[3, 4], 2.5),
        (&[4, 5], &[1, 2, 3], 3f64),
        (&[1, 2], &[1, 2, 3], 2f64),
        (&[1, 2, 3], &[1, 2, 3], 2f64),
    ];

    #[test]
    fn test_my_brute_force() {
        for &(nums1, nums2, expected) in TESTCASES.iter() {
            assert_eq!(
                Solution::my_brute_force(nums1.to_vec(), nums2.to_vec()),
                expected
            );
        }
    }

    #[test]
    fn test_merge_sort_solution() {
        for &(nums1, nums2, expected) in TESTCASES.iter() {
            assert_eq!(
                Solution::merge_sort_solution(nums1.to_vec(), nums2.to_vec()),
                expected
            );
        }
    }
}

/// [0ms, O(logn)] solution, but too long and hard to read
/// TODO Refactor, use checked_sub to detect usize overflow?
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (len_a, len_b) = (nums1.len(), nums2.len());
    // 保证数组a的长度更短，我们遍历较短的数组a节约时间，加上二分查找后使得时间复杂度降低到O(log(min(m,n)))
    if len_a > len_b {
        // 如果在leetcode的提交中，这里要写成Self::xxx或Solution::xxx才能够递归
        return find_median_sorted_arrays(nums2, nums1);
    }

    // 如果较短数组的长度是0，统一了较长数组长度是奇数偶数的情况
    if len_a == 0 {
        return (nums2[(len_b - 1) / 2] as f64 + nums2[len_b / 2] as f64) / 2_f64;
    }
    // 如果较短数组的长度是1，则较短数组的分割线左边或右边会没有值，这是一种特殊的边界条件
    if len_a == 1 {
        let mut nums_b = nums2;
        let insert_index = match nums_b.binary_search(&nums1[0]) {
            Ok(index) => index,
            Err(index) => index,
        };
        nums_b.insert(insert_index, nums1[0]);
        return (nums_b[len_b / 2] as f64 + nums_b[(len_b + 1) / 2] as f64) / 2_f64;
    }

    let total_len = len_a + len_b;
    // 如果是总数是奇数，中位数左边部分会多包含一个元素
    let half_len = (total_len + 1) / 2;

    // 往后的情况，nums1和nums2的长度至少为2，

    // 不能记分隔线左边元素的索引，如果分隔线在最左边，则索引会是-1导致usize溢出报错
    // 边界条件：a_divider_right_index=0时分隔线在最左边；a_divider_right_index=len_a时分隔线在最右边
    let mut a_divider_right_index;
    let mut b_divider_right_index;

    // 折半查找的左右游标
    let (mut a_left, mut a_right) = (0, len_a);
    let mut a_divider_left: i32;
    let mut a_divider_right: i32;
    let mut b_divider_left: i32;
    let mut b_divider_right: i32;
    /* 初始条件
    总数是奇数个时：[3|4], [1 2|5]
    */
    loop {
        // 如果a是[1,2,3,4]: (a_divider_left, a_divider_right) = (1,2)
        // 注意这里是加，如果分隔线左边过小，如[1,2,3,4]、[3,6,8,9]，第二次循环时nums1的分隔线会从2|3移到3|4
        a_divider_right_index = (a_right + a_left) / 2;
        if a_divider_right_index == 0 {
            a_divider_right_index = 1;
        }
        // 如果a和b都用的是分隔线右边的索引的话，b_divider_right_index不需要减一
        b_divider_right_index = half_len - a_divider_right_index;
        // dbg!((a_divider_right_index, b_divider_right_index));

        a_divider_left = nums1[a_divider_right_index - 1];
        a_divider_right = nums1[a_divider_right_index];
        b_divider_left = nums2[b_divider_right_index - 1];
        b_divider_right = nums2[b_divider_right_index];

        // a的右半边太大了，a的分隔线左移，b的分隔线右移
        if a_divider_left > b_divider_right {
            // println!("a_divider_left({}) > b_divider_right({})\na's divider move left, b's divider move right", a_divider_left, b_divider_right);
            if a_divider_right_index <= 1 {
                // 移动后a的分隔线已经在最左边了
                return if total_len % 2 == 0 {
                    if b_divider_right_index == len_b - 1 {
                        // [4,5,6]、[1,2,3]
                        (nums1[0] + nums2[len_b - 1]) as f64 / 2_f64
                    } else {
                        // [4,5]、[1,2,3,6]
                        // 1 2 3 | 6
                        //       | 4 5
                        (nums2[b_divider_right_index]
                            + nums2[b_divider_right_index + 1].min(nums1[0]))
                            as f64
                            / 2_f64
                    }
                } else {
                    nums2[b_divider_right_index] as f64
                };
            }
            a_right = a_divider_right_index - 1;
        } else if b_divider_left > a_divider_right {
            // println!("b_divider_left({}) > a_divider_right({})\na's divider move right, b's divider move left", b_divider_left, a_divider_right);
            if a_divider_right_index == len_a - 1 {
                // 移动后a的分隔线已经在最右边了
                return if total_len % 2 == 0 {
                    if b_divider_right_index == 1 {
                        // [1,2]、[3,4]
                        (nums1[len_a - 1] + nums2[0]) as f64 / 2_f64
                    } else {
                        // [1,3]、[2,4,5,6]
                        (nums2[b_divider_right_index - 2].max(nums1[len_a - 1])
                            + nums2[b_divider_right_index - 1]) as f64
                            / 2_f64
                    }
                } else {
                    // [1,2]、[3,4,5,6,7]
                    nums2[b_divider_right_index - 2].max(nums1[len_a - 1]) as f64
                };
            }
            a_left = a_divider_right_index + 1;
        } else {
            break;
        }
    }
    if total_len % 2 == 0 {
        (a_divider_left.max(b_divider_left) + a_divider_right.min(b_divider_right)) as f64 / 2_f64
    } else {
        a_divider_left.max(b_divider_left) as f64
    }
}

/**
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

FIXME [1,2],[3,4]的测试用例在本地运行能通过，在leetcode上运行就不行
*/
fn move_divider_of_two_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (len_a, len_b) = (nums1.len(), nums2.len());
    // 保证数组a的长度更短，我们遍历较短的数组a节约时间，加上二分查找后使得时间复杂度降低到O(log(min(m,n)))
    if len_a > len_b {
        return move_divider_of_two_arrays(nums2, nums1);
    }

    // 如果较短数组的长度是0，统一了较长数组长度是奇数偶数的情况
    if len_a == 0 {
        return (nums2[(len_b - 1) / 2] as f64 + nums2[len_b / 2] as f64) / 2_f64;
    }
    // 如果较短数组的长度是1，则较短数组的分割线左边或右边会没有值，这是一种特殊的边界条件
    if len_a == 1 {
        let mut nums_b = nums2;
        let insert_index = match nums_b.binary_search(&nums1[0]) {
            Ok(index) => index,
            Err(index) => index,
        };
        nums_b.insert(insert_index, nums1[0]);
        return (nums_b[len_b / 2] as f64 + nums_b[(len_b + 1) / 2] as f64) / 2_f64;
    }

    // 往后的情况，nums1和nums2的长度至少为2
    let total_len = len_a + len_b;
    // 如果是总数是奇数，中位数左边部分会多包含一个元素
    let half_len = (total_len + 1) / 2;

    // 不能记分隔线左边元素的索引，如果分隔线在最左边，则索引会是-1导致usize溢出报错
    // 边界条件：a_divider_right_index=0时分隔线在最左边；a_divider_right_index=len_a时分隔线在最右边
    let mut a_divider_right_index;
    let mut b_divider_right_index;

    // 折半查找的左右游标
    let (mut a_left, mut a_right) = (0, len_a);
    let mut a_divider_left: i32 = 0;
    let mut a_divider_right: i32 = 0;
    let mut b_divider_left: i32 = 0;
    let mut b_divider_right: i32 = 0;
    /* 初始条件
    总数是奇数个时：[3|4], [1 2|5]
    */
    while a_left <= a_right {
        // 如果a是[1,2,3,4]: (a_divider_left, a_divider_right) = (1,2)
        a_divider_right_index = (a_left + a_right) / 2;
        // 如果a和b都用的是分隔线右边的索引的话，b_divider_right_index不需要减一
        b_divider_right_index = half_len - a_divider_right_index;
        dbg!((a_divider_right_index, b_divider_right_index));

        a_divider_left = if a_divider_right_index == 0 {
            // 如果nums1的分隔线已经到底了
            // 将a_divider_left设为i32最小值，
            // 让程序走else if b_divider_left > a_divider_right {
            // 走该分支会增加a_left的值从而跳出while循环
            i32::MIN
        } else {
            nums1[a_divider_right_index - 1]
        };
        a_divider_right = if a_divider_right_index == len_a - 1 {
            i32::MAX
        } else {
            nums1[a_divider_right_index]
        };
        b_divider_left = if b_divider_right_index == 0 {
            i32::MIN
        } else {
            nums1[b_divider_right_index - 1]
        };
        b_divider_right = if b_divider_right_index == len_b - 1 {
            i32::MAX
        } else {
            nums2[a_divider_right_index]
        };

        // a的右半边太大了，a的分隔线左移，b的分隔线右移
        if a_divider_left > b_divider_right {
            println!("a_divider_left({}) > b_divider_right({})\na's divider move left, b's divider move right", a_divider_left, b_divider_right);
            a_right = a_divider_right_index - 1;
        } else if b_divider_left > a_divider_right {
            println!("b_divider_left({}) > a_divider_right({})\na's divider move right, b's divider move left", b_divider_left, a_divider_right);
            a_left = a_divider_right_index + 1;
        } else {
            break;
        }
    }
    // 如果分隔线已经在最左边，则让b_divider_left或a_divider_left等于i32的最小值，好让它在比较左边两个元素的最大值时不会被选中
    // 如果分隔线已经在最右边，则让b_divider_right或a_divider_right等于i32的最大值，好让它在比较右边两个元素的最小值时不会被选中
    if total_len % 2 == 0 {
        (a_divider_left.max(b_divider_left) + a_divider_right.min(b_divider_right)) as f64 / 2_f64
    } else {
        a_divider_left.max(b_divider_left) as f64
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
/// FIXME 没有考虑各种边际情况，leetcode上部分测试用例不通过，没有考虑nums1或nums2为空的情况，没有考虑nums1一个元素都不取和nums2一个元素都不取的情况
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
        nums1[i].min(nums2[j]) as f64
    }
}
