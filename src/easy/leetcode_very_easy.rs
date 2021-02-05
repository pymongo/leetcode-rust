/*! leetcode超简单题，不看答案20分钟内搞定，例如每次周赛第一题(送分题)
## 简短一行Rust代码能解决的题:
- [剑指Offer 17. 打印从1到最大n位的十进制数]: (1..10i32.pow(n as u32)).collect()
*/

/// https://leetcode.com/problems/to-lower-case/
fn to_lower_case(s: String) -> String {
    s.to_ascii_lowercase()
}

/// https://leetcode.com/problems/defanging-an-ip-address/
fn defanging_an_ip_address(address: String) -> String {
    address.replace(".", "[.]")
}

/// https://leetcode.com/problems/valid-number/
/// 这题正确的解法应该是DFA(有限状态机)，手写的状态机应该会比标准库的f32解析状态机性能更好
fn is_number(s: String) -> bool {
    s.trim().parse::<f32>().is_ok()
}

/// https://leetcode.com/problems/running-sum-of-1d-array/
fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
}

/// https://leetcode.com/problems/shuffle-the-array/
fn shuffle_the_array(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n: usize = n as usize;
    let mut ret: Vec<i32> = Vec::with_capacity(2 * n);
    for i in 0..n {
        ret.push(nums[i]);
        ret.push(nums[i + n]);
    }
    ret
}

/// https://leetcode.com/problems/count-of-matches-in-tournament/
fn number_of_matches(mut n: i32) -> i32 {
    let mut ret = 0;
    while n != 1 {
        let matches = n / 2;
        ret += matches;
        n -= matches;
    }
    ret
}

/// https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/
fn generate_the_string(n: i32) -> String {
    if n % 2 == 1 {
        "a".repeat(n as usize)
    } else {
        let mut ret = "a".repeat(n as usize - 1);
        ret.push('b');
        ret
    }
}

/// https://leetcode.com/problems/robot-return-to-origin/
fn judge_circle(moves: String) -> bool {
    let mut up_and_down = 0i32;
    let mut left_and_right = 0i32;
    moves.into_bytes().into_iter().for_each(|byte| match byte {
        b'U' => up_and_down += 1,
        b'D' => up_and_down -= 1,
        b'L' => left_and_right += 1,
        b'R' => left_and_right -= 1,
        _ => unreachable!(),
    });
    up_and_down == 0 && left_and_right == 0
}

/// https://leetcode.com/problems/fizz-buzz/
fn fizz_buzz(n: i32) -> Vec<String> {
    let mut ret = Vec::new();
    for i in 1..=n {
        if i % 3 == 0 {
            if i % 5 == 0 {
                ret.push("FizzBuzz".to_string());
            } else {
                ret.push("Fizz".to_string());
            }
        } else if i % 5 == 0 {
            ret.push("Buzz".to_string());
        } else {
            ret.push(i.to_string());
        }
    }
    ret
}

/// https://leetcode.com/problems/minimum-time-visiting-all-points/
fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut ret = 0;
    for i in 1..n {
        let dx = (points[i][0] - points[i - 1][0]).abs();
        let dy = (points[i][1] - points[i - 1][1]).abs();
        ret += dx.max(dy);
    }
    ret
}

/// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    // points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    // let mut last_x = points[0][0];
    // for point in points.into_iter().skip(1) {
    //     max_width = max_width.max(point[0] - last_x);
    //     last_x = point[0];
    // }
    let mut points_x: Vec<i32> = points.into_iter().map(|v| v[0]).collect();
    points_x.sort_unstable();
    points_x
        .windows(2)
        .map(|a| a[1] - a[0])
        .max()
        .unwrap_or_default()
}

/// https://leetcode.com/problems/self-dividing-numbers/
fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut ret = Vec::new();
    'outer: for num in left..=right {
        let mut n = num;
        while n != 0 {
            match num.checked_rem(n % 10) {
                Some(remainder) => {
                    // 如果不能被组成该数字的其中一位整数，则不是自除数
                    if remainder != 0 {
                        continue 'outer;
                    }
                }
                // 取余数%操作符的rhs是0时，则checked_rem会得到None，避免: panicked at 'attempt to calculate the remainder with a divisor of zero'
                None => continue 'outer,
            }
            n /= 10;
        }
        ret.push(num);
    }
    ret
}

#[test]
fn test_self_dividing_numbers() {
    assert_eq!(
        self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
}

/// https://leetcode.com/problems/max-increase-to-keep-city-skyline/
/// 大意: 先算出旧矩阵每行每列的最大值，然后遍历矩阵看看当前值最大能加到什么，然后累加最大能增加的量
#[allow(clippy::needless_range_loop)]
fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut max_row: Vec<i32> = Vec::with_capacity(m);
    let mut max_col: Vec<i32> = vec![std::i32::MIN; n];
    for row in grid.iter() {
        max_row.push(*row.iter().max().unwrap());
    }
    for j in 0..n {
        for i in 0..m {
            max_col[j] = max_col[j].max(grid[i][j]);
        }
    }

    let mut ret = 0;
    for i in 0..m {
        let curr_max_row = max_row[i];
        for j in 0..n {
            // 最大能增长的高度等于行列最大值二者的最小值
            ret += std::cmp::min(curr_max_row, max_col[j]) - grid[i][j];
        }
    }
    ret
}

/// https://leetcode.com/problems/design-parking-system/
struct ParkingSystem {
    big_slot_cap: u16,
    medium_slot_cap: u16,
    small_slot_cap: u16,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            big_slot_cap: big as u16,
            small_slot_cap: small as u16,
            medium_slot_cap: medium as u16,
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        #[inline(always)]
        fn helper(slot: &mut u16) -> bool {
            if *slot == 0 {
                false
            } else {
                *slot -= 1;
                true
            }
        }
        match car_type {
            1 => helper(&mut self.big_slot_cap),
            2 => helper(&mut self.medium_slot_cap),
            3 => helper(&mut self.small_slot_cap),
            _ => false,
        }
    }
}

/// https://leetcode.com/problems/k-closest-points-to-origin/
/// 这题的正统解法应该是quick_select_min过程重复k次
fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    points
        .sort_unstable_by_key(|x| unsafe { x.get_unchecked(0).pow(2) + x.get_unchecked(1).pow(2) });
    points.truncate(k as usize);
    points
}

/// https://leetcode.com/problems/find-k-closest-elements/
/// 这题的正统解法应该是二分法，因为输入数组是有序的
fn find_closest_elements(mut arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    arr.sort_unstable_by(|a, b| (a - x).abs().cmp(&(b - x).abs()).then(a.cmp(b)));
    arr.truncate(k as usize);
    // 找到最接近原点的k的点后，再次排序，确保输出能有序
    arr.sort_unstable();
    arr
}

/// https://leetcode.com/problems/height-checker/
/// 同学们按身高升序排列，统计未站在正确位置的学生数
fn height_checker(heights: Vec<i32>) -> i32 {
    let mut correct = heights.clone();
    correct.sort_unstable();
    heights
        .into_iter()
        .zip(correct.into_iter())
        .filter(|(a, b)| a != b)
        .count() as i32
}

/** https://leetcode.com/problems/count-binary-substrings/
要数0和1数量相等的子串，也就是统计0和1分隔位置两侧0和1个数的最小值
```text
L: last_count, R: current count

1. last,curr=0,1    ans=0+0
  |1|00|111|
     ^
 L|R|
2. last,curr=1,2    ans=0+1
  |1|00|111|
        ^
   L| R|
3. last,curr=2,3    ans=1+2(out of loop)
  |1|00|111|
            ^
      L|  R|
```
*/
fn count_binary_substrings(s: String) -> i32 {
    let s = s.into_bytes();
    let mut ret = 0;
    // curr_count和last_count表示连续1或连续0的长度
    let (mut curr_count, mut last_count) = (1, 0);
    let mut last_byte = s[0];
    for byte in s.into_iter().skip(1) {
        if byte == last_byte {
            curr_count += 1;
        } else {
            ret += last_count.min(curr_count);
            last_count = curr_count;
            curr_count = 1;
        }
        last_byte = byte;
    }
    ret += last_count.min(curr_count);
    ret
}

/// https://leetcode.com/problems/max-consecutive-ones/
/// 这题跟count_binary_substring有点像，也是全为0或1的数组
fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let (mut cur_ones_len, mut max_ones_len) = (0, 0);
    for num in nums.into_iter() {
        if num == 1 {
            cur_ones_len += 1;
        } else {
            max_ones_len = max_ones_len.max(cur_ones_len);
            cur_ones_len = 0;
        }
    }
    max_ones_len.max(cur_ones_len)
}

#[test]
fn test_find_max_consecutive_ones() {
    const TEST_CASES: [(&[i32], i32); 3] = [(&[1], 1), (&[1, 0, 1, 1, 0, 1], 2), (&[0], 0)];
    for &(nums, expected) in TEST_CASES.iter() {
        assert_eq!(find_max_consecutive_ones(nums.to_vec()), expected);
    }
}

/// https://leetcode.com/problems/plus-one/
fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for digit in digits.iter_mut().rev() {
        if *digit == 9 {
            *digit = 0;
        } else {
            *digit += 1;
            // 如果不需要进位，则提前return
            return digits;
        }
    }
    // 跳出循环时必定是`if *digit == 9`(需要进位)的分支
    digits.insert(0, 1);
    digits
}

/// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
fn maximum_product_of_two_elements_in_an_array(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by_key(|&num| std::cmp::Reverse(num));
    (nums[0] - 1) * (nums[1] - 1)
}

/// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/
fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    start_time
        .into_iter()
        .zip(end_time.into_iter())
        .filter(|&(start, end)| start <= query_time && query_time <= end)
        .count() as i32
}

#[test]
fn test_busy_student() {
    assert_eq!(busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
}

/// https://leetcode.com/problems/transpose-matrix/
/// return [list(i) for i in zip(*a)]
#[allow(clippy::needless_range_loop)]
fn transpose_matrix(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (a.len(), a[0].len());
    let mut ret = Vec::with_capacity(n);
    for j in 0..n {
        let mut row = Vec::with_capacity(m);
        for i in 0..m {
            row.push(a[i][j]);
        }
        ret.push(row);
    }
    ret
}

/// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
fn max_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by_key(|&num| std::cmp::Reverse(num));
    (nums[0] - 1) * (nums[1] - 1)
}

/// https://leetcode.com/problems/count-and-say/
fn count_and_say(n: i32) -> String {
    let mut last = vec![b'1'];
    for _ in 1..n {
        let last_len = last.len();
        let mut curr: Vec<u8> = Vec::new();
        let mut same_byte_first_index = 0;
        for i in 1..last_len {
            if last[same_byte_first_index] != last[i] {
                curr.push(b'0' + (i - same_byte_first_index) as u8);
                curr.push(last[same_byte_first_index]);
                same_byte_first_index = i;
            }
        }
        // 防止从 "1" -> "11"的递推过程没有计数
        curr.push(b'0' + (last_len - same_byte_first_index) as u8);
        curr.push(last[same_byte_first_index]);
        last = curr;
    }
    unsafe { String::from_utf8_unchecked(last) }
}

#[test]
fn test_count_and_say() {
    const TEST_CASES: [(i32, &str); 4] = [(1, "1"), (2, "11"), (3, "21"), (4, "1211")];
    for &(n, expected) in TEST_CASES.iter() {
        assert_eq!(count_and_say(n), expected.to_string());
    }
}

/// https://leetcode.com/problems/queue-reconstruction-by-height/
fn reconstruct_queue(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // 按身高h倒序排列再按k(前面有几个人身高大于等于当前people)升序
    a.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
    // 预先创建好全为空slot的返回值数组，采用「插空」的方法，依次给每一个人在当前的队列中选择一个插入的位置，
    // 因为每个人前面有几个比他大的人数是确定的，而且身高逆序排列后能优先将大个子安排在index较前的slot
    // 用insert的原因是，后面遇到小个子，例如(7,0)之后是(5,0)，7已经占据了下标0，所以小个子就(insert(0))往后挪一格占据下标1
    let mut ret = Vec::with_capacity(a.len());
    for people in a.into_iter() {
        let slot_index = people[1] as usize;
        ret.insert(slot_index, people);
    }
    ret
}

#[test]
fn test_reconstruct_queue() {
    let test_cases = vec![(
        vec_vec![[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]],
        vec_vec![[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]],
    )];
    for (input, output) in test_cases.into_iter() {
        assert_eq!(reconstruct_queue(input), output);
    }
}

/// https://leetcode.com/problems/matrix-cells-in-distance-order/
fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
    let max_dist = r0.max(r - 1 - r0) + c0.max(c - 1 - c0);
    // 桶排序
    let mut bucket = vec![Vec::new(); max_dist as usize + 1];
    for i in 0..r {
        for j in 0..c {
            bucket[((r0 - i).abs() + (c0 - j).abs()) as usize].push(vec![i, j]);
        }
    }

    // 由于距离相等时，顺序点的坐标无关，所以可以将排序好的桶组合接起来
    let mut ret = Vec::new();
    for each in bucket.into_iter() {
        ret.extend(each);
    }
    ret
}

/// https://leetcode.com/problems/xor-operation-in-an-array/
fn xor_operation(n: i32, start: i32) -> i32 {
    (start..).step_by(2).take(n as usize).fold(0, |a, b| a ^ b)
}

/// https://leetcode.com/problems/create-target-array-in-the-given-order/
fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ret = Vec::with_capacity(n);
    for i in 0..n {
        ret.insert(index[i] as usize, nums[i]);
    }
    ret
}

/// https://leetcode.com/problems/decompress-run-length-encoded-list/
fn decompress_run_length_encoded_list(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();
    for i in (0..nums.len()).step_by(2) {
        ret.extend(vec![nums[i + 1]].repeat(nums[i] as usize));
    }
    ret
}

/// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
/// 尽管题目要求逆序(左往右)累加累乘每位，但是由于加法和乘法的各项可以互换，所以我右往左遍历每位也是可以的
fn subtract_product_and_sum(mut n: i32) -> i32 {
    let (mut sum, mut product) = (0, 1);
    while n != 0 {
        let digit = n % 10;
        sum += digit;
        product *= digit;
        n /= 10;
    }
    product - sum
}

/// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    let mut ret = Vec::with_capacity(candies.len());
    for candy in candies.into_iter() {
        ret.push(candy + extra_candies >= max);
    }
    ret
}

/// https://leetcode.com/problems/range-sum-query-immutable/
struct RangeSumOffline {
    prefix_sum: Vec<i32>,
}

impl RangeSumOffline {
    fn new(mut nums: Vec<i32>) -> Self {
        // nums.iter().scan(0, |acc, n| { *acc += n; Some(*acc) }).collect()
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        Self { prefix_sum: nums }
    }

    /// 另一种前缀和的表示方法是，arr[i]表示数组前i项的和，arr[0]=0，求解答案的表达式是arr[j+1]-arr[i]
    fn sum_range(&self, i: i32, j: i32) -> i32 {
        unsafe {
            self.prefix_sum.get_unchecked(j as usize)
                - self.prefix_sum.get((i - 1) as usize).unwrap_or(&0)
        }
    }
}

#[test]
fn test_range_sum_offline() {
    let arr = RangeSumOffline::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(arr.sum_range(0, 2), 1);
    assert_eq!(arr.sum_range(2, 5), -1);
    assert_eq!(arr.sum_range(0, 5), -3);
}

/// https://leetcode.com/problems/maximum-gap/
fn maximum_gap(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    // nums.windows(2).fold(0, |s, x| s.max(x[1] - x[0]))
    let mut ret = 0; // all num is positive
    for i in 1..nums.len() {
        ret = ret.max(nums[i] - nums[i - 1]);
    }
    ret
}

/// https://leetcode.com/problems/contains-duplicate/
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = std::collections::HashSet::new();
    for num in nums.into_iter() {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }
    false
}

/// https://leetcode.com/problems/contains-duplicate-2/
/// 一个长度为k的窗口内，是否存在重复元素
#[allow(clippy::needless_range_loop)]
fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    if k == 0 {
        return false;
    }
    let n = nums.len();
    let k = if k > n as i32 { n } else { k as usize };
    let mut set = std::collections::HashSet::with_capacity(k);
    for i in 0..k {
        if set.contains(&nums[i]) {
            return true;
        }
        set.insert(nums[i]);
    }
    for i in k..n {
        if set.contains(&nums[i]) {
            return true;
        }
        set.remove(&nums[i - k]);
        set.insert(nums[i]);
    }
    false
}

/// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    if n == 1 {
        return vec![-1];
    }
    let mut max = arr[n - 1];
    arr[n - 1] = -1;
    for i in (0..=(n - 2)).rev() {
        let temp = arr[i];
        arr[i] = max;
        max = max.max(temp);
    }
    arr
}

#[test]
fn test_replace_elements() {
    assert_eq!(
        replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
}

/// https://leetcode.com/problems/richest-customer-wealth/
fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .into_iter()
        .map(|row| row.into_iter().sum::<i32>())
        .max()
        .unwrap_or_default()
}

/** https://leetcode.com/problems/merge-sorted-array/
## 从后往前遍历的解题思路
参考一道面试题，如何将占据内存地址[0:10]的数组复制到内存地址[5:15]上
首先顺序复制的话，复制到第6个时会把第1个给覆盖掉
如果使用倒序复制的方法，新旧数组的指针都从后往前遍历，那就能避免重复
这道题数组nums1的后半部分预留了全是0的存储空间，所以从后往前遍历时既能更新nums1又不用担心nums1出现重叠导致覆盖的问题
*/
fn merge_two_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
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
fn test_merge_two_sorted_array() {
    const TEST_CASES: [(&[i32], i32, &[i32], i32, &[i32]); 2] = [
        (&[1, 2, 3, 0, 0, 0], 3, &[2, 5, 6], 3, &[1, 2, 2, 3, 5, 6]),
        (&[2, 0], 1, &[1], 1, &[1, 2]),
    ];
    for &(nums1, m, nums2, n, expected) in TEST_CASES.iter() {
        let mut nums1 = nums1.to_vec();
        merge_two_sorted_array(&mut nums1, m, &mut nums2.to_vec(), n);
        assert_eq!(nums1, expected.to_vec());
    }
}

/// https://leetcode.com/problems/first-bad-version/
struct FirstBadVersion(i32);

impl FirstBadVersion {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, versions: i32) -> bool {
        versions >= self.0
    }

    fn first_bad_version(&self, n: i32) -> i32 {
        let (mut start, mut end) = (0, n);
        while start < end {
            let mid = start + (end - start) / 2;
            if self.isBadVersion(mid) {
                // 如果出错了，不能排除掉mid，错误可能在[mid,end]
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        start
    }
}

#[test]
fn test_first_bad_version() {
    const TEST_CASES: [(i32, i32); 1] = [(4, 5)];
    for &(bad, len) in TEST_CASES.iter() {
        let temp = FirstBadVersion(bad);
        assert_eq!(temp.first_bad_version(len), bad);
    }
}

/// https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/
fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
    // 注意题目的入参是n行m列，而非正常的m行n列
    let (m, n) = (n as usize, m as usize);
    // 一开始的矩阵，都是0，所以都不是偶数(false)
    let mut mat = vec![vec![false; n]; m];
    for indice in indices.into_iter() {
        let (row, col) = (indice[0] as usize, indice[1] as usize);
        for row in mat.iter_mut().take(m) {
            row[col] = !row[col];
        }
        for each in mat[row].iter_mut().take(n) {
            *each = !*each;
        }
    }
    mat.into_iter()
        .map(|row| row.into_iter().filter(|&each| each).count())
        .sum::<usize>() as i32
}

/// https://leetcode.com/problems/flipping-an-image/
fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for row in a.iter_mut() {
        row.reverse();
        row.iter_mut().for_each(|val| *val = 1 - *val);
    }
    a
}

/// https://leetcode.com/problems/jewels-and-stones/
fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let jewels: std::collections::HashSet<u8> = j.into_bytes().into_iter().collect(); // HashSet::from_iter
    let mut ret = 0;
    for stone in s.into_bytes().into_iter() {
        if jewels.contains(&stone) {
            ret += 1;
        }
    }
    ret
}

/// https://leetcode.com/problems/minimum-deletion-cost-to-avoid-repeating-letters/
/// 花最小代价让字符串相邻两个元素不重复，所以遇到连续的重复字符，例如连续5个a，则需要删掉4个a，留下cost数组中耗费最大的那个a
fn min_cost_to_avoid_repeating_chars(s: String, cost: Vec<i32>) -> i32 {
    let s = s.into_bytes();
    let n = s.len();
    let mut i = 0;
    let mut ret = 0;
    while i < n {
        let byte = s[i];
        let mut max_cost_of_same_byte = 0;
        let mut cost_sum = 0;
        // 找到连续的一片重复字母
        while i < n && s[i] == byte {
            max_cost_of_same_byte = max_cost_of_same_byte.max(cost[i]);
            cost_sum += cost[i];
            i += 1;
        }
        ret += cost_sum - max_cost_of_same_byte;
    }
    ret
}

#[test]
fn test_minimum_deletion_cost_to_avoid_repeating_letters() {
    assert_eq!(
        min_cost_to_avoid_repeating_chars("abaac".into(), vec![1, 2, 3, 4, 5]),
        3
    );
}

/// https://leetcode.com/problems/minimum-operations-to-make-array-equal/
fn min_operations(n: i32) -> i32 {
    (1..)
        .step_by(2)
        .take(n as usize / 2)
        .map(|each| n - (each + 1))
        .sum()
    // return n * n /4;
}

/// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
/// 任意重排数组，能否形成等差数列
fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let difference = arr[1] - arr[0];
    for i in 2..arr.len() {
        if arr[i] - arr[i - 1] != difference {
            return false;
        }
    }
    true
}

/// https://leetcode.com/problems/destination-city/
/// 找出无环图中的终点(出度为0的点)
/// 还有种解法是把所有起点做成HashSet再遍历找到not contains的终点，则为第一个出度为0的点
fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut outdegree = std::collections::HashMap::<String, u8>::with_capacity(paths.len());
    for path in paths.into_iter() {
        let mut it = path.into_iter().take(2);
        *outdegree.entry(it.next().unwrap()).or_default() += 1;
        outdegree.entry(it.next().unwrap()).or_default();
    }
    for (city, outdegree) in outdegree.into_iter() {
        if outdegree == 0 {
            return city;
        }
    }
    unreachable!()
}

/// https://leetcode.com/problems/assign-cookies/
/// children表示每个child的所需摄入的热量，cookie表示每个饼干的热量，贪心思路是排序后尽量让小的饼干满足小胃口的孩子
fn assign_cookies(mut children: Vec<i32>, mut cookies: Vec<i32>) -> i32 {
    children.sort_unstable();
    cookies.sort_unstable();
    // while i < len_g && j < len_s {
    //     if cookies[j] >= children[i] {
    //         i += 1;
    //         j += 1;
    //     } else {
    //         j += 1;
    //     }
    // }
    let mut cookies = cookies.into_iter();
    let mut ret = 0;
    for child in children.into_iter() {
        while let Some(cookie) = cookies.next() {
            if cookie >= child {
                ret += 1;
                break;
            }
        }
    }
    ret
}

#[test]
fn test_find_content_children() {
    const TEST_CASES: [(&[i32], &[i32], i32); 2] = [
        // 两个面值为1的糖果🍬只能满足第一个孩子(胃口为1)，因为每个孩子最多吃一个糖果
        (&[1, 2, 3], &[1, 1], 1),
        (&[1, 2], &[1, 2, 3], 2),
    ];
    // for &(input, output) in
}

/// https://leetcode.com/contest/weekly-contest-222/problems/maximum-units-on-a-truck/
/// https://leetcode.com/problems/maximum-units-on-a-truck/
/// 有点像背包问题，因为所有物体的容积都是1，所以这题应该也能用贪心去解题，尽量先放价值更高的物件
fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
    box_types.sort_unstable_by_key(|box_type| -box_type[1]);
    let mut ret = 0;
    for box_type in box_types.into_iter() {
        // 这里类似于Go语言解构数组的写法: const [size, unit] = boxTypes[i];
        // refutable pattern: let [quantity, unit_price, ..] = box_type[..]; 意思是这种写法是可辩驳的(refutable)，要写成if let或match
        let (quantity, unit_price) = (box_type[0], box_type[1]);
        if quantity <= truck_size {
            ret += quantity * unit_price;
            truck_size -= quantity;
        } else {
            ret += truck_size * unit_price;
            break;
        }
    }
    ret
}

#[test]
fn test_maximum_units() {
    let test_cases = vec![(vec_vec![[1, 3], [2, 2], [3, 1]], 4, 8)];
    for (box_types, truck_size, max_value) in test_cases.into_iter() {
        assert_eq!(maximum_units(box_types, truck_size), max_value);
    }
}

/// https://leetcode.com/problems/positions-of-large-groups/
fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let s = s.into_bytes();
    let (mut i, n) = (0, s.len());
    let mut ret = vec![];
    while i < n {
        let start = i;
        while i < n && s[i] == s[start] {
            i += 1;
        }
        if i - start >= 3 {
            ret.push(vec![start as i32, i as i32 - 1]);
        }
    }
    ret
}

/// https://leetcode.com/problems/lemonade-change/
fn lemonade_change(bills: Vec<i32>) -> bool {
    // 面值为20的纸币是最大的，基本没用，不能用于找零
    let (mut currency_5, mut currency_10) = (0u16, 0u16);
    for bill in bills {
        match bill {
            // 多一张面值为5的纸币
            5 => currency_5 += 1,
            10 => {
                if currency_5 == 0 {
                    // 不能找零5元
                    return false;
                }
                currency_5 -= 1;
                currency_10 += 1;
            }
            // 难点在这，找零10+5还是找零5+5+5呢?由于面值为5的泛用性更强，能给10找零，所以贪心一点优先找零10的
            // 因为用5美元找零的场景比用10美元的多，所以优先消耗
            20 => {
                if currency_10 > 0 && currency_5 > 0 {
                    currency_10 -= 1;
                    currency_5 -= 1;
                } else if currency_5 >= 3 {
                    currency_5 -= 3;
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    true
}

/// https://leetcode.com/problems/reformat-phone-number/
/// https://leetcode.com/contest/weekly-contest-220/problems/reformat-phone-number/
fn reformat_phone_number(number: String) -> String {
    let mut s: Vec<u8> = number
        .into_bytes()
        .into_iter()
        .filter(|x| x.is_ascii_digit())
        .collect();
    let len = s.len();
    let mut n_3_pairs = len / 3;
    let rem_3 = len % 3;
    let mut n_2_pairs = 0;
    if rem_3 == 1 {
        n_3_pairs -= 1;
        n_2_pairs += 2;
    } else if rem_3 == 2 {
        n_2_pairs += 1;
    }

    let mut insert = 0;
    for _ in 0..n_3_pairs {
        insert += 3;
        s.insert(insert, b'-');
        insert += 1;
    }
    for _ in 0..n_2_pairs {
        insert += 2;
        s.insert(insert, b'-');
        insert += 1;
    }
    // 去掉末尾的'-'
    s.pop();
    // println!("{:?}", s.clone().into_iter().map(|x| x as char).collect::<Vec<char>>());
    unsafe { String::from_utf8_unchecked(s) }
}

#[test]
fn test_reformat_phone_number() {
    const TEST_CASES: [(&str, &str); 2] = [
        ("--17-5 229 35-39475 ", "175-229-353-94-75"),
        ("1-23-45 6", "123-456"),
    ];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(reformat_phone_number(input.to_string()), output.to_string());
    }
}

/// https://leetcode.com/problems/count-number-of-teams/
#[allow(clippy::needless_range_loop)]
#[allow(clippy::comparison_chain)]
fn num_teams(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ret = 0;
    for mid in 1..n - 1 {
        let mid_num = nums[mid];
        let (mut left_smaller, mut left_bigger) = (0, 0);
        for i in 0..mid {
            if nums[i] < mid_num {
                left_smaller += 1;
            } else if nums[i] > mid_num {
                left_bigger += 1;
            }
        }
        let (mut right_smaller, mut right_bigger) = (0, 0);
        for i in mid + 1..n {
            if nums[i] < mid_num {
                right_smaller += 1;
            } else if nums[i] > mid_num {
                right_bigger += 1;
            }
        }
        ret += left_smaller * right_bigger + left_bigger * right_smaller;
    }
    ret
    /*
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if (a[i] < a[j] && a[j] < a[k]) || (a[i] > a[j] && a[j] > a[k]) {
                    ret += 1;
                }
            }
        }
    }
    */
}

/// '#'表示退格操作，请你比较两个含退格操作符的字符串是否相等
fn backspace_compare(s: String, t: String) -> bool {
    fn parse(s: String) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::new();
        for byte in s.into_bytes() {
            if byte == b'#' {
                let _ = ret.pop();
            } else {
                ret.push(byte);
            }
        }
        ret
    }
    parse(s) == parse(t)
}

#[test]
fn test_backspace_compare() {
    const TEST_CASES: [(&str, &str, bool); 4] = [
        ("ab#c", "ad#c", true),
        ("ab##", "c#d#", true),
        ("a##c", "#a#c", true),
        ("a#c", "b", false),
    ];
    for &(s, t, expected) in &TEST_CASES {
        assert_eq!(backspace_compare(s.to_string(), t.to_string()), expected);
    }
}

/// https://leetcode.com/problems/special-positions-in-a-binary-matrix/
/// 数一数矩阵上总共有几个值为1的位置满足横竖两个方向上仅有它一个为1，其余为0
#[allow(clippy::needless_range_loop)]
fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (mat.len(), mat[0].len());
    let mut ret = 0;
    for i in 0..m {
        'for_j: for j in 0..n {
            if mat[i][j] == 1 {
                for row in 0..m {
                    if row == i {
                        continue; // for row in 0..m
                    }
                    if mat[row][j] == 1 {
                        continue 'for_j;
                    }
                }
                for col in 0..n {
                    if col == j {
                        continue;
                    }
                    if mat[i][col] == 1 {
                        continue 'for_j;
                    }
                }
                ret += 1;
            }
        }
    }
    ret
}

#[test]
fn test_num_special() {
    let test_cases = vec![(
        vec_vec![[0, 0, 0, 1], [1, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        2,
    )];
    for (points, min_cost) in test_cases {
        assert_eq!(num_special(points), min_cost);
    }
}

/// https://leetcode.com/problems/lucky-numbers-in-a-matrix/
fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut ret = Vec::with_capacity(m.max(n));
    'outer: for i in 0..m {
        // 找到当前行的最小值及其列坐标
        let mut row_min_val = matrix[i][0];
        let mut row_min_idx = 0;
        for j in 0..n {
            if matrix[i][j] < row_min_val {
                row_min_val = matrix[i][j];
                row_min_idx = j;
            }
        }
        // 判断这个行最小值是不是所在列的最大值
        for row in matrix.iter().take(m) {
            if row[row_min_idx] > row_min_val {
                continue 'outer;
            }
        }
        ret.push(row_min_val);
    }
    ret
}

#[test]
fn test_lucky_numbers() {
    assert_eq!(
        lucky_numbers(vec_vec![[3, 7, 8], [9, 11, 13], [15, 16, 17]]),
        vec![15]
    );
}

/// https://leetcode.com/problems/check-if-n-and-its-double-exist/
fn check_if_n_and_its_double_exist(nums: Vec<i32>) -> bool {
    let mut set = std::collections::HashSet::new();
    for num in nums.into_iter() {
        if set.contains(&num) {
            return true;
        } else {
            // if set.contains(&(2 * n)) || (n % 2 == 0 && set.contains(&(n / 2))) {
            //     return true;
            // }
            // set.insert(n);
            if num % 2 == 0 {
                set.insert(num / 2);
            }
            set.insert(2 * num);
        }
    }
    false
}

#[test]
fn test_check_if_n_and_its_double_exist() {
    const TEST_CASES: [(&[i32], bool); 3] = [
        (&[-2, 0, 10, -19, 4, 6, -8], false),
        (&[-10, 12, -20, -8, 15], true),
        (&[7, 1, 14, 11], true), // 14=2*7
    ];
    for &(input, output) in &TEST_CASES {
        assert_eq!(check_if_n_and_its_double_exist(input.into()), output);
    }
}

/// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/
fn repeated_n_times(a: Vec<i32>) -> i32 {
    // let n = a.len() as u16 / 2;
    // let mut counter = std::collections::HashMap::new();
    // for num in a.into_iter() {
    //     *counter.entry(num).or_insert(0u16) += 1;
    // }
    //
    // for (num, count) in counter.into_iter() {
    //     if count == n {
    //         return num
    //     }
    // }
    // unreachable!()

    // 由于其他元素只出现了一次，所以不需要counter也行
    let mut set = std::collections::HashSet::new();
    for num in a.into_iter() {
        if !set.insert(num) {
            return num;
        }
    }
    unreachable!()
}

/// https://leetcode.com/problems/defuse-the-bomb/
fn defuse_the_bomb(mut code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    if k == 0 {
        return vec![0; n];
    }
    let is_negative = k < 0;
    let k = if is_negative {
        code.reverse();
        -k as usize
    } else {
        k as usize
    };
    let mut ret = Vec::with_capacity(n);

    for i in 0..n {
        let mut sum = 0;
        for j in 1..=k {
            // rotate-array循环数组遍历除了mod，还可以整个数组往右复制一份
            sum += code[(i + j) % n];
        }
        ret.push(sum);
    }
    if is_negative {
        ret.reverse();
    }

    ret
}

#[test]
fn test_defuse_the_bomb() {
    const TEST_CASES: [(&[i32], i32, &[i32]); 2] = [
        (&[5, 7, 1, 4], 3, &[12, 10, 16, 13]),
        (&[2, 4, 9, 3], -2, &[12, 5, 6, 13]),
        // 3 9 4 2
    ];
    for &(code, k, output) in TEST_CASES.iter() {
        assert_eq!(defuse_the_bomb(code.to_vec(), k), output);
    }
}

/// https://leetcode.com/problems/summary-ranges/
fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return Vec::with_capacity(0);
    }
    let mut range_start = 0;
    let mut range_end = 0;

    let mut ret = Vec::new();
    for i in 1..nums.len() {
        if nums[i] - nums[range_end] == 1 {
            range_end = i;
        } else {
            if range_start == range_end {
                ret.push(nums[range_end].to_string());
            } else {
                ret.push(format!("{}->{}", nums[range_start], nums[range_end]));
            }
            range_start = i;
            range_end = i;
        }
    }
    if range_start == range_end {
        ret.push(nums[range_end].to_string());
    } else {
        ret.push(format!("{}->{}", nums[range_start], nums[range_end]));
    }
    ret
}

#[test]
fn test_summary_ranges() {
    const TEST_CASES: [(&[i32], &[&str]); 2] =
        [(&[0, 1, 2, 4, 5, 7], &["0->2", "4->5", "7"]), (&[], &[])];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(summary_ranges(input.into()), output);
    }
}

/// https://leetcode.com/problems/binary-prefix-divisible-by-5/
fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
    let mut num = 0;
    let mut ret = Vec::with_capacity(a.len());
    for bit in a.into_iter() {
        // 由于是否能被5整除只跟十位和个位有关，所以num每次迭代时都%10避免溢出
        num = (num * 2 + bit) % 10;
        ret.push(num % 5 == 0);
    }
    ret
}

/// https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/
fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut max_width = 0;
    let mut max_width_count = 0;
    for rec in rectangles.into_iter() {
        let width = rec[0].min(rec[1]);
        match width.cmp(&max_width) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => {
                max_width_count += 1;
            }
            std::cmp::Ordering::Greater => {
                max_width = width;
                max_width_count = 1;
            }
        }
    }
    max_width_count
}

/// https://leetcode.com/problems/calculate-money-in-leetcode-bank/
fn total_money(mut n: i32) -> i32 {
    let mut nth_week = 1;
    let mut ret = 0;
    while n >= 7 {
        ret += (nth_week + nth_week + 6) * 7 / 2;
        n -= 7;
        nth_week += 1;
    }
    ret += (nth_week + nth_week + n - 1) * n / 2;
    ret
}

/// https://leetcode.com/problems/decode-xored-array/
fn decode_xored_array(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut ret = vec![first];
    for each in encoded {
        ret.push(ret.last().unwrap() ^ each);
    }
    ret
}

#[test]
fn test_decode_xored_array() {
    /*
    encoded: 1 2 3
    decoded: 1 ? ? ? (first=1)
    decoded[0]^decoded[1]=encoded[0]
    1^decoded[1]=1
    1^1^decoded[1]=1^1
    decoded[1]=0
    */
    const TEST_CASES: [(&[i32], i32, &[i32]); 1] = [(&[1, 2, 3], 1, &[1, 0, 2, 1])];
    for &(encoded, first, decoded) in &TEST_CASES {
        assert_eq!(decode_xored_array(encoded.into(), first), decoded);
    }
}

/// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    // word1.join("") == word2.join("")
    let mut s1 = Vec::new();
    let mut s2 = Vec::new();
    for word in word1 {
        s1.extend(word.into_bytes());
    }
    for word in word2 {
        s2.extend(word.into_bytes());
    }
    s1 == s2
    // if s1.len() != s2.len() {
    //     return false;
    // }
    // for (ch1, ch2) in s1.into_iter().zip(s2.into_iter()) {
    //     if ch1 != ch2 {
    //         return false;
    //     }
    // }
    // true
}

/// https://leetcode.com/problems/design-twitter/
struct Twitter {
    tweets: Vec<Tweet>,
    /// key: user_id, value: user_following
    user_following: std::collections::HashMap<i32, std::collections::HashSet<i32>>,
}

struct Tweet {
    tweet_id: i32,
    user_id: i32,
}

impl Twitter {
    /** Initialize your data structure here. */
    #[inline]
    fn new() -> Self {
        Self {
            tweets: vec![],
            user_following: std::collections::HashMap::new(),
        }
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push(Tweet { tweet_id, user_id });
        self.user_following
            .entry(user_id)
            .or_insert_with(|| [user_id].iter().copied().collect());
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed.
    Each item in the news feed must be posted by users who the user followed or by the user herself.
    Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        self.user_following
            .get(&user_id)
            .map(|following| {
                self.tweets
                    .iter()
                    .rev()
                    .filter(|tweet| following.contains(&tweet.user_id))
                    .take(10)
                    .map(|tweet| tweet.tweet_id)
                    .collect()
            })
            .unwrap_or_default()
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, user_id: i32, followee_id: i32) {
        match self.user_following.get_mut(&user_id) {
            Some(following) => {
                following.insert(followee_id);
            }
            None => {
                self.user_following
                    .insert(user_id, [user_id, followee_id].iter().copied().collect());
            }
        }
        self.user_following
            .entry(user_id)
            .or_default()
            .insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, user_id: i32, followee_id: i32) {
        if user_id == followee_id {
            // 自己不能取关自己
            return;
        }
        if let Some(following) = self.user_following.get_mut(&user_id) {
            following.remove(&followee_id);
        }
    }
}

#[test]
fn test_design_twitter() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
    twitter.unfollow(1, 2);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
}

/// https://leetcode.com/problems/add-to-array-form-of-integer/
fn add_to_array_form(mut a: Vec<i32>, mut k: i32) -> Vec<i32> {
    a.reverse();
    let mut carry = 0;
    let mut i = 0;
    while k != 0 || carry == 1 {
        // 如果有进位
        if i == a.len() {
            a.push(0);
        }

        let temp = a[i] + k % 10 + carry;
        a[i] = temp % 10;
        carry = temp / 10;

        k /= 10;
        i += 1;
    }
    a.reverse();
    a
}

#[cfg(not)]
fn add_to_array_form_overflow(a: Vec<i32>, k: i32) -> Vec<i32> {
    let mut num = 0u128;
    for digit in a.into_iter() {
        num = num * 10 + digit as u128;
    }
    num += k as u128;
    if num == 0 {
        return vec![0];
    }
    let mut ret = vec![];
    while num != 0 {
        ret.push((num % 10) as i32);
        num /= 10;
    }
    ret.reverse();
    ret
}

#[test]
fn test_add_to_array_form() {
    const TEST_CASES: [(&[i32], i32, &[i32]); 3] = [
        (&[1, 2, 0, 0], 34, &[1, 2, 3, 4]),
        (&[0], 999, &[9, 9, 9]),
        (&[9], 1, &[1, 0]),
    ];
    for &(a, k, output) in TEST_CASES.iter() {
        assert_eq!(add_to_array_form(a.to_vec(), k), output);
    }
}

/// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let n = prices.len();
    let mut ret = vec![];
    for i in 0..n {
        let mut discount = 0;
        for j in i + 1..n {
            if prices[j] <= prices[i] {
                discount = prices[j];
                break;
            }
        }
        ret.push(prices[i] - discount);
    }
    ret
}

/// https://leetcode.com/problems/sort-array-by-increasing-frequency/
/// 按出现次数少到次数多排序，如果出现次数相同，则按数值大小倒序
fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut counter = [0u8; 201];
    let mut max_freq = 1u8;
    for num in nums {
        let idx = (num + 100) as usize;
        counter[idx] += 1;
        max_freq = max_freq.max(counter[idx]);
    }
    // 类似桶排序的思想
    let mut ret = vec![vec![]; (max_freq + 1) as usize];
    for i in (0..201).rev() {
        let num = i as i32 - 100;
        let count = counter[i] as usize;
        ret[count].append(&mut vec![num].repeat(count));
    }
    ret.into_iter().fold(vec![], |mut a, mut b| {
        a.append(&mut b);
        a
    })
}

#[test]
fn test_frequency_sort() {
    const TEST_CASES: [(&[i32], &[i32]); 1] = [(&[2, 3, 1, 3, 2], &[1, 3, 3, 2, 2])];
    for &(input, output) in &TEST_CASES {
        assert_eq!(frequency_sort(input.into()), output);
    }
}

/// https://leetcode.com/problems/find-pivot-index/
fn pivot_index(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 3 {
        return -1;
    }

    let mut left_sum = 0;
    let mut right_sum: i32 = nums.iter().skip(1).sum();
    if left_sum == right_sum {
        return 0;
    }

    for pivot in 1..n {
        left_sum += nums[pivot - 1];
        right_sum -= nums[pivot];
        if left_sum == right_sum {
            return pivot as i32;
        }
    }

    -1
}

#[test]
fn test_pivot_index() {
    const TEST_CASES: [(&[i32], i32); 3] = [
        (&[1, 7, 3, 6, 5, 6], 3),
        (&[1, 2, 3], -1),
        (&[-1, -1, -1, 0, 1, 1], 0),
    ];
    for &(input, output) in &TEST_CASES {
        assert_eq!(pivot_index(input.into()), output);
    }
}

/// https://leetcode.com/problems/find-the-highest-altitude/
fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut cur = 0;
    let mut max = 0;
    for each in gain {
        cur += each;
        max = max.max(cur);
    }
    max
}

#[test]
fn test_largest_altitude() {
    const TEST_CASES: [(&[i32], i32); 1] = [(&[-5, 1, 5, 0, -7], 1)];
    for &(input, output) in &TEST_CASES {
        assert_eq!(largest_altitude(input.into()), output);
    }
}

/// https://leetcode.com/problems/count-the-number-of-consistent-strings/
fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut counter = [false; 26];
    for ch in allowed.into_bytes() {
        counter[(ch - b'a') as usize] = true;
    }
    let mut ret = 0;
    'loop_words: for word in words {
        for ch in word.into_bytes() {
            if !counter[(ch - b'a') as usize] {
                continue 'loop_words;
            }
        }
        ret += 1;
    }
    ret
}

#[test]
fn test_count_consistent_strings() {
    const TEST_CASES: [(&str, &[&str], i32); 1] =
        [("ab", &["ad", "bd", "aaab", "baa", "badab"], 2)];
    for &(allowed, words, output) in &TEST_CASES {
        let words = words.iter().copied().map(ToString::to_string).collect();
        assert_eq!(count_consistent_strings(allowed.to_string(), words), output);
    }
}

/// https://leetcode.com/problems/fair-candy-swap/
/// 其实就是two_sum两数之差的变种题
fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let a_sum: i32 = a.iter().sum();
    let b_sum: i32 = b.iter().sum();
    let avg_sum = (a_sum + b_sum) / 2;
    // a_sum + b - a = avg_sum
    // b - a = target
    let target = avg_sum - a_sum;
    // 现在就转为两数之差(two sum)类解法了
    let mut map = std::collections::HashMap::with_capacity(b.len());
    for b in b {
        map.insert(b - target, b);
    }
    for a in a {
        if let Some(b) = map.get(&a) {
            return vec![a, *b];
        }
    }
    unreachable!()
}

#[test]
fn test_fair_candy_swap() {
    const TEST_CASES: [(&[i32], &[i32], &[i32]); 4] = [
        (&[1, 1], &[2, 2], &[1, 2]),
        (&[1, 2], &[2, 3], &[1, 2]),
        (&[2], &[1, 3], &[2, 3]),
        (&[1, 2, 5], &[2, 4], &[5, 4]),
    ];
    for &(a, b, output) in &TEST_CASES {
        assert_eq!(fair_candy_swap(a.into(), b.into()), output.to_vec());
    }
}

/// https://leetcode.com/problems/maximum-average-subarray-i/
fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut window_sum: i32 = nums.iter().take(k).sum();
    let mut max_sum = window_sum;
    for i in k..nums.len() {
        window_sum = window_sum - nums[i - k] + nums[i];
        max_sum = max_sum.max(window_sum);
    }
    max_sum as f64 / k as f64
}

#[test]
fn test_find_max_average() {
    const TEST_CASES: [(&[i32], i32, f64); 1] = [(&[1, 12, -5, -6, 50, 3], 4, 12.75)];
    for &(nums, k, output) in &TEST_CASES {
        assert_eq!(find_max_average(nums.to_vec(), k), output);
    }
}
