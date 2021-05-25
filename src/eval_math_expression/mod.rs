//! 用「栈」能解 arg: (), left_node_output: (), right_node_output: (), ret: () arg: (), left_node_output: (), right_node_output: (), ret: ()决需要eval数学表达式的运算优先级问题
//! 栈顶元素: 表示下一个乘除运算的lhs
//! 高优先级(乘除运算): 栈顶元素是lhs，直接更新栈顶元素
//! 低优先级(加减运算): 减法的rhs乘以-1转为加法，将加法的rhs入栈，这样栈顶元素就变成刚刚入栈的rhs,最后求和栈，所以加法是最后算的，满足低优先级的需求
mod evaluate_reverse_polish_notation;

/// https://leetcode.com/problems/clumsy-factorial/
/// 由于乘除优先计算，所以: 遇到乘除立即算(更新栈顶元素)，遇到加减先入栈
/// 类似 乘除更新栈顶，加减直接入栈，返回值是栈元素累加 这种 eval 计算表达式的还有: 150, 224, 227等题
fn clumsy_factorial(n: i32) -> i32 {
    /*
    const OPERATORS: [fn(i32, i32) -> i32; 4] = [
        i32::saturating_mul,
        i32::div_euclid,
        i32::saturating_add,
        i32::saturating_sub
    ];
    */
    let mut stack = vec![n];
    let mut operator_flag = 0b0000_0001_u8;
    for operand in (1..n).rev() {
        match operator_flag {
            0b0000_0001 => *stack.last_mut().unwrap() *= operand,
            0b0000_0100 => *stack.last_mut().unwrap() /= operand,
            0b0001_0000 => stack.push(operand),
            0b0100_0000 => stack.push(-operand),
            _ => unreachable!(),
        }
        operator_flag = operator_flag.rotate_left(2);
    }
    stack.into_iter().sum()
}

#[test]
fn test_clumsy_factorial() {
    // 由于要先乘除后加减的运算优先级，所以建议4个一组的进行计算
    const TEST_CASES: [(i32, i32); 2] = [
        (4, 7),   // 7 = 4 * 3 / 2 + 1
        (10, 12), // 12 = 10 * 9 / 8 + 7 - （6 * 5 / 4 + 3） - 2 * 1
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(clumsy_factorial(input), output);
    }
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
fn eval_int_math_expression(s: String) -> i32 {
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
fn test_eval_int_math_expression() {
    const TEST_CASES: [(&str, i32); 4] = [
        ("3+2*2", 7),
        (" 3/2 ", 1),
        (" 3+5 / 2 ", 5),
        ("99 - 101", -2),
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(eval_int_math_expression(input.to_string()), output);
    }
}

/// https://leetcode.com/problems/basic-calculator/
struct Helper {
    s: Vec<u8>,
    len: usize,
    cursor: usize,
}

impl Helper {
    /// 递归结束条件: 遇到右括号 或 cursor遍历完字符串s
    /// 允许`-()`的表达式但是不允许`*()`的表达式，乘除必须要有lhs
    /// 参考题解: https://leetcode-cn.com/problems/basic-calculator/solution/jia-jian-cheng-chu-gua-hao-yun-suan-by-w-ldll/
    /// 优先级高: 遇到左括号就递归eval括号内的值，优先级中: 遇到乘除立即算，优先级低: 加减先入栈最后算
    /// 我这里只用了current_operator和last_operator，数字和运算符分别存入两个栈也是可行的
    fn helper(&mut self) -> i32 {
        let mut stack = vec![];
        // last_operator in ['(', '*', '/', '+', '-']
        let mut last_operator = b'+';
        let mut rhs = 0;
        while self.cursor < self.len {
            let ch = self.s[self.cursor];
            // 一定要让游标先动，这样才能让下个match的'('分支进入递归时能让游标指向'('的后一个字符))
            self.cursor += 1;
            match ch {
                b' ' => continue,
                b'0'..=b'9' => {
                    // 小心处理这两种测试用例时，字符串最后一个数字没有参与运算 (" 2-1 + 2 ", 3), ("1 + 1", 2)
                    rhs = rhs * 10 + i32::from(ch - b'0');
                    continue;
                }
                b'(' => rhs = self.helper(),
                _ => {}
            }
            // ch in ['(', ')', '*', '/', '+', '-']
            match last_operator {
                b'+' => stack.push(rhs),
                b'-' => stack.push(-rhs),
                b'*' => *stack.last_mut().unwrap() *= rhs,
                b'/' => *stack.last_mut().unwrap() /= rhs,
                b'(' => {}
                _ => unreachable!(),
            }
            if ch == b')' {
                return stack.into_iter().sum();
            }
            rhs = 0;
            last_operator = ch;
        }
        // test_cases: "1 + 1" and " 2-1 + 2 "
        if rhs != 0 {
            match last_operator {
                b'+' => stack.push(rhs),
                b'-' => stack.push(-rhs),
                b'*' => *stack.last_mut().unwrap() *= rhs,
                b'/' => *stack.last_mut().unwrap() /= rhs,
                _ => unreachable!(),
            }
        }
        stack.into_iter().sum()
    }
}

/// https://leetcode.com/problems/basic-calculator/
fn eval_int_with_parentheses(s: String) -> i32 {
    let s = s.into_bytes();
    let len = s.len();
    let mut helper = Helper { s, len, cursor: 0 };
    helper.helper()
}

#[test]
fn test_eval_int_with_parentheses() {
    const TEST_CASES: [(&str, i32); 7] = [
        ("-1", -1),
        ("1", 1),
        (" 2-1 + 2 ", 3),
        ("1 + 1", 2),
        ("(1+(4+5+2)-3)+(6+8)", 23),
        ("- (3 + (4 + 5))", -12),
        ("-((1*2+3)*(3-2/1))", -5),
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(eval_int_with_parentheses(input.to_string()), output);
    }
}

/*
fn fib(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    int tmp = fib(n-1);
    tmp += fin(n-2);

    return tmp;
    return fib(n-1) + fib(n-2);
}

fn fib3(a: i32, b: i32, n: i32) -> i32 {
    return fib3(b, a+b, n-1);
}

#[test]
fn a_s() {
    fib(3);
}

struct FibStackFrame {
    arg_n: i32,
    step: i32,
    return_val: i32
}

struct A<'a> {
    arg: i32,
    // f(n-1)
    left_node_output: i32,
    // f(n-2)
    right_node_output: i32,
    ret: &'a mut i32,
}

fn a(n: i32) -> i32 {
    let mut ret = 0;
    let mut stack = vec![A {
        arg: n,
        left_node_output: 0,
        right_node_output: 0,
        ret: &mut ret
    }];
    while let Some(cur_stack_frame) = stack.pop() {
        if cur_stack_frame.arg == 0 || cur_stack_frame.arg == 1 {
            *cur_stack_frame.ret = 1;
        } else {

        }
        cur_stack_frame.left_node_output = todo!();
        cur_stack_frame.right_node_output = todo!();
    }
    ret
}

fn f(n: i32) -> i32 {
    let mut stack = vec![];
    stack.push(FibStackFrame {
        arg_n: n,
        step: 1,
        return_val: 0
    });
    while let Some(mut item) = stack.pop() {
        if item.arg_n == 0 || item.arg_n == 1 {
            item.return_val = 1;
            stack.last_mut().unwrap().return_val += item.return_val;
            continue;
        }
        match item.step {
            1 => {
                stack.push(FibStackFrame {
                    arg_n: n,
                    step: 2,
                    return_val: 0,
                });
                // f(n-1)
                stack.push(FibStackFrame {
                    arg_n: n-1,
                    step: 1,
                    return_val: 0,
                });
            },
            2 => {
                stack.push(FibStackFrame {
                    arg_n: n,
                    step: 3,
                    return_val: 0,
                });
                stack.push(FibStackFrame {
                    arg_n: n-1-1,
                    step: 1,
                    return_val: 0,
                });
            },
            3 => {
                if stack.is_empty() {
                    return item.return_val;
                } else {
                    stack.last_mut().unwrap().return_val += item.return_val;
                }
            }
            _ => unreachable!()
        }
    }
    unreachable!()
}

*/
