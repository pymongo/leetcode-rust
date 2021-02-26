/*!
## codeforces的stdin/stdout注意事项

1. 不支持dbg!宏，dbg!编译不报错但是不会显示到stdout，意味着无法通过dbg!去debug (leetcode也不支持dbg!)
2. stdin每行的分隔符是CR+LF两个byte，例如本题的测试用例8在stdin上为: [56,13,10]+EOF
3. Rust目前没有Python的os.linesep可以获取当前操作系统的换行符，不利于单元测试的跨平台

但是在mac上(terminal或IDEA)，stdin输入回车只有LF，不是codeforces的CRLF，不方便模拟codeforces上运行代码的真实环境
mac/linux: [49,48,48,13(CR),10(LF)]
codeforces: [49,48,48,10(LF)]

注意在IDEA上发送EOF的快捷键是cmd+d
*/

/// https://codeforces.com/problemset/problem/4/A
fn cf_1a_theatre_square(
    mut reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let input_split: Vec<u32> = input
        .trim_end()
        .split_whitespace()
        .map(|each| each.parse::<u32>().unwrap())
        .collect();
    let (n, m, a) = (input_split[0], input_split[1], input_split[2]);
    let num_width = (n / a) as u64 + (n % a != 0) as u64; // divmod, ceil
    let num_length = (m / a) as u64 + (m % a != 0) as u64;
    write!(&mut writer, "{}", num_width * num_length)?;
    Ok(())
}

#[test]
fn test_cf_1a_theatre_square() {
    const TEST_CASES: [(&[u8], &[u8]); 1] = [(b"6 6 4\n", b"4")];
    for &(input, expected_output) in &TEST_CASES {
        let mut output = Vec::new();
        cf_1a_theatre_square(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

/// https://codeforces.com/problemset/problem/4/A
/// 本题问的是一个数能否被分成两个偶数之和，显然只要不等于2的所有偶数都符合条件
fn cf_4a_watermelon<R, W>(mut reader: R, mut writer: W) -> Result<(), Box<dyn std::error::Error>>
where
    R: std::io::Read,
    W: std::io::Write,
{
    // max input len testcase is "100\r\n"([49,48,48,13(CR),10(LF)])
    let mut read_buffer = [0u8; 5];
    let _read_size = reader.read(&mut read_buffer)?;
    let mut num = 0u8;
    for &byte in read_buffer.iter() {
        if byte == b'\r' || byte == b'\n' {
            break;
        }
        num = num * 10 + byte - b'0';
    }
    if num % 2 == 0 && num != 2 {
        write!(&mut writer, "YES")?;
    } else {
        write!(&mut writer, "NO")?;
    };
    Ok(())
}

#[test]
fn test_cf_4a_watermelon() {
    const TEST_CASES: [(&[u8], &[u8]); 2] = [
        (b"8\r\n", b"YES"), // codeforces testcase_1
        (b"99\n", b"NO"),   // mac_os input(without CR byte)
    ];

    for &(input, expected_output) in &TEST_CASES {
        let mut output = Vec::new();
        cf_4a_watermelon(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

/** https://codeforces.com/problemset/problem/71/A
在英语中有一种将很长的单词缩写方法是: 首字母+中间有几个字母+尾字母
例如 kubernetes 开头k结尾s，k和s中间有8个字母，所以缩写成k8s
例如 internationalization 开头i结尾n中间18个字母，缩写成i18n
*/
fn cf_71a_way_too_long_words(
    reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines().flatten() {
        input.push(line);
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

#[test]
fn test_cf_71a_way_too_long_words() {
    const TEST_CASES: [(&[u8], &[u8]); 1] = [(
        b"4\nword\nlocalization\ninternationalization\npneumonoultramicroscopicsilicovolcanoconiosis",
        b"word\nl10n\ni18n\np43s\n",
    )];
    for &(input, expected_output) in &TEST_CASES {
        let mut output = Vec::new();
        cf_71a_way_too_long_words(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

/// https://codeforces.com/problemset/problem/231/A
fn cf_231a_team(
    reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines().flatten() {
        input.push(line);
    }
    let mut ret = 0u16;
    for each in input.into_iter().skip(1) {
        let each = each.into_bytes();
        let num_sure_about_the_solution = each[0] - b'0' + each[2] - b'0' + each[4] - b'0';
        if num_sure_about_the_solution >= 2 {
            ret += 1;
        }
    }
    write!(&mut writer, "{}", ret)?;
    Ok(())
}

#[test]
fn test_cf_231a_team() {
    const TEST_CASES: [(&[u8], &[u8]); 1] = [(b"2\n1 0 0\n0 1 1\n", b"1")];
    for &(input, expected_output) in &TEST_CASES {
        let mut output = Vec::new();
        cf_231a_team(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

fn main() {
    cf_1a_theatre_square(std::io::stdin().lock(), std::io::stdout().lock()).unwrap();
}
