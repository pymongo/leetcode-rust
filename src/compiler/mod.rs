//! 用「栈」能解 arg: (), left_node_output: (), right_node_output: (), ret: () arg: (), left_node_output: (), right_node_output: (), ret: ()决需要eval数学表达式的运算优先级问题
//! 栈顶元素: 表示下一个乘除运算的lhs
//! 高优先级(乘除运算): 栈顶元素是lhs，直接更新栈顶元素
//! 低优先级(加减运算): 减法的rhs乘以-1转为加法，将加法的rhs入栈，这样栈顶元素就变成刚刚入栈的rhs,最后求和栈，所以加法是最后算的，满足低优先级的需求
mod basic_calculator_ii_no_parentheses;
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
        // 由于要先乘除后加减的运算优先级，所以建议4个一组的进行计算
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
    const TEST_CASES: [(i32, i32); 2] = [
        (4, 7),   // 7 = 4 * 3 / 2 + 1
        (10, 12), // 12 = 10 * 9 / 8 + 7 - （6 * 5 / 4 + 3） - 2 * 1
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(clumsy_factorial(input), output);
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
