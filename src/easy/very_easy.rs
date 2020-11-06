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
    const TESTCASES: [(&[i32], i32, &[i32]); 1] = [(&[2, 5, 1, 3, 4, 7], 3, &[2, 3, 5, 4, 1, 7])];
    for &(nums, n, expected) in TESTCASES.iter() {
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
    const TESTCASES: [(&str, i32); 3] = [("", 0), ("()()", 1), ("()(()())", 2)];
    for &(s, expected) in TESTCASES.iter() {
        assert_eq!(max_depth(s.to_owned()), expected);
    }
}
