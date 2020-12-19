/** https://leetcode.com/problems/find-common-characters/
需求: 求出输入Vec<String>中在所有String中均出现过的字母
由于输入仅由小写字母组成，很容易就想到用ASCII码的数组
为了命中CPU缓存，要每行表示一个小写字母，26行n列的矩阵，才能在计数完每个单词的字母出现次数后，逐行扫描统计字母出现次数的至少值
*/
#[allow(clippy::needless_range_loop)]
fn find_common_chars(a: Vec<String>) -> Vec<String> {
    let n = a.len();
    let mut arr = vec![vec![0u8; n]; 26];
    for word in 0..n {
        for c in a[word].as_bytes() {
            arr[(c - b'a') as usize][word] += 1;
        }
    }

    let mut ret = Vec::new();
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
            ret.push(letter_char.to_string());
        }
    }

    ret
}

#[test]
fn test_find_common_chars() {
    const TEST_CASES: [(&[&str], &[&str]); 2] = [
        (&["bella", "label", "roller"], &["e", "l", "l"]),
        (&["cool", "lock", "cook"], &["c", "o"]),
    ];

    for &(input, output) in &TEST_CASES {
        let input: Vec<String> = input.iter().map(ToString::to_string).collect();
        let output: Vec<String> = output.iter().map(ToString::to_string).collect();
        assert_eq!(find_common_chars(input), output);
    }
}
