struct TestCase {
    input: Vec<String>,
    output: Vec<String>
}

impl TestCase {
    fn new(input: &[&str], output: &[&str]) -> Self {
        Self {
            input: input.iter().map(|&s| s.to_string()).collect(),
            output: output.iter().map(|&s| s.to_string()).collect()
        }
    }
}

fn get_test_case() -> Vec<TestCase> {
    vec![
        // e和l都在所有单词中出现，l在所有单词中至少出现了2次，所以返回e,l,l
        TestCase::new(&["bella", "label", "roller"], &["e", "l", "l"]),
        TestCase::new(&["cool", "lock", "cook"], &["c", "o"]),
    ]
}

#[test]
fn test() {
    for test_case in get_test_case() {
        assert_eq!(Solution::common_chars(test_case.input), test_case.output);
    }
}

struct Solution;
const CHAR_A: u8 = b'a';

impl Solution {
    /**
    由于输入仅由小写字母组成，很容易就想到用ASCII码的数组
    为了命中CPU缓存，要每行表示一个小写字母，26行n列的矩阵，才能在计数完每个单词的字母出现次数后，逐行扫描统计字母出现次数的至少值
    */
    fn common_chars(a: Vec<String>) -> Vec<String> {
        let n = a.len();
        let mut arr = vec![vec![0u8; n]; 26];
        for word in 0..n {
            for &c in a[word].as_bytes() {
                arr[(c-CHAR_A) as usize][word] += 1;
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
            let letter_char = (letter as u8 + CHAR_A) as char;
            for _ in 0..common_occur_times {
                res.push(letter_char.to_string());
            }
        }

        res
    }
}
