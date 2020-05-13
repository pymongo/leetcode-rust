/* 解题收获
看到时间复杂度要求O(log(n)，基本只能用「二分查找」
 */

#[cfg(feature = "unused")]
pub fn run() {
    // print!("ans = {}", find_median2(vec![1, 3], vec![2]));
    // print!("ans = {}", find_median2(vec![-2, -1], vec![3]));
    println!("ans = {}", logn(vec![1, 2, 3, 4], vec![3, 6, 8, 9]));
}

// use std::collections::BTreeSet;

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
*/
#[cfg(feature = "unused")]
fn logn(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // println!("ans = {}", logn(vec![1, 2, 3, 4], vec![3, 6, 8, 9]))
    let ans: f64;
    let (a_len, b_len) = (nums1.len(), nums2.len());
    let (mut a_mid_left, mut a_mid_right) = ((a_len / 2) - 1, a_len / 2);
    let (mut b_mid_left, mut b_mid_right) = ((b_len / 2) - 1, b_len / 2);
    // if !(nums1[a_mid_left] <= nums2[b_mid_right]
    //   && nums1[b_mid_left] <= nums2[a_mid_right]) {
    //
    // }
    loop {
        if nums1[a_mid_left] > nums2[b_mid_right] {
            // a的右半边太大了！b的分割线左移一位、a的分割线右移一位
            a_mid_left -= 1;
            a_mid_right -= 1;
            b_mid_left += 1;
            b_mid_right += 1;
        } else if nums2[b_mid_left] > nums1[a_mid_right] {
            // b的右半边太大了！b的分割线左移一位、a的分割线右移一位
            b_mid_left -= 1;
            b_mid_right -= 1;
            a_mid_left += 1;
            a_mid_right += 1;
        } else {
            // calc answer
            ans = (std::cmp::max(nums1[a_mid_left], nums2[b_mid_left])
                + std::cmp::min(nums1[a_mid_right], nums2[b_mid_right])) as f64 / 2 as f64;
            break;
        }
    }
    ans
}

// 隐含条件nums1和nums2是有序的
#[cfg(feature = "unused")]
fn find_median2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
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
        ans = (arr[stop_index] + arr[stop_index - 1]) as f64 / 2 as f64;
    } else {
        ans = arr[stop_index] as f64;
    }
    ans
}


// 隐含条件nums1和nums2是有序的
// 弊端：TreeSet会去重，需要手写排序算法
// 2064 / 2085 test cases passed
#[cfg(feature = "unused")]
fn find_median(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    use std::collections::BTreeSet;
    let mut ans: f64 = 0 as f64;
    let mut sorted_set: BTreeSet<i32> = BTreeSet::new();
    let (mut nums1, mut nums2) = (nums1, nums2);
    nums1.append(&mut nums2);
    for num in nums1 {
        sorted_set.insert(num);
    }
    let len: usize = sorted_set.len();

    let mut index: usize = 0;

    if len % 2 == 0 {
        let mut middle_left = 0;
        let mut middle_right = 0;
        for each in sorted_set {
            if index == (len / 2 - 1) {
                middle_left = each;
            } else if index == len / 2 {
                middle_right = each;
                break;
            }
            index += 1;
        }
        ans = (middle_left + middle_right) as f64 / 2 as f64
    } else {
        for each in sorted_set {
            if index == len / 2 {
                ans = each as f64;
                break;
            }
            index += 1;
        }
    }
    ans
}

/* 美服第一 二分查找的算法
use std::cmp::Ordering;
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

impl Solution {
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
}
*/

/* 国服第一
impl Solution {
fn get_min(x :usize, y :usize) ->usize{
    if x > y {
        return  y;
    }else {
        return  x;
    }

}


fn getKth(nums1: &Vec<i32>, start1 :usize, end1 :usize, nums2: &Vec<i32>, start2 :usize, end2 :usize, k :usize) -> i32{

    let len1 = end1 - start1 + 1;
    let len2 = end2 - start2 + 1;
    if len1 > len2 {
        return  Solution::getKth(nums2, start2, end2, nums1, start1, end1, k);
    }

    if len1 == 0 {
        return nums2[start2 + k -1];
    }

    if k == 1 {
        let mut temp1 = nums1[start1];
        let mut  temp2 = nums2[start2];
        if temp1 > temp2 {
            return  temp2;
        }else{
            return  temp1;
        }
    }

    let i = start1 + Solution::get_min(len1, k/2) -1;
    let j = start2 + Solution::get_min(len2, k/2) -1;
    if nums1[i] > nums2[j]{
        return Solution::getKth(nums1, start1, end1, nums2, j + 1, end2, k - (j - start2 +1));
    }else {
        return Solution::getKth(nums1, i + 1, end1, nums2, start2, end2, k -(i - start1 +1 ));
    }

}


pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

let n  = nums1.len();
let m  = nums2.len();

let left  = (n + m + 1)/2;
let right  = (n + m +2)/2;

let temp :f64 = 0.5;

return (Solution::getKth(&nums1, 0, n-1, &nums2, 0, m -1, left)  + Solution::getKth(&nums1, 0, n -1, &nums2, 0, m -1, right)) as f64 * temp;


}

}
*/