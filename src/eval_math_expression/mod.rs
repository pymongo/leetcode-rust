//! 用「栈」能解决需要eval数学表达式的运算优先级问题
//! 栈顶元素: 表示下一个乘除运算的lhs
//! 高优先级(乘除运算): 栈顶元素是lhs，直接更新栈顶元素
//! 低优先级(加减运算): 减法的rhs乘以-1转为加法，将加法的rhs入栈，这样栈顶元素就变成刚刚入栈的rhs,最后求和栈，所以加法是最后算的，满足低优先级的需求

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
    let mut operator_flag = 0b0000_0001u8;
    for operand in (1..=n - 1).rev() {
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
    for &(input, output) in TEST_CASES.iter() {
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
*/
fn eval_int_math_expression(s: String) -> i32 {
    fn read_a_i32(bytes: &[u8], cursor: &mut usize) -> i32 {
        // assert bytes[cursor] is start index of i32
        let mut int_val = (bytes[*cursor] - b'0') as i32;
        while let Some(next) = bytes.get(*cursor + 1) {
            if next.is_ascii_digit() {
                int_val = int_val * 10 + (bytes[*cursor + 1] - b'0') as i32;
                *cursor += 1;
            } else {
                break;
            }
        }
        *cursor += 1;
        int_val
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
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(eval_int_math_expression(input.to_string()), output);
    }
}

enum StackVal {
    Int(i32),
    LeftParenthesis,
    RightParenthesis,
}

impl StackVal {
    fn unwrap(self) -> i32 {
        if let Self::Int(int) = self {
            int
        } else {
            panic!("StackVal is not a Int")
        }
    }
}

impl std::ops::MulAssign<i32> for StackVal {
    fn mul_assign(&mut self, rhs: i32) {
        if let Self::Int(int) = self {
            *int *= rhs;
        } else {
            panic!("StackVal is not a Int");
        }
    }
}

impl std::ops::DivAssign<i32> for StackVal {
    fn div_assign(&mut self, rhs: i32) {
        if let Self::Int(int) = self {
            *int /= rhs;
        } else {
            panic!("StackVal is not a Int");
        }
    }
}

fn eval_int_with_parentheses(s: String) -> i32 {
    fn read_a_i32(bytes: &[u8], cursor: &mut usize) -> i32 {
        let mut int_val = (bytes[*cursor] - b'0') as i32;
        while let Some(next) = bytes.get(*cursor + 1) {
            if next.is_ascii_digit() {
                int_val = int_val * 10 + (bytes[*cursor + 1] - b'0') as i32;
                *cursor += 1;
            } else {
                break;
            }
        }
        *cursor += 1;
        int_val
    }

    let s = s.into_bytes();
    let mut cursor = 0;
    let len = s.len();

    let mut stack: Vec<StackVal> = vec![];
    let mut operator = b'+'; // 默认运算符是加，这样遇到第一个数字时会存入stack作为lhs

    while cursor < len {
        match s[cursor] {
            b'0'..=b'9' => {
                let rhs = read_a_i32(&s, &mut cursor);
                match operator {
                    b'+' => stack.push(StackVal::Int(rhs)),
                    b'-' => stack.push(StackVal::Int(-rhs)),
                    b'*' => *stack.last_mut().unwrap() *= rhs,
                    b'/' => *stack.last_mut().unwrap() /= rhs,
                    _ => unreachable!(),
                }
                continue;
            }
            b'+' => operator = b'+',
            b'-' => operator = b'-',
            b'*' => operator = b'*',
            b'/' => operator = b'/',
            b'(' => stack.push(StackVal::LeftParenthesis),
            b')' => {
                // 遇到一对括号，则提前对括号内的stack进行求值
                let mut new_peek = 0;
                while let Some(pop) = stack.pop() {
                    match pop {
                        StackVal::Int(int) => new_peek += int,
                        StackVal::LeftParenthesis => break,
                        StackVal::RightParenthesis => unreachable!(),
                    }
                }
                stack.push(StackVal::Int(new_peek));
            }
            _ => {}
        }
        cursor += 1;
    }
    stack.into_iter().map(|x| x.unwrap()).sum()
}

#[test]
fn test_eval_int_with_parentheses() {
    const TEST_CASES: [(&str, i32); 5] = [
        ("1 + 1", 2),
        (" 2-1 + 2 ", 3),
        ("(1+(4+5+2)-3)+(6+8)", 23),
        ("- (3 + (4 + 5))", -12),
        ("-((1+2*3)*(3-2/1))", -7),
    ];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(eval_int_with_parentheses(input.to_string()), output);
    }
}
