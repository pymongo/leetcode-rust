/*! 简短一行Rust代码能解决的题:
- [剑指Offer 17. 打印从1到最大n位的十进制数]: (1..10i32.pow(n as u32)).collect()
*/

/** https://leetcode.com/problems/shuffle-the-array/
数组nums按 \[x1,x2,...,xn,y1,y2,...,yn] 的格式排列
请你将数组按 [x1,y1,x2,y2,...,xn,yn] 格式重新排列
*/
fn shuffle_the_array(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n: usize = n as usize;
    let mut ret: Vec<i32> = Vec::with_capacity(2 * n);
    for i in 0..n {
        ret.push(nums[i]);
        ret.push(nums[i + n]);
    }
    ret
}

#[test]
fn test_shuffle() {
    const TEST_CASES: [(&[i32], i32, &[i32]); 1] = [(&[2, 5, 1, 3, 4, 7], 3, &[2, 3, 5, 4, 1, 7])];
    for &(nums, n, expected) in TEST_CASES.iter() {
        let output = shuffle_the_array(nums.to_vec(), n);
        assert_eq!(&output[..], expected);
    }
}

/** https://leetcode.com/problems/climbing-stairs/
& https://leetcode.com/problems/fibonacci-number/
比尾递归O(n)更快的还有O(log(n))的解法：
1. 斐波那契公式(公式中的乘方需要log(n)时间复杂度)
2. Binet's formula 利用矩阵解斐波那契
*/
const fn fib_recursive(n: u32, a: u32, b: u32) -> u32 {
    if n == 1 {
        b
    } else {
        // 注意尾递归解法只能从1逼近n，普通递归解法一般是从f(n-1)一直加到f(1)
        fib_recursive(n - 1, b, a + b)
    }
}

const fn fib_iterative(n: i32) -> i32 {
    let (mut a, mut b) = (1u32, 1u32);
    let mut temp: u32;
    let mut n = n;
    while n > 1 {
        temp = b;
        b += a;
        a = temp;
        n -= 1;
    }
    b as i32
}

/// https://leetcode.com/problems/to-lower-case/
fn to_lower_case(s: String) -> String {
    // 既然是ASCII编码，更高效的做法可能是u8数组判断在大写范围的挨个-32
    s.to_ascii_lowercase()
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

/** https://leetcode.com/problems/check-array-formation-through-concatenation/
```compile_failed
fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    let mut pieces_index = vec![None; 101];
    for ref piece in pieces {
        // creates a temporary which is freed while still in use
        pieces_index[piece[0] as usize] = Some(piece);
    }
    false
}
```
*/
fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    // arr[i]/pieces[i][j] range 1..=100
    const UNINIT: usize = 101;
    let mut pieces_index = vec![UNINIT; 101];
    for (i, piece) in pieces.iter().enumerate() {
        // since integers in pieces are distinct, so each piece[0] is distinct
        pieces_index[piece[0] as usize] = i;
    }
    let (mut i, n) = (0usize, arr.len());
    while i < n {
        let idx = pieces_index[arr[i] as usize];
        if idx != UNINIT {
            let piece = &pieces[idx];
            let (mut j, m) = (0usize, piece.len());
            while j < m && piece[j] == arr[i] {
                i += 1;
                j += 1;
            }
            // 如果piece的所有数字没有全被用上，也返回false
            if j < m {
                return false;
            }
        } else {
            // 因为arr和pieces都是unique/distinct的，如果所有pieces都不是以arr[i]开头则不匹配
            return false;
        }
    }
    true
}

/// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
    // arr.sort_by_cached_key(|&x| (x.count_ones, x));
    arr.sort_unstable_by(|a, b| {
        // 下面这行加起来的算法还不如不加呢，lazy一点只有当a和b的count_ones相同的时候才继续往后比较
        // (a.count_ones() as i32 + a).cmp(&(b.count_ones() as i32 + b))
        a.count_ones().cmp(&b.count_ones()).then(a.cmp(b))
    });
    arr
}

/// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    // points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    // let mut max_width = 0;
    // let mut last_x = points[0][0];
    // for point in points.into_iter().skip(1) {
    //     max_width = max_width.max(point[0] - last_x);
    //     last_x = point[0];
    // }
    // max_width
    let mut points_x: Vec<i32> = points.into_iter().map(|v| v[0]).collect();
    points_x.sort_unstable();
    // TODO 能不能用flat_map将windows拆成(a, b)这样的元组?
    points_x
        .windows(2)
        .map(|a| a[1] - a[0])
        .max()
        .unwrap_or_default()
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

/// https://leetcode.com/problems/max-increase-to-keep-city-skyline/
/// 大意: 先算出旧矩阵每行每列的最大值，然后遍历矩阵看看当前值最大能加到什么，然后累加最大能增加的量
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

/// https://leetcode.com/problems/set-matrix-zeroes/
/// 需求: 如果矩阵的某个元素为0，则将0所在行和所在列的全部元素置0
/// 注意: 要先遍历一次矩阵，发现哪些坐标是0，然后再将相应行和列置零，不能一边遍历一边置零否则会污染后面的元素
fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());
    // 已经设成全为0的行和列
    let (mut zero_row, mut zero_col) = (vec![false; m], vec![false; n]);
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                zero_row[i] = true;
                zero_col[j] = true;
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if zero_row[i] || zero_col[j] {
                matrix[i][j] = 0;
            }
        }
    }
}

/// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
/// 求字符串内有效括号的最大深度
fn max_depth(s: String) -> i32 {
    let mut ret = 0;
    let mut depth = 0;
    for byte in s.into_bytes() {
        // 由于leetcode这题暂时没有全是左括号例如"((("的测试用例，所以这样也能AC
        match byte {
            b'(' => {
                depth += 1;
                ret = ret.max(depth);
            }
            b')' => {
                depth -= 1;
            }
            _ => {}
        }
    }
    ret
}

#[test]
fn test_max_depth() {
    const TEST_CASES: [(&str, i32); 3] = [("", 0), ("()()", 1), ("()(()())", 2)];
    for &(s, expected) in TEST_CASES.iter() {
        assert_eq!(max_depth(s.to_owned()), expected);
    }
}

/// https://leetcode.com/problems/design-parking-system/
struct ParkingSystem {
    big_slot_cap: u16,
    medium_slot_cap: u16,
    small_slot_cap: u16,
}

impl ParkingSystem {
    /// 0 <= big, medium, small <= 1000
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

/// https://leetcode.com/problems/ugly-number/
fn is_ugly(mut num: i32) -> bool {
    if num == 0 {
        return false;
    }
    while num % 2 == 0 {
        num /= 2;
    }
    while num % 3 == 0 {
        num /= 3;
    }
    while num % 5 == 0 {
        num /= 5;
    }
    num == 1
}

/// https://leetcode.com/problems/valid-number/
/// 这题正确的解法应该是DFA(有限状态机)，手写的状态机应该会比标准库的f32解析状态机性能更好
fn is_number(s: String) -> bool {
    s.trim().parse::<f32>().is_ok()
}

/// https://leetcode.com/problems/island-perimeter/
/// 逐行遍历grid中所有为1的格子，遇到一个1就往上下左右四个方向延伸，遇到边界或0就周长加一，遇到1则不加
fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut perimeter = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                continue;
            }
            // up and down
            if i == 0 || grid[i - 1][j] == 0 {
                perimeter += 1;
            }
            if i == m - 1 || grid[i + 1][j] == 0 {
                perimeter += 1;
            }
            // left and right
            if j == 0 || grid[i][j - 1] == 0 {
                perimeter += 1;
            }
            if j == n - 1 || grid[i][j + 1] == 0 {
                perimeter += 1;
            }
        }
    }
    perimeter
}

#[test]
fn test_island_perimeter() {
    let test_cases: Vec<(Vec<Vec<i32>>, i32)> = vec![(
        vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ],
        16,
    )];
    for (grid, perimeter) in test_cases {
        assert_eq!(island_perimeter(grid), perimeter)
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

/// https://leetcode.com/problems/matrix-diagonal-sum/
/// 本题仅要求算两条主对角线，既↘和↙两个方向的最长对角线
fn matrix_diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let mut ret = 0;

    for j in 0..n {
        // 累加左上-右下对角线
        ret += mat[n - j - 1][j];
        // 累加左下-右上对角线
        ret += mat[j][j];
    }

    // 如果是矩阵长度为奇数，则中间元素会被重复计算，需要去掉
    if n % 2 == 1 {
        ret -= mat[n / 2][n / 2];
    }

    ret
}

#[test]
fn test_diagonal_sum() {
    const TEST_CASES: [(&[&[i32]], i32); 2] = [
        (
            &[&[1, 1, 1, 1], &[1, 1, 1, 1], &[1, 1, 1, 1], &[1, 1, 1, 1]],
            8,
        ),
        (&[&[5]], 5),
    ];
    for &(mat, res) in &TEST_CASES {
        let n = mat.len();
        let mut mat_vec = Vec::with_capacity(n);
        for &row in mat {
            mat_vec.push(row.to_vec());
        }
        assert_eq!(matrix_diagonal_sum(mat_vec), res);
    }
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
    // 跳出循环时必定是`if *digit == 9`的分支
    digits.insert(0, 1);
    digits
}

#[cfg(FALSE)]
/// https://leetcode.com/problems/random-pick-index/
/// 应对很长的无法全部存入内存的数组online data，正统做法应该用: 蓄水池抽样(Random Reservoir Sampling)
struct RandomPickIndex {
    rand_thread_rng: rand::rngs::ThreadRng,
    index: std::collections::HashMap<i32, Vec<i32>>,
}

#[cfg(FALSE)]
impl RandomPickIndex {
    fn new(nums: Vec<i32>) -> Self {
        let mut nums_index = std::collections::HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            nums_index
                .entry(num)
                .or_insert_with(Vec::new)
                .push(i as i32);
        }
        Self {
            rand_thread_rng: rand::thread_rng(),
            index: nums_index,
        }
    }

    /// 如果nums中存在多个target，则等概率地随机返回一个满足nums[i]=target的下标i
    fn pick(&mut self, target: i32) -> i32 {
        use rand::seq::SliceRandom;
        *self
            .index
            .get(&target)
            .unwrap()
            .choose(&mut self.rand_thread_rng)
            .unwrap()
    }
}

/// https://leetcode.com/problems/top-k-frequent-elements/
/// return [num for num, _ in collections.Counter(nums).most_common(k)]
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut counter = std::collections::HashMap::<i32, i32>::with_capacity(n);
    for &num in &nums {
        *counter.entry(num).or_insert(0) += 1;
    }
    // 小根堆: (-出现次数, 数字)，所以堆顶会是出现次数最低的数字，随时可以被别人挤掉
    let mut heap = std::collections::BinaryHeap::<(i32, i32)>::with_capacity(k);
    for (&num, &cnt) in &counter {
        if heap.len() == k {
            if -cnt < heap.peek().unwrap().0 {
                heap.pop();
                heap.push((-cnt, num));
            }
        } else {
            heap.push((-cnt, num));
        }
    }
    heap.into_iter().rev().map(|(_, num)| num).collect()
}

/// https://leetcode.com/problems/defanging-an-ip-address/
fn defanging_an_ip_address(address: String) -> String {
    address.replace(".", "[.]")
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
        vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ],
        vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1],
        ],
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

/// https://leetcode.com/problems/create-target-array-in-the-given-order/submissions/
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

/// https://leetcode.com/problems/range-sum-query-immutable/submissions/
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

/** https://leetcode.com/problems/2-keys-keyboard/
初次看到这题，我还以为是用倍增法，例如要生成9个字符，我以我是2**3+1，最后一下鼠标复制一个字符再粘贴
结果这题只能是「全选后复制粘贴」
所以如果n是质数，那就只能就最初的1个字母复制1次，粘贴n-1次
如果n是非质数: 答案就是n分解质因数的因子之和，例如6=2*3，次数是5
*/
fn copy_and_paste_min_steps(mut n: i32) -> i32 {
    let mut factor = 2;
    let mut factor_sum = 0;
    while n > 1 {
        while n % factor == 0 {
            n /= factor;
            factor_sum += factor;
        }
        factor += 1;
    }
    factor_sum
}

/// https://leetcode.com/problems/design-an-ordered-stream/
struct OrderedStream {
    data: Vec<Option<String>>,
    len: usize,
    ptr: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        let n = (n + 1) as usize;
        Self {
            data: vec![None; n],
            len: n,
            ptr: 1,
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        let mut ret = Vec::new();
        let idx = id as usize;
        self.data[idx] = Some(value);
        if self.ptr == idx {
            for i in idx..self.len {
                if let Some(s) = self.data[i].take() {
                    ret.push(s);
                } else {
                    // Then, update ptr to the last id + 1
                    self.ptr = i;
                    break;
                }
            }
        }
        ret
    }
}

/// https://leetcode.com/problems/group-anagrams/
/// 由于Python没有原始数组，list是可变的不能Hash，所以list要转为tuple多了很多额外的操作
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut group = std::collections::HashMap::new();
    for s in strs.into_iter() {
        let mut counter = [0u8; 26]; // 0 <= strs[i].length <= 100
        for &byte in s.as_bytes() {
            counter[(byte - b'a') as usize] += 1;
        }
        group.entry(counter).or_insert_with(Vec::new).push(s)
    }
    // same as nightly `into_values` API: consume HashMap and get a vec of values
    group.into_iter().map(|(_k, v)| v).collect()
}

/// https://leetcode.com/problems/increasing-decreasing-string/
fn sort_string(s: String) -> String {
    let n = s.len();
    let mut counter = [0u8; 26];
    for byte in s.into_bytes().into_iter() {
        counter[(byte - b'a') as usize] += 1;
    }

    let mut ret = Vec::with_capacity(n);
    while ret.len() < n {
        for i in 0..26 {
            if counter[i] > 0 {
                counter[i] -= 1;
                ret.push(b'a' + i as u8);
            }
        }
        for i in (0..26).rev() {
            if counter[i] > 0 {
                counter[i] -= 1;
                ret.push(b'a' + i as u8);
            }
        }
    }

    unsafe { String::from_utf8_unchecked(ret) }
}

#[test]
fn test_sort_string() {
    const TEST_CASES: [(&str, &str); 2] =
        [("aaaabbbbcccc", "abccbaabccba"), ("leetcode", "cdelotee")];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(sort_string(input.to_string()), output.to_string());
    }
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

/// https://leetcode.com/problems/4sum-ii/
fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut pairs = std::collections::HashMap::new();
    for num_a in a.into_iter() {
        for num_b in b.iter() {
            *pairs.entry(num_a + num_b).or_default() += 1;
        }
    }
    let mut count = 0;
    for num_c in c.into_iter() {
        for num_d in d.iter() {
            count += pairs.get(&(-num_c - num_d)).unwrap_or(&0);
        }
    }
    count
}

/// https://leetcode.com/problems/largest-perimeter-triangle/
/// 贪心的角度去想，排序后从右到左遍历连续的三个数，就能找到较长周长的三角形
fn largest_perimeter(mut a: Vec<i32>) -> i32 {
    a.sort_unstable();
    for i in (2..a.len()).rev() {
        if a[i - 2] + a[i - 1] > a[i] {
            return a[i - 2] + a[i - 1] + a[i];
        }
    }
    0i32
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

/// https://leetcode.com/problems/reveal-cards-in-increasing-order/
fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    deck.sort_unstable();
    let mut ret = vec![deck.pop().unwrap()];
    while let Some(deck_last) = deck.pop() {
        let ret_last = ret.pop().unwrap();
        ret.insert(0, ret_last);
        // error: ret.insert(0, ret.pop().unwrap());
        ret.insert(0, deck_last);
    }
    ret
}

/**
1. 排序deck: [17,13,11,2,3,5,7] => [2,3,5,7,11,13,17], ret: []
2. deck: [2,3,5,7,11,13], ret: [17]
3. deck: [2,3,5,7,11], ret: [13,17]
4. deck: [2,3,5,7], ret: [17,13] => [11,17,13]
...
*/
#[test]
fn test_deck_revealed_increasing() {
    assert_eq!(
        deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
        vec![2, 13, 3, 11, 5, 17, 7]
    );
}

/// https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/
fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
    // 注意题目的入参是n行m列，而非正常的m行n列
    let (m, n) = (n as usize, m as usize);
    // 一开始的矩阵，都是0，所以都不是偶数(false)
    let mut mat = vec![vec![false; n]; m];
    for indice in indices.into_iter() {
        let (row, col) = (indice[0] as usize, indice[1] as usize);
        for i in 0..m {
            mat[i][col] = !mat[i][col];
        }
        for j in 0..n {
            mat[row][j] = !mat[row][j];
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

/// https://leetcode.com/problems/shuffle-string/
/// 能不能用In-Place的swap操作完成重排？我联想到rotate_string那题教室换座位的情况，A的新座位在B，A挤到B的slot，把B挤出教室，然后B再去挤自己的新座位...
fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut ret = vec![0u8; s.len()];
    for (i, byte) in indices.into_iter().zip(s.into_bytes().into_iter()) {
        ret[i as usize] = byte;
    }
    unsafe { String::from_utf8_unchecked(ret) }
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

/** https://leetcode.com/problems/minimum-deletion-cost-to-avoid-repeating-letters/
```python
# 周赛#205第三题(https://leetcode-cn.com/contest/weekly-contest-205/problems/replace-all-s-to-avoid-consecutive-repeating-characters/)
@staticmethod
def f(s: str, cost: List[int]) -> int:
    n, i = len(cost), 1
    ret = 0
    while i < n:
        if s[i] == s[i - 1]:
            j = i
            # 找到连续的一片重复字母
            while j < n and s[j] == s[j - 1]:
                j += 1
            ret += sum(cost[i - 1:j]) - max(cost[i - 1:j])
            i = j
        else:
            i += 1
    return ret
```
花最小代价让字符串相邻两个元素不重复，所以遇到连续的重复字符，例如连续5个a，则需要删掉4个a，留下cost数组中耗费最大的那个a
*/
fn minimum_deletion_cost_to_avoid_repeating_letters(s: String, cost: Vec<i32>) -> i32 {
    let s = s.into_bytes();
    let n = s.len();
    let mut i = 0;
    let mut ret = 0;
    while i < n {
        let byte = s[i];
        let mut max_cost_of_same_byte = 0;
        let mut cost_sum = 0;
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
        minimum_deletion_cost_to_avoid_repeating_letters("abaac".into(), vec![1, 2, 3, 4, 5]),
        3
    );
}

/// https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/
fn replace_question_mark_to_avoid_consecutive_repeating_char(s: String) -> String {
    let mut s = s.into_bytes();
    let n = s.len();
    for i in 0..n {
        if s[i] != b'?' {
            continue;
        }
        for letter in b'a'..=b'z' {
            if let Some(left) = s.get(i.wrapping_sub(1)) {
                if letter.eq(left) {
                    continue;
                }
            }
            if let Some(right) = s.get(i + 1) {
                if letter.eq(right) {
                    continue;
                }
            }
            s[i] = letter;
        }
    }
    unsafe { String::from_utf8_unchecked(s) }
}

/// https://leetcode.com/problems/running-sum-of-1d-array/
fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
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
