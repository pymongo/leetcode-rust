//! https://leetcode.com/problems/longest-palindromic-substring/
//! 本题多达五种解: O(n): Manacher，后缀数组...

struct Solution;

impl Solution {
    /// 斜着遍历矩阵不能命中CPU缓存，dp解法性能远不如中心对称
    fn dp(s: String) -> String {
        let s = s.into_bytes();
        let n = s.len();
        let (mut max_len, mut max_start) = (0usize, 0usize);
        let mut dp = vec![vec![false; n]; n];
        // 枚举子串的长度(枚举区间型动态规划的区间长度)，由于填表时依赖左下角的值，所以区间型动态规划最佳的填表方向是「左上-右下」对角线那样斜着填
        for len in 1..=n {
            // 枚举子串的起始位置i
            for i in 0..(n - len + 1) {
                let j = i + len - 1;
                if len == 1 {
                    // 最长的对角线
                    dp[i][j] = true;
                } else if len == 2 {
                    // 最长的对角线往右上的第二天对角线
                    dp[i][j] = s[i] == s[j];
                } else {
                    // 依赖更小区间(左下角值)是否回文和扩展首尾后首尾字符是否相等
                    dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];
                }
                if dp[i][j] && len > max_len {
                    max_len = len;
                    max_start = i;
                }
            }
        }
        unsafe { String::from_utf8_unchecked(s[max_start..max_start + max_len].to_owned()) }
    }

    // 除了manacher算法之外，用suffix array(后缀数组)算法也是O(n)的时间复杂度
    fn manacher(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }

        // 一、字符串预处理
        // 给字符串的头尾加上^和$，同时给原字符串的间隙加上#
        // 字符串预处理的功能：
        // 1. 这样中心扩展的时候，判断两端字符是否相等的时候，如果到了边界就一定会不相等
        // 2. 头尾的^和$不相等，避免了中心扩散时usize类型溢出成-1
        // 3. 字符串的长度变成了2n+3，永远为奇数个
        let mut new_str = vec!['^', '#'];
        for ch in s.chars() {
            new_str.push(ch);
            new_str.push('#');
        }
        new_str.push('$');

        // length of the new string
        let new_len = 2 * len + 3;

        // Define a secondary array p[], where p[i] represents the radius of the longest palindrome centered on i.
        let mut radius_of_i = vec![0usize; new_len];

        // `max_len`: The length of the longest palindrome string in the original string
        let mut max_radius = 0usize;
        let mut max_radius_center_index = 0usize;

        // `right` represents the right boundary of
        // the longest palindrome centered on`center`
        // right = center + radius_of_i[center]
        let mut center = 0usize;
        let mut right = 0usize;
        for i in 1..(new_len - 1) {
            if i < right {
                // [2 * center - i] is mirror of center for i
                radius_of_i[i] = radius_of_i[2 * center - i].min(right - i);
            } else {
                radius_of_i[i] = 1;
            }

            while new_str[i - radius_of_i[i]] == new_str[i + radius_of_i[i]] {
                radius_of_i[i] += 1;
            }

            if right < i + radius_of_i[i] {
                center = i;
                right = i + radius_of_i[i];
            }
            // FIXME 随着i逐渐右移，center对应的半径【可能不是】最大半径的中心点
            if radius_of_i[i] > max_radius {
                max_radius = radius_of_i[i];
                max_radius_center_index = i;
            }
        }

        // 因为center会不断往右移，center对应的半径【可能不是】最大半径的中心点
        // position的API类似于Find
        let max_left = max_radius_center_index + 1 - max_radius;
        let max_right = max_radius_center_index + max_radius - 1;
        let mut longest_palindrome_substring = String::with_capacity(max_radius - 1);
        for i in max_left..max_right {
            if new_str[i] != '#' {
                longest_palindrome_substring.push(new_str[i]);
            }
        }

        longest_palindrome_substring
    }

    /** Manacher算法
    可以认为是「动态规划」+「中心扩散」+「KMP算法(利用左边的已进行回文判断的子串)」
    KMP算法是用于字符串匹配时，判断一个字符串是否contains某个字符串


    预处理字符串(给字符串间隙加#)，避免了中心扩散解法的偶数情况要遍历字符串的间隙
    同时还会个字符串的头尾加上^和$，这样中心扩展的时候，判断两端字符是否相等的时候，如果到了边界就一定会不相等
    如果是Java，头和尾可以是#，但是Rust头尾一定要不一样的防止usize类型越界
    经过上述两个处理，字符串的长度永远是奇数了

    Manacher算法第二个核心概念是，利用了回文数的中心对称性
    例如最大回文串是'a#b#a'，长度+1除以2之后得到了aba的长度是3，中心位置是b索引为2，原字符串开始位置的索引为2-(3+1)/2
    例如最大回文串是'b#b'，长度+1除以2之后得到了bb的长度是2，中心位置是#索引为1，原字符串开始位置的索引为1-(2+1)/2
    */
    fn manacher_old(s: String) -> String {
        let input_str_len = s.len();
        if input_str_len < 2 {
            return s;
        }

        // 对输入的字符串作预处理
        // 在字符串的间隙间插入'#'，然后在字符串的开头和结束位置插入'^'和'$'
        let mut new_str = vec!['^', '#'];
        for char in s.chars() {
            new_str.push(char);
            new_str.push('#');
        }
        new_str.push('$');

        // 除了比原字符串多了n个#，还多了一个#、^、$，所以新字符串的长度是2n+3
        let len = 2 * input_str_len + 3;
        // 新字符串的回文半径=老字符串的最大长度
        let mut max_radius = 0usize;
        let mut max_radius_center_index = 0usize;

        // radius[i] represents the radius of the longest palindrome centered on i.
        let mut radius_of_i = vec![0usize; len];

        /* 以下两个变量，充分利用了回文数中心对称的特点，用到了动态规划，利用之前左边部分已经判别过回文的特点减少遍历 */
        // 当前已记录的最长回文子串 最远能向右扩散的索引
        // FIXME 注意当i即将超过right时，center和right会变，变了之后很可能不再是最长回文的索引
        let mut right = 0usize;
        // 当前已记录的最长回文子串 最远能向右扩散的中心索引
        // max_len_center_index = max_len_right_index + radius_of_i[max_len_right_index]
        let mut center = 0usize;
        // 因为center会不断往右移，center对应的半径【可能不是】最大半径的中心点
        let mut i_mirror_of_center;

        // 根据i和max_len_right_index之间的大小关系作分类讨论
        for i in 1..(len - 1) {
            // 情况1：一开始以及遍历到字符串末尾的情况
            if i >= right {
                // dbg!("情况1");
                radius_of_i[i] = 1;
                // 要用中心对称算法扩散i
                while new_str[i - radius_of_i[i]] == new_str[i + radius_of_i[i]] {
                    radius_of_i[i] += 1;
                }
                // 如果发现了更长的回文子串，更新center和right的索引
                if i + radius_of_i[i] > right {
                    center = i;
                    right = i + radius_of_i[i];
                }
                if radius_of_i[i] > max_radius {
                    max_radius = radius_of_i[i];
                    max_radius_center_index = i;
                }
            } else {
                // 情况2：i在right左边，但是不可能也在center的左边，因为center一定是访问过的
                //       所以这种情况下，i在center和right的中间
                // 情况2的分析具体看leetcode的题解「动态规划、中心扩散、Manacher 算法」
                // https://leetcode-cn.com/problems/longest-palindromic-substring/solution/zhong-xin-kuo-san-dong-tai-gui-hua-by-liweiwei1419/

                // 因为mirror+i = 2*center
                i_mirror_of_center = 2 * center - i;
                match radius_of_i[i_mirror_of_center].cmp(&(right - i)) {
                    // 情况2.1: 以i_mirror_of_center出发的回文串总体长度在最大半径之内，i_mirror_of_center中的半径【小于max_radius】
                    // 根据对称性，直接照抄镜像的值
                    std::cmp::Ordering::Less => {
                        // dbg!("情况2.1");
                        // 尽管这种情况下不用更新最大半径，但是【填值是必须的】方便遍历更右边时需要用到当前位置的值
                        radius_of_i[i] = radius_of_i[i_mirror_of_center];
                        if i + radius_of_i[i] > right {
                            center = i;
                            right = i + radius_of_i[i];
                        }
                    }
                    // 情况2.2: 【可能会更新max_radius】先把p[mirror] 的值抄过来，然后继续“中心扩散法”
                    std::cmp::Ordering::Equal => {
                        // dbg!("情况2.2");
                        radius_of_i[i] = radius_of_i[i_mirror_of_center];
                        while new_str[i - radius_of_i[i]] == new_str[i + radius_of_i[i]] {
                            radius_of_i[i] += 1;
                        }
                        if i + radius_of_i[i] > right {
                            center = i;
                            right = i + radius_of_i[i];
                        }
                        if radius_of_i[i] > max_radius {
                            max_radius = radius_of_i[i];
                            max_radius_center_index = i;
                        }
                    }
                    // 情况2.3:
                    std::cmp::Ordering::Greater => {
                        // dbg!("情况2.3");
                        radius_of_i[i] = right - i;
                    }
                }
            }
        }

        let max_left = max_radius_center_index + 1 - max_radius;
        let max_right = max_radius_center_index + max_radius - 1;
        let mut longest_palindrome_substring = String::with_capacity(max_radius - 1);
        for i in max_left..max_right {
            if new_str[i] != '#' {
                longest_palindrome_substring.push(new_str[i]);
            }
        }

        longest_palindrome_substring
    }

    /** 中心扩散算法(4ms)
    以cbbd为例，从索引0开始遍历到len-2，因为最后一个字符扩散出去也不可能是回文
    从第一个字符出发是为了让默认的最长回文子串是第一个字符

    遍历字符串内的每个奇数中心和偶数中心，奇数中心是cbbd的c、b、b三个字符为中心进行扩散
    偶数中心是 c和b间隙、b和b间隙、b和d间隙的三个间隙

    没想到中心对称的算法的性能比dp解法好多了，同样是O(n^2)的时间复杂度，中心对称算法空间复杂度是O(1)，耗时4ms
    */
    fn expand_around_center(s: String) -> String {
        fn helper(chars: &[u8], len: usize, left: usize, right: usize) -> usize {
            let (mut left, mut right) = (left, right);
            let mut left_is_palindromic_and_overflow = false;
            // dbg!((left, right));
            while right < len {
                if chars[left] == chars[right] {
                    if left == 0 {
                        left_is_palindromic_and_overflow = true;
                        break;
                    }
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
            // dbg!((left, right));
            if right == left {
                1
            } else if left_is_palindromic_and_overflow {
                right - left + 1
            } else {
                // 跳出循环时刚好满足chars[left] != chars[right]
                // 所以真正的长度是j-i-1
                right - left - 1
            }
        }
        let s = s.into_bytes();
        let len = s.len();
        if len < 2 {
            return unsafe { String::from_utf8_unchecked(s) };
        }

        let mut max_len = 0;
        let mut max_len_start_index = 0;
        // 奇数中心对称的长度(从字符串索引的字符出发)
        let mut odd_expand_len;
        // 偶数中心对称的长度(从字符串两个元素之间的间隙出发)
        let mut even_expand_len;
        let mut temp_len: usize;

        // 遍历到倒数第二个字符
        for i in 0..(len - 1) {
            // 从字符串内一个字符出发(奇数)
            odd_expand_len = helper(&s, len, i, i);
            // 从字符串内两个字符之间的间隙出发(偶数)
            even_expand_len = helper(&s, len, i, i + 1);
            temp_len = odd_expand_len.max(even_expand_len);
            if temp_len > max_len {
                max_len = temp_len;
                // 这步需要在纸上画图统一奇偶数的规律
                max_len_start_index = i - (temp_len - 1) / 2;
            }
        }
        let mut result = String::with_capacity(max_len);
        for i in max_len_start_index..(max_len_start_index + max_len) {
            result.push(s[i] as char);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    const TESTCASES: [(&str, &str); 6] = [
        ("babad", "bab"),
        ("abadd", "aba"),
        ("cbbd", "bb"),
        ("aba", "aba"),
        ("ac", "a"),
        ("ccc", "ccc"),
    ];

    #[test]
    fn test_dp() {
        for &(input, expected) in TESTCASES.iter() {
            assert_eq!(Solution::dp(input.to_string()), expected.to_string());
        }
    }

    #[test]
    fn test_manacher() {
        for &(input, expected) in TESTCASES.iter() {
            assert_eq!(Solution::manacher(input.to_string()), expected.to_string());
        }
    }

    #[test]
    fn test_manacher_old() {
        for &(input, expected) in &TESTCASES {
            assert_eq!(
                Solution::manacher_old(input.to_string()),
                expected.to_string()
            );
        }
    }

    #[test]
    fn test_expand_around_center() {
        for &(input, expected) in TESTCASES.iter() {
            assert_eq!(
                Solution::expand_around_center(input.to_string()),
                expected.to_string()
            );
        }
    }
}

/**
TODO 建议看Python版本的区间型动态规划
dp[i][j] 表示子串 s[i..j] 是否为回文子串
一个回文字符串去掉两头后，依然是个回文
if dp[i+1][j-1] && s[i]==s[j] {
    dp[i][j] = true;
}
例如求证aba(0,2)是不是回文，判断b(1,1)是不是回文 且 s[0]==s[2]?
初始条件
  c b b a (纵坐标是i——最长子串的开始索引)
c T 1 2 4
b   T 3 5
b     T 6
a       T

初始条件(输入是长度为3的回文串)
  a b a
a T 1 2
b   T 3
a     T
## 优化
当子串的长度是2或3时，不需要检查子串是否回文，所以边界条件可以是j-i<3
*/
fn dp_new(s: String) -> String {
    let s = s.into_bytes();
    let len = s.len();
    if len < 2 {
        return unsafe { String::from_utf8_unchecked(s) };
    }

    let mut max_start_index = 0usize;
    let mut max_len = 1_usize;
    let mut temp_len;
    let mut dp = vec![vec![true; len]; len];

    // 如果是从上往下，从左往右地扫，以列为基准的遍历，
    // 外层for循环一定是j而不是i
    for j in 1..len {
        for i in 0..j {
            // TODO 【优化】当子串的长度是2或3时，如果chars[i]==chars[j]那就肯定是回文了
            if s[i] == s[j] && dp[i + 1][j - 1] {
                temp_len = j - i + 1;
                if temp_len > max_len {
                    max_start_index = i;
                    max_len = temp_len;
                }
            } else {
                dp[i][j] = false;
            }
        }
    }
    unsafe { String::from_utf8_unchecked(s[max_start_index..max_start_index + max_len].to_owned()) }
    // let mut result = String::with_capacity(max_len);
    // for i in max_start_index..=max_end_index {
    //     result.push(s[i] as char);
    // }
    // result
}

/*
TODO 建议看Python版本的区间型动态规划
动态规划(数学归纳法)的解法：
递推/状态转移方程：如果a[0]==a[-1]，而且a[1..-2]是个回文数，则a也是个回文数
因为需要原顺序与逆序进行比较，所以可以列出 横坐标是start 纵坐标是end 的表
以 "cbba" 为例，列出一个bool的二维数组：
dp初始值：a[k]==a[k]时为true，表示是回文数
\对角线是dp的初始值，因为竖着的end必须比start大，所以只需遍历左下三角
  c b b a
c T
b ? T
b   % T
a       T
(1)案例分析1 (?处)
例如我想知道end=2, start=0构成的子串cb(如图问号处)是不是回文
先判断a[start] == a[end]，因为不满足条件所以不是，?处填上F
(2)案例分析2 (%处)
%是满足a[start]==a[end]，再看看%的上一项是在%的右上方，属于右上角区域，所以是空字符串也满足回文条件
所以%也是个回文数
因此我们遍历时需要从右边往左，从上到下才能保证每次取右上时能有值
(3)遍历
len = 4;
初始：start=len-2, end=len-1遍历到start+1
start=2, end=3->3;
start=1, end=3->2;
改良：
写完后我才发现start作为纵坐标更合适，刚好能让二位数组的index变为s[start][end]
FIXME 按列从上往下扫，外层for循环是j，内层for循环是i

由于求第N列的值时只需要N-1列的数据，所以定义一个2*N的数组也能满足需求，节约内存
*/
