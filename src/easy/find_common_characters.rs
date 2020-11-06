//! https://leetcode.com/problems/find-common-characters/
struct Solution;

impl Solution {
    /**
    由于输入仅由小写字母组成，很容易就想到用ASCII码的数组
    为了命中CPU缓存，要每行表示一个小写字母，26行n列的矩阵，才能在计数完每个单词的字母出现次数后，逐行扫描统计字母出现次数的至少值
    */
    fn common_chars(a: Vec<String>) -> Vec<String> {
        let n = a.len();
        let mut arr = vec![vec![0u8; n]; 26];
        for word in 0..n {
            for c in a[word].as_bytes() {
                arr[(c - b'a') as usize][word] += 1;
            }
        }

        let mut res = Vec::new();
        'outer: for letter in 0..26usize {
            let mut common_occur_times = 0;
            for word in 0..n {
                let letter_occur_times = arr[letter][word];
                if letter_occur_times == 0 {
                    continue 'outer;
                }
                if common_occur_times == 0 {
                    common_occur_times = letter_occur_times;
                } else {
                    common_occur_times = common_occur_times.min(letter_occur_times);
                }
            }
            let letter_char = (letter as u8 + b'a') as char;
            for _ in 0..common_occur_times {
                res.push(letter_char.to_string());
            }
        }

        res
    }
}

#[cfg(test)]
const TESTCASES: [(&[&str], &[&str]); 2] = [
    (&["bella", "label", "roller"], &["e", "l", "l"]),
    (&["cool", "lock", "cook"], &["c", "o"]),
];

#[test]
fn test() {
    for &(input, output) in &TESTCASES {
        let input: Vec<String> = input.iter().map(ToString::to_string).collect();
        let output: Vec<String> = output.iter().map(ToString::to_string).collect();
        assert_eq!(Solution::common_chars(input), output);
    }
}
