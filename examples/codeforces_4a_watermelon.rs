/*! https://codeforces.com/problemset/problem/4/A
注意在IDEA上发送EOF的快捷键是cmd+d，本题的测试用例1是 8+回车+EOF

本题问的是一个数能否被分成两个偶数之和，显然只要不等于2的所有偶数都符合条件
*/
use std::io::Read;

fn main() {
    // max input len testcase is "100\n"
    let mut input = String::with_capacity(4);
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    // pop last '\n' char
    input.pop();
    // The first (and the only) input line contains integer number w (1≤w≤100)
    let num = input.parse::<u8>().unwrap();
    if num != 2 && num % 2 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
