/*! https://codeforces.com/problemset/problem/71/A
在英语中通常会将很长的单词进行缩写，其中有一种缩写方法是: 首字母+中间有几个字母+尾字母
例如 kubernetes 开头字母是k，结尾字母是s，k和s中间有8个字母，所以缩写成k8s
例如 internationalization 开头字母i和结尾字母n中间有18个字母，所以缩写成i18n
*/

fn solution(
    reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(str) = line {
            input.push(str);
        }
    }
    for string in input.into_iter().skip(1) {
        let len = string.len();
        if len <= 10 {
            writeln!(&mut writer, "{}", string)?;
        } else {
            let bytes = string.into_bytes();
            writeln!(
                &mut writer,
                "{}{}{}",
                bytes[0] as char,
                len - 2, // len - 2(first and last)
                *bytes.last().unwrap() as char
            )?;
        }
    }
    Ok(())
}

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout().lock()).unwrap();
}

#[cfg(test)]
const TEST_CASES: [(&[u8], &[u8]); 1] = [(
    b"4\nword\nlocalization\ninternationalization\npneumonoultramicroscopicsilicovolcanoconiosis",
    b"word\nl10n\ni18n\np43s\n",
)];

#[test]
fn test_solution() {
    for &(input, expected_output) in &TEST_CASES {
        let mut output = Vec::new();
        solution(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}
