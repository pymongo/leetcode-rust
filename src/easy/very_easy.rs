/*!
简短一行Rust代码能解决的题:
[剑指Offer 17. 打印从1到最大n位的十进制数]: (1..10i32.pow(n as u32)).collect()
*/

/** https://leetcode.com/problems/nim-game/
nim游戏规则：有若干个石头，两人每回合轮流拿走一些石头，每个人可以拿走1-3块石头
如果轮到A的回合时，石头数量是4的倍数，那么A必败(博弈问题的必败态)
或者利用二进制判断是不是 4 的倍数，
只需要通过和 3 （二进制 11）进行相与，
如果是 4 的倍数，那么结果一定是 0。

算法如下：
    x&3==0，则是4的倍数。
原理：
先来看一组数字的二进制表示
    4　　　　0100
    8　　　　1000
    12      1100
    16     10000
    20     10100
由此可见 4 的倍数的二进制表示的后 2 为一定为 0。

从另外一个角度来看，4 的二进制表示是 0100，任何 4 的倍数一定是在此基础上增加 n 个 0100
由此也可得 4 的倍数的二进制表示的后 2 为一定为 0。
*/
const fn nim_game_bitwise(n: i32) -> bool {
    // (n % 4) != 0
    (n & 3) != 0
}

/** https://leetcode.com/problems/shuffle-the-array/
数组nums按 \[x1,x2,...,xn,y1,y2,...,yn] 的格式排列

请你将数组按 [x1,y1,x2,y2,...,xn,yn] 格式重新排列
*/
fn shuffle_the_array(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n: usize = n as usize;
    let mut result: Vec<i32> = Vec::with_capacity(2 * n);
    for i in 0..n {
        result.push(nums[i]);
        result.push(nums[i + n]);
    }
    result
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
    let mut res = Vec::new();
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
        res.push(num);
    }
    res
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
    let mut res = 0;
    for i in 1..n {
        let dx = (points[i][0] - points[i - 1][0]).abs();
        let dy = (points[i][1] - points[i - 1][1]).abs();
        res += dx.max(dy);
    }
    res
}

/// https://leetcode.com/problems/max-increase-to-keep-city-skyline/
/// 大意: 先算出旧矩阵每行每列的最大值，然后遍历矩阵看看当前值最大能加到什么，然后累加最大能增加的量
fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut max_row: Vec<i32> = Vec::with_capacity(m);
    let mut max_col: Vec<i32> = vec![std::i32::MIN; n];
    for i in 0..m {
        max_row.push(*grid[i].iter().max().unwrap());
    }
    for j in 0..n {
        for i in 0..m {
            max_col[j] = max_col[j].max(grid[i][j]);
        }
    }

    let mut res = 0;
    for i in 0..m {
        let curr_max_row = max_row[i];
        for j in 0..n {
            // 最大能增长的高度等于行列最大值二者的最小值
            res += std::cmp::min(curr_max_row, max_col[j]) - grid[i][j];
        }
    }
    res
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
    let mut res = 0;
    let mut depth = 0;
    for byte in s.into_bytes() {
        // 由于leetcode这题暂时没有全是左括号例如"((("的测试用例，所以这样也能AC
        match byte {
            b'(' => {
                depth += 1;
                res = res.max(depth);
            }
            b')' => {
                depth -= 1;
            }
            _ => {}
        }
    }
    res
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
fn matrix_diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let mut res = 0;

    for j in 0..n {
        // 累加左上-右下对角线
        res += mat[n - j - 1][j];
        // 累加左下-右上对角线
        res += mat[j][j];
    }

    // 如果是矩阵长度为奇数，则中间元素会被重复计算，需要去掉
    if n % 2 == 1 {
        res -= mat[n / 2][n / 2];
    }

    res
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

/// https://leetcode.com/problems/number-of-1-bits/
/// Rust: n.count_ones(), Java: Integer.bitCount(n)
fn hamming_weight(n: u32) -> i32 {
    fn impl_count_ones_best(n: u32) -> i32 {
        let mut count = 0;
        let mut mask = 0b1;
        for _ in 0..32 {
            if n & mask == 1 {
                count += 1;
            }
            mask <<= 1;
        }
        count
    }
    fn impl_count_ones_by_mask(mut n: u32) -> i32 {
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
    impl_count_ones_best(n)
}

/// https://leetcode.com/problems/hamming-distance/
/// 两个整数之间的汉明距离指的是这两个数字对应二进制位不同的位置的数目
/// 思路: 异或后数位1的个数
fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

/// https://leetcode.com/problems/reverse-bits/
fn reverse_bits(x: u32) -> u32 {
    fn reverse_bits_best(mut n: u32) -> u32 {
        // ret = return
        let (mut ret, mut power) = (0u32, 0u32);
        while n != 0 {
            ret += (n & 1) << power;
            n >>= 1;
            power -= 1;
        }
        ret
    }

    x.reverse_bits()
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

/// https://leetcode.com/problems/random-pick-index/
/// 应对很长的无法全部存入内存的数组online data，正统做法应该用: 蓄水池抽样(Random Reservoir Sampling)
struct RandomPickIndex {
    rand_thread_rng: rand::rngs::ThreadRng,
    index: std::collections::HashMap<i32, Vec<i32>>,
}

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
    let mut res = Vec::with_capacity(n);
    for j in 0..n {
        let mut row = Vec::with_capacity(m);
        for i in 0..m {
            row.push(a[i][j]);
        }
        res.push(row);
    }
    res
}

/// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
fn max_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by_key(|&num| std::cmp::Reverse(num));
    (nums[0] - 1) * (nums[1] - 1)
}
