//! leetcode较简单题，可能需要点脑筋急转弯(贪心)或我偷看答案才写出了
//! 或者是我没怎么看懂题目偷看答案后发现此题很无聊

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
            #[allow(clippy::suspicious_operation_groupings)]
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

/// https://leetcode.com/problems/hanota-lcci/
fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
    // std::mem::swap(a, c);
    fn move_top_down(n: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        if n == 0 {
            return;
        }
        // 先将a前n-1个圆盘经由c移到b
        move_top_down(n - 1, a, c, b);
        // 把a最底下(也就最后一个/最大圆盘)从a移到b
        c.push(a.pop().unwrap());
        // 再将b的所有圆盘经由a移到c
        move_top_down(n - 1, b, a, c);
    }
    move_top_down(a.len(), a, b, c);
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

/// https://leetcode.com/problems/can-place-flowers/
fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    // 头尾加上0，这样就不用边界检查(这个我没想到，还有戳气球dp那题也是头尾加个分值为1的气球避免边界情况要单独讨论)
    flowerbed.insert(0, 0);
    flowerbed.push(0);
    let mut ret = 0i32;
    let len = flowerbed.len();
    for i in 1..len - 1 {
        if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
            flowerbed[i] = 1;
            ret += 1;
        }
    }
    n <= ret
}

/// https://leetcode.com/problems/corporate-flight-bookings
#[allow(clippy::needless_range_loop)]
fn corp_flight_bookings(records: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut stations = vec![0; n + 1];
    for record in records {
        // 每个record的下标 0=上车站点, 1=下车站点, 2=上下车的人数
        let cnt = record[2];
        stations[record[0] as usize - 1] += cnt;
        stations[record[1] as usize] -= cnt;
    }
    let mut curr = 0;
    // 根据差分数组还原原数组
    for i in 0..=n as usize {
        curr += stations[i];
        stations[i] = curr;
    }
    stations.pop();
    stations
}

#[test]
fn test_corp_flight_bookings() {
    let test_cases = vec![(
        vec_vec![[1, 2, 10], [2, 3, 20], [2, 5, 25]],
        5,
        vec![10, 55, 45, 25, 25],
    )];
    for (records, n, output) in test_cases.into_iter() {
        assert_eq!(corp_flight_bookings(records, n), output);
    }
}

/// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/
/// 不能想当然的去比较三文治0的个数和需要三文治0的学生数，假设三文治前两个是0，后面有999个1，学生有1个0和999个1，因为第二个三明治是0卡住了后面999全是1的学生
fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut ones = students.into_iter().sum::<i32>();
    // 既然数组全由0和1组成，那么0的个数就等于 len-sum
    let mut zeros = sandwiches.len() as i32 - ones;
    for sandwich in sandwiches {
        if sandwich == 0 {
            if zeros == 0 {
                break;
            }
            zeros -= 1;
        } else {
            if ones == 0 {
                break;
            }
            ones -= 1;
        }
    }
    ones + zeros
}

#[test]
fn test_count_students() {
    const TEST_CASES: [(&[i32], &[i32], i32); 1] = [(&[1, 1, 1, 0, 0, 1], &[1, 0, 0, 0, 1, 1], 3)];
    for &(students, sandwiches, n_students_not_eat) in &TEST_CASES {
        assert_eq!(
            count_students(students.to_vec(), sandwiches.to_vec()),
            n_students_not_eat
        );
    }
}

/// https://leetcode.com/problems/goal-parser-interpretation/
fn goal_parser_interpret(command: String) -> String {
    let s = command.into_bytes();
    let n = s.len();
    let mut ret = Vec::with_capacity(n);
    let mut i = 0;
    while i < n {
        match s[i] {
            b'G' => {
                ret.push(b'G');
                i += 1;
            }
            b'(' => {
                if s[i + 1] == b')' {
                    ret.push(b'o');
                    i += 2;
                } else {
                    ret.push(b'a');
                    ret.push(b'l');
                    i += 4;
                }
            }
            _ => unreachable!(),
        }
    }
    unsafe { String::from_utf8_unchecked(ret) }
}

#[test]
fn test_goal_parser_interpret() {
    const TEST_CASE: [(&str, &str); 2] = [("()()", "oo"), ("G()(al)", "Goal")];
    for &(input, output) in TEST_CASE.iter() {
        assert_eq!(goal_parser_interpret(input.to_string()), output.to_string())
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

/// https://leetcode.com/problems/design-an-ordered-stream/
/// 这题一开始没看懂题目在说什么，偷看答案后发现挺无聊的
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

/// https://lintcode.com/problem/1-bit-and-2-bit-characters/
/// 以1开头的字符长度是2，以0开头的字符长度是1，问你最后一个字符属于长度为1内还是属于长度为2的字符内，直接遍历到倒数第二个字符即可
fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let n = bits.len();
    let last_idx = n - 1;
    let mut i = 0usize;
    while i < last_idx {
        if bits[i] == 0 {
            i += 1;
        } else {
            i += 2;
        }
    }
    i == last_idx
}

/// https://leetcode.com/problems/arithmetic-slices/
fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut continues_arithmetic_len = 0;
    for i in 2..a.len() {
        if a[i - 1] - a[i - 2] == a[i] - a[i - 1] {
            continues_arithmetic_len += 1
        } else {
            ret += continues_arithmetic_len * (continues_arithmetic_len + 1) / 2;
            continues_arithmetic_len = 0;
        }
    }
    ret + continues_arithmetic_len * (continues_arithmetic_len + 1) / 2
}

/** https://leetcode.com/problems/rotated-digits/
这题的题目描述信息太少表述不清，大意指的是数码管(seven-segment display)显示的 0-9 数字
只有 2和5 互为左右翻转，6和9互为上下翻转
0,1,8上下翻转后还是自身，3,4,7翻转后不是合法的数字
请问一个十进制数num的如果能翻转，翻转后的数不等于自身，则称num为好数，统计1..=n中一共有几个好数
*/
fn rotated_digits(n: i32) -> i32 {
    fn is_good(mut num: i32) -> bool {
        // 因为n<=10000而且10000不是好数，所以可以认为至多是4位数
        let mut digits = [0u8; 4];
        for i in (0..4).rev() {
            let digit = (num % 10) as u8;
            if digit == 3 || digit == 4 || digit == 7 {
                // 只要有一位不合法，就不是好数
                return false;
            }
            digits[i] = digit;
            num /= 10;
        }
        for &digit in &digits {
            // 如果所有数字都合法，只要有一位翻转后的数字不一样，就是好数
            if digit == 2 || digit == 5 || digit == 6 || digit == 9 {
                return true;
            }
        }
        false
    }
    (1..=n).into_iter().filter(|&num| is_good(num)).count() as i32
}

/// https://leetcode.com/problems/distribute-candies/
/// 由于糖果数量是偶数，而且需要平分，那么可以分为以下两种情况
/// 1. candy种类大于等于n/2，每种拿1个最多拿n/2种
/// 2. candy种类小于n/2，每种拿一个后还不够n/2，绰绰有余，所以糖果A可以多拿几个
/// 这题我本来想的是遍历生成counter后，count>1的糖果挨个减1，这样能尽量保留多点种类(如果动物标本要扔掉一半，尽量保留更多的种类)，
/// 如果丢弃数还不到一半，再随便找些count=1的扔掉，但是太懒了没写偷看答案了
fn distribute_candies(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let set: std::collections::HashSet<i32> = nums.into_iter().collect();
    set.len().min(len / 2) as i32
    // nums.sort_unstable();
    // nums.dedup();
    // nums.len().min(len / 2) as i32
}

/// https://leetcode.com/problems/check-if-it-is-a-straight-line/
fn check_straight_line(p: Vec<Vec<i32>>) -> bool {
    let dx0 = p[1][0] - p[0][0];
    let dy0 = p[1][1] - p[0][1];
    let n = p.len();
    for i in 2..n {
        if (p[i][1] - p[i - 1][1]) * dx0 != (p[i][0] - p[i - 1][0]) * dy0 {
            return false;
        }
    }
    true
}

/// https://leetcode.com/problems/maximum-product-of-three-numbers/
/// 要么全正选最大三个，要么两负一正
fn maximum_product(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    (nums[n - 1] * nums[n - 2] * nums[n - 3]).max(nums[0] * nums[1] * nums[n - 1])
}
