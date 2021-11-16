/// https://leetcode.com/problems/basic-calculator-ii/
#[cfg(feature = "rustc_private")]
fn basic_calculator_ii_using_rustc_lexer(expr: String) -> i32 {
    use rustc_lexer::{Base, LiteralKind, TokenKind};
    let mut last_operand = TokenKind::Plus;
    let mut stack = vec![];
    let mut offset = 0;
    for token in rustc_lexer::tokenize(&expr) {
        #[allow(clippy::wildcard_enum_match_arm)]
        match token.kind {
            TokenKind::Whitespace => {}
            TokenKind::Literal {
                kind:
                    LiteralKind::Int {
                        base: Base::Decimal,
                        ..
                    },
                ..
            } => {
                let rhs = expr[offset..offset + token.len].parse::<i32>().unwrap();
                match last_operand {
                    TokenKind::Plus => stack.push(rhs),
                    TokenKind::Minus => stack.push(-rhs),
                    TokenKind::Star => *stack.last_mut().unwrap() *= rhs,
                    TokenKind::Slash => *stack.last_mut().unwrap() /= rhs,
                    _ => {}
                }
            }
            _ => last_operand = token.kind,
        }
        offset += token.len;
    }
    stack.into_iter().sum()
}

/** https://leetcode.com/problems/basic-calculator-ii/
## 需求
做一个 整数的, 加减乘除四则运算的 eval求值函数，输入字符串格式四则运算的表达式，输出整数

## 227_计算器2 和 224_计算器1 的不同点
227更难，四则运算的基础上多了括号的处理
虽然多了括号，但是227只有加减运算，比较可行的解决方案是用两个栈，一个存储运算符号和括号，另一个存储数值

## 与 clumsy_factorial 的不同点
这题比 clumsy_factorial 更难，解析字符串过程中不能同时知道运算符和操作数
需要先存储上一个操作符，遇到下一个操作数后才能进行加减乘除处理
而 clumsy_factorial 这题简单在于同时能知道 运算符和操作数 (因为乘除加减循环来，操作数则是从n-1列举到1)

## tweet

```text
写了个 eval 数学表达式的函数
大体思路就是将运算符分成三个优先级
优先级高: 遇到括号立即递归调用 eval 出括号内的值
优先级中: 遇到乘除立即算(更新栈顶的lhs)
优先级低: 遇到加减先入栈，最后算
不过我还是打了很多断点单步调试几十次，才正确实现 eval 算法(太烧脑了)
```
*/
fn basic_calculator_ii(s: String) -> i32 {
    fn read_a_i32(bytes: &[u8], cursor: &mut usize) -> i32 {
        // assert bytes[cursor] is start index of i32
        let mut number = i32::from(bytes[*cursor] - b'0');
        while let Some(next) = bytes.get(*cursor + 1) {
            if next.is_ascii_digit() {
                number = number * 10 + i32::from(bytes[*cursor + 1] - b'0');
                *cursor += 1;
            } else {
                break;
            }
        }
        *cursor += 1;
        number
    }

    let s = s.into_bytes();
    let mut cursor = 0;
    let len = s.len();

    // trim whitespace from start
    while s[cursor] == b' ' {
        cursor += 1;
    }
    // 先初始化 lhs (输入字符串的第一个整数也就是第一个运算符的左操作数)
    // 这样往后的字符串解析 都是 操作符+右操作数成对出现
    // 例如"3+2*2"，先初始化 stack=vec![3]，然后往后顺序解析出两个操作符+操作数的对子 ('+','2') 和 ('*','2')
    let mut stack = vec![read_a_i32(&s, &mut cursor)];

    let mut operator = b'\0';
    while cursor < len {
        match s[cursor] {
            b'0'..=b'9' => {
                let rhs = read_a_i32(&s, &mut cursor);
                match operator {
                    b'+' => stack.push(rhs),
                    b'-' => stack.push(-rhs),
                    b'*' => *stack.last_mut().unwrap() *= rhs,
                    b'/' => *stack.last_mut().unwrap() /= rhs,
                    _ => unreachable!(),
                }
                // cursor 已经在 read_a_i32 中修改过了，所以直接 continue 避免执行后面的`cursor += 1`
                continue;
            }
            b'+' => operator = b'+',
            b'-' => operator = b'-',
            b'*' => operator = b'*',
            b'/' => operator = b'/',
            _ => {}
        }
        cursor += 1;
    }
    stack.into_iter().sum()
}

#[test]
fn test_basic_calculator_ii() {
    const TEST_CASES: [(&str, i32); 4] = [
        ("3+2*2", 7),
        (" 3/2 ", 1),
        (" 3+5 / 2 ", 5),
        ("99 - 101", -2),
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(basic_calculator_ii(input.to_string()), output);
    }
}
