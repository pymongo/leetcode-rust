/*! https://codeforces.com/problemset/problem/4/A

注意在IDEA上发送EOF的快捷键是cmd+d

## codeforces的stdin/stdout注意事项

1. 不支持dbg!宏，dbg!编译不报错但是不会显示到stdout，意味着无法通过dbg!去debug
2. stdin每行的分隔符是CR+LF两个byte，例如本题的测试用例8在stdin上为: [56,13,10]+EOF

但是在mac上(terminal或IDEA)，stdin输入回车只有LF，不是codeforces的CRLF，不方便模拟和测试
mac/linux: [49,48,48,13(CR),10(LF)]
codeforces: [49,48,48,10(LF)]

本题问的是一个数能否被分成两个偶数之和，显然只要不等于2的所有偶数都符合条件
The first (and the only) input line contains integer number w (1≤w≤100)

首次用Rust解了codeforces的两题，主要困难是codeforces的stdin用的是CRLF换行符，mac系统上是LF换行符，
而且codeforces不支持Rust的dbg!宏去调试代码。

好在Rust的单元测试可以模拟stdin和stdout，通过泛型和trait让stdin,array,vector等抽象成一个函数去复用(如图1红框所示)

*/
fn solution<R, W>(mut reader: R, mut writer: W) -> Result<(), Box<dyn std::error::Error>>
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

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout().lock()).unwrap();
}

#[cfg(test)]
const TESTCASES: [(&[u8], &[u8]); 2] = [
    (b"8\r\n", b"YES"), // codeforces testcase_1
    (b"99\n", b"NO"),   // mac_os input(without CR byte)
];

#[test]
fn test_solution() {
    for &(input, expected_output) in &TESTCASES {
        let mut output = Vec::new();
        solution(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}
