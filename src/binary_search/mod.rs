/*!
TODO 二分法解决的几类问题
- [x] 有序数组的二分搜索: 34(二分搜索第一个和最后一个), 704(经典二分搜索)
- [x] 山脉数组(mountain array)类问题: 852、(941)、1095
- [x] 旋转排序数组: 33, 81, 153, 154
- [ ] 分割数组的最大值: 410
- [ ] 转变数组后最接近目标值的数组和: 1300
- [x] 平方根: 69(牛顿连续均值求平方根?)
- [ ] 寻找重复数: 287
- [x] 爱吃香蕉的珂珂: 875
*/
mod median_of_two_sorted_arrays;
mod mountain_array;
mod search_a_2d_matrix;

/// https://leetcode.com/problems/binary-search/
fn binary_search_any(nums: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len() as i32 - 1;
    while start <= end {
        let mid = start + (end - start) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Less => start = mid + 1,
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Greater => end = mid - 1,
        }
    }
    -1
}

#[test]
fn test_binary_search_any() {
    const TEST_CASES: [(&[i32], i32, i32); 3] = [(&[5], 5, 0), (&[5], -5, -1), (&[2, 5], 0, -1)];
    for (nums, target, output) in TEST_CASES {
        assert_eq!(binary_search_any(nums.to_owned(), target), output);
    }
}

/// https://leetcode.com/problems/search-insert-position/
fn binary_search_first_to_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }
    if nums[left as usize] >= target {
        left
    } else {
        left + 1
    }
}

#[test]
fn test_binary_search_first_to_insert() {
    const TEST_CASES: [(&[i32], i32, i32); 1] = [(&[1, 3, 5, 6], 2, 1)];
    for (nums, target, insert_index) in TEST_CASES {
        assert_eq!(
            binary_search_first_to_insert(nums.to_owned(), target),
            insert_index
        );
    }
}

/// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
/// Vec::binary_search() return last position of target
/// 三种二分搜索的目标: 返回任意等于target的下标(最简单,right=len,left<right), 返回第一个下标，返回最后一个下标
fn binary_search_first_and_last(nums: Vec<i32>, target: i32) -> Vec<i32> {
    /**
    Round 1:
    1 1 1
    L M R
    break:
    1 1 1
    L R
    */
    fn binary_search_first(nums: &[i32], target: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return -1;
        }
        let (mut left, mut right) = (0, n - 1);
        // 只要L和R相邻就可以跳出循环了
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            match unsafe { nums.get_unchecked(mid) }.cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                // binary_search first和last的两个区别:
                // 1. Equal分支时, search_first(right=mid), search_last(start=mid去排除较前的值为target的部分)
                // 2. 如果是search_first，跳出循环后优先判断 if nums[left]==target
                std::cmp::Ordering::Equal => right = mid,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }
        if nums[left] == target {
            return left as i32;
        }
        if nums[right] == target {
            return right as i32;
        }
        -1
    }

    /// same as binary_search() in std
    fn binary_search_last(nums: &[i32], target: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return -1;
        }
        let (mut left, mut right) = (0, n - 1);
        // 只要L和R相邻就可以跳出循环了
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            match unsafe { nums.get_unchecked(mid) }.cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Equal => left = mid,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }
        if nums[right] == target {
            return right as i32;
        }
        if nums[left] == target {
            return left as i32;
        }
        -1
    }
    let first = binary_search_first(&nums, target);
    if first == -1 {
        return vec![-1, -1];
    }
    vec![first, binary_search_last(&nums, target)]
}

#[test]
fn test_binary_search_first_and_last() {
    const TEST_CASES: [(&[i32], i32, &[i32]); 4] = [
        (&[-2, -1], 0, &[-1, -1]),
        (&[5, 7, 7, 8, 8, 10], 8, &[3, 4]),
        (&[5, 7, 7, 8, 8, 10], 6, &[-1, -1]),
        (&[], 0, &[-1, -1]),
    ];
    for (nums, target, excepted) in TEST_CASES {
        assert_eq!(
            binary_search_first_and_last(nums.to_vec(), target),
            excepted.to_vec()
        );
    }
}
