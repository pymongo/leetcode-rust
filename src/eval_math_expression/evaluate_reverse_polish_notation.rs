/// https://leetcode.com/problems/evaluate-reverse-polish-notation/
/// 类似题型: leetcode_682
fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens {
        let ch = token.as_bytes()[0];
        //if ch.is_ascii_digit() || (ch == b'-' && token.len() > 1) {
        if ch.is_ascii_digit() || token.len() > 1 {
            stack.push(token.parse().unwrap());
        } else {
            let rhs = stack.pop().unwrap();
            let lhs = stack.last_mut().unwrap();
            match ch {
                b'+' => *lhs += rhs,
                b'-' => *lhs -= rhs,
                b'*' => *lhs *= rhs,
                b'/' => *lhs /= rhs,
                _ => unreachable!(),
            }
        }
    }
    stack[0]
}

/*
解释测试用例(&["4","13","5","/","+"], 6)
(4 + (13 / 5)) = 6
stack = [4, 13, 5]
ch = "/"
rhs = stack.pop()
lhs = stack.peek()
lhs = lhs / rhs
*/
#[test]
fn test_eval_rpn() {
    const TEST_CASES: [(&[&str], i32); 3] = [
        (&["2", "1", "+", "3", "*"], 9),
        (&["4", "13", "5", "/", "+"], 6),
        (
            &[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
            ],
            22,
        ),
    ];
    for (input, output) in TEST_CASES {
        let tokens = input.iter().map(|&x| x.to_string()).collect();
        assert_eq!(eval_rpn(tokens), output);
    }
}
