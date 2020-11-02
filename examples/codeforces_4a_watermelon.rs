/*! https://codeforces.com/problemset/problem/4/A

注意在IDEA上发送EOF的快捷键是cmd+d

## codeforces的stdin/stdout注意事项

1. 不支持dbg!宏，dbg!编译不报错但是不会显示到stdout，意味着无法通过dbg!去debug
2. stdin每行的分隔符是CR+LF两个byte，例如本题的测试用例8在stdin上为: [56,13,10]+EOF

但是在mac上(terminal或IDEA)，stdin输入回车只有LF，不是codeforces的CRLF，不方便模拟和测试
mac: [49,48,48,13(CR),10(LF)]
codeforces: [49,48,48,10(LF)]

本题问的是一个数能否被分成两个偶数之和，显然只要不等于2的所有偶数都符合条件
The first (and the only) input line contains integer number w (1≤w≤100)
*/
use std::io::Read;

fn main() {
    // max input len testcase is "100\r\n"([49,48,48,13(CR),10(LF)])
    let mut input = [0u8; 5];
    let _input_size = std::io::stdin().lock().read(&mut input).unwrap();
    let mut num = 0u8;
    // for in in 0..input_size-2
    for i in 0..5 {
        if input[i] == b'\r' || input[i] == b'\n' {
            break;
        }
        num = num * 10 + input[i] - b'0';
    }
    // println!("input: {:?}, num: {}, input_size: {}", input, num, input_size);
    if num % 2 == 0 && num != 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
