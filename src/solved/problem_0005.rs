// 国服第一0ms的解答，Manacher是唯一能实现O(n)时间复杂度的算法
#[cfg(not)]
pub fn longest_palindrome_manacher(s: String) -> String {
    /* Manacher Algorithm
     * step1: add '$''#' into string
     * The character `$` here is just to prevent overbounds
     * there is an even palindrome `abba` and an odd palindrome `opxpo` in `s="abbahopxpo"`,
     * which are converted into `#a#b#b#a#` and `#o#p#x#p#o#`, and the length is converted into odd
     */
    let mut new_str = vec!['$', '#'];
    for ch in s.chars() {
        new_str.push(ch);
        new_str.push('#');
    }
    new_str.push('\0');

    // length of the new string
    let len = new_str.len();

    // Define a secondary array p[], where p[i] represents the radius of the longest palindrome centered on i.
    let mut p = vec![0usize; len];

    // `max_len`: The length of the longest palindrome string in the original string
    let mut max_len = 0usize;

    // Define two variables, `mx` and` id`
    // `mx` represents the right boundary of the longest palindrome centered on` id`, which is `mx = id + p[id]`
    let mut id = 0usize;
    let mut mx = 0usize;
    for i in 1..(len - 1) {
        if i < mx {
            p[i] = p[2 * id - i].min(mx - i);
        } else {
            p[i] = 1;
        }

        while new_str[i - p[i]] == new_str[i + p[i]] {
            p[i] += 1;
        }

        if mx < i + p[i] {
            id = i;
            mx = i + p[i];
        }
        // `p[i] - 1` is exactly the length of the longest palindrome string in the original string
        max_len = max_len.max(p[i] - 1);
    }

    /* Get longest palindromic substring
     * left: left boundery of the longest palindromic substring
     * right: right boundery of the longest palindromic substring
     */
    let left = p.iter().position(|&x| x == (max_len + 1)).unwrap() - max_len + 1;
    let right = left + max_len * 2;
    let mut longest_palindrome_substring = String::from("");
    for i in left..right {
        if new_str[i] != '#' {
            longest_palindrome_substring.push(new_str[i]);
        }
    }

    /* Return longest palindromic substring */
    longest_palindrome_substring
}

// 全球服第一的答案，似乎并不是Manacher算法
#[cfg(features = "unused")]
pub fn longest_palindrome(s: String) -> String {
    let seq: Vec<char> = s.chars().collect();
    let len = seq.len();
    if len < 1 { return s; }
    let (mut idx, mut curr_len, mut curr_start, mut curr_end) = (0, 0, 0, 0);
    while idx < len {
        let (mut i, mut j) = (idx, idx);
        let ch = seq[idx];
        // handle same char
        while i > 0 && seq[i - 1] == ch { i -= 1 };
        while j < len - 1 && seq[j + 1] == ch { j += 1 };
        idx = j + 1;
        while i > 0 && j < len - 1 && seq[i - 1] == seq[j + 1] {
            i -= 1;
            j += 1;
        }
        let max_len = j - i + 1;
        if max_len > curr_len {
            curr_len = max_len;
            curr_start = i;
            curr_end = j;
        }
        if max_len >= len - 1 {
            break;
        }
    }

    s[curr_start..curr_end + 1].to_owned()
}

// 这老哥用paris存储start/end的组合也是挺有意思的，不过我还是习惯使用left/right
#[cfg(not)]
pub fn longest_palindrome_best_brute_force(s: String) -> String {
    if s.as_str() { return s; }
    let s: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut pair = (0, 0);
    while i < (s.len() - 1) {
        let mut j;
        let mut k;
        if s[i] == s[i + 1] {
            j = i;
            k = i + 1;
            while j > 0 && k < s.len() - 1 && s[j] == s[k] {
                j -= 1;
                k += 1;
            }
            if s[j] != s[k] {
                j += 1;
                k -= 1;
            }
            if k - j > pair.1 - pair.0 {
                pair.0 = j;
                pair.1 = k;
            }
        }
        if i > 0 && s[i - 1] == s[i + 1] {
            j = i - 1;
            k = i + 1;
            while j > 0 && k < s.len() - 1 && s[j] == s[k] {
                j -= 1;
                k += 1;
            }
            if s[j] != s[k] {
                j += 1;
                k -= 1;
            }
            if k - j > pair.1 - pair.0 {
                pair.0 = j;
                pair.1 = k;
            }
        }
        i += 1;
    }
    s[pair.0..pair.1 + 1].iter().collect()
}