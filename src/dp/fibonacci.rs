/** https://leetcode.com/problems/climbing-stairs/
& https://leetcode.com/problems/fibonacci-number/
虽然斐波那契数超简单的dp，但是作为经典题值得单独整理成一个文件
比尾递归O(n)更快的还有O(log(n))的解法：
1. 斐波那契公式(公式中的乘方需要log(n)时间复杂度)
2. Binet's formula 利用矩阵解斐波那契
*/
const fn fib_recursive(n: i32, a: i32, b: i32) -> i32 {
    if n == 1 {
        b
    } else {
        // 注意尾递归解法只能从1逼近n，普通递归解法一般是从f(n-1)一直加到f(1)
        fib_recursive(n - 1, b, a + b)
    }
}

const fn fib_iterative(mut n: i32) -> i32 {
    let (mut a, mut b) = (0_i32, 1_i32);
    while n >= 1 {
        let temp = b;
        b += a;
        a = temp;
        n -= 1;
    }
    a
}

fn fib_reduce(n: i32) -> i32 {
    (0..n)
        .fold((0, 1), |accum, _each| (accum.1, accum.0 + accum.1))
        .0
}

/// eval expression need to know how stack machine work
fn fib_stack_machine(n: i32) -> i32 {
    #[derive(Debug)]
    struct FibStackFrame {
        n: i32,
        step: u8,
    }
    impl FibStackFrame {
        const fn new(n: i32, step: u8) -> Self {
            Self { n, step }
        }
    }

    let mut stack: Vec<FibStackFrame> = vec![FibStackFrame::new(n, 0)];
    // for easy, we assert all stack frame return addr in same
    let mut ans = 0;
    while let Some(frame) = stack.pop() {
        if frame.n == 0 || frame.n == 1 {
            ans += 1;
            continue;
        }
        match frame.step {
            0 => {
                stack.push(FibStackFrame::new(frame.n, 1));
                // f(n-1)
                stack.push(FibStackFrame::new(frame.n - 1, 0));
            }
            1 => {
                stack.push(FibStackFrame::new(frame.n, 2));
                // f(n-2)
                stack.push(FibStackFrame::new(frame.n - 2, 0));
            }
            2 => {
                // f(n-1)+f(n-2) calc ok, do nothing
            }
            _ => unreachable!(),
        }
    }

    ans
}

#[test]
fn test_fib() {
    assert_eq!(fib_stack_machine(4), 5);
}
