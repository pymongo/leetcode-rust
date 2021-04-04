/*!
# 编译型语言和脚本语言在负数计算上的差异

## -1 / 10
- Python/Ruby: -1
- C/C++/Java/Rust: 0
test:
ruby -e "p -1 / 10"
root -e "-1 / 10"

## -1 % 10
Python/Ruby: 9
C/C++/Java/Rust: -1
test:
ruby -e "p -1 % 10"
root -e "-1 % 10"
*/
