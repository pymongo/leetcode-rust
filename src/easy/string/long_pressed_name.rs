struct Solution;

impl Solution {
    /**
    官方解答: 直接双指针
    ```go
    func isLongPressedName(name string, typed string) bool {
        i, j := 0, 0
        for j < len(typed) {
            if i < len(name) && name[i] == typed[j] {
                // 作为name的一部分。此时会「匹配」name中的一个字符
                i++
                j++
            } else if j > 0 && typed[j] == typed[j-1] {
                // 作为长按键入的一部分，此时它应当与前一个字符相同
                j++
            } else {
                return false
            }
        }
        return i == len(name)
    }
    ```
    */
    fn official_solution(name: String, typed: String) -> bool {
        let (mut i, mut j) = (0, 0);
        let (name, typed) = (name.into_bytes(), typed.into_bytes());
        let (name_len, typed_len) = (name.len(), typed.len());
        while j < typed_len {
            #[allow(clippy::suspicious_operation_groupings)]
            if i < name_len && name[i] == typed[j] {
                i += 1;
                j += 1;
            } else if j > 0 && typed[j] == typed[j - 1] {
                j += 1;
            } else {
                return false;
            }
        }
        i == name_len
    }

    /**
    虽然解法比较繁琐丑陋，但是学到了peekableIterator，复习下迭代器的种类
    - peekable(peek, next_if)
    - take_while(类似peekable迭代器的next_if，能接收过滤条件的迭代器)
    - skip_while(和take_while相反，跳过满足过滤条件的元素)
    - scan(类似fold/reduce)
    - cycle(循环)
    - fuse(遇到第一个None就停掉)

    迭代器好用的API: inspect(允许chain迭代器时中途打印某层的值), partition(consume迭代器，将其根据条件分成两个Vec，用于快速排序)
    */
    fn is_long_pressed_name(name: String, typed: String) -> bool {
        // dbg!(&name, &typed);
        if name.is_empty() || typed.is_empty() {
            return false;
        }
        let mut name = name.into_bytes().into_iter().peekable();
        let mut typed = typed.into_bytes().into_iter().peekable();
        let mut last_a = name.next().unwrap();
        let mut last_b = typed.next().unwrap();
        if last_a != last_b {
            return false;
        }
        while let (Some(&a), Some(&b)) = (name.peek(), typed.peek()) {
            // dbg!(a as char, b as char, last_a as char, last_b as char);
            match (a == last_a, b == last_b) {
                (false, false) => {
                    // dbg!("(false, false)");
                    if a != b {
                        // saeed ssaaedd
                        //   ^     ^
                        // dbg!("a != b");
                        return false;
                    } else {
                        // alex aaleex
                        //   ^    ^
                        // dbg!("a == b");
                        // dbg!(a, b);
                        last_a = name.next().unwrap();
                        last_b = typed.next().unwrap();
                    }
                }
                (false, true) => {
                    // dbg!("(false, true)");
                    // alex aaleex
                    //  ^    ^
                    last_b = typed.next().unwrap();
                }
                (true, false) => {
                    // dbg!("(true, false)");
                    // saeed saed
                    //    ^     ^
                    return false;
                }
                (true, true) => {
                    // dbg!("(true, true)");
                    last_a = name.next().unwrap();
                    last_b = typed.next().unwrap();
                }
            }
        }
        // TODO use unstable peekable iterator next_if API to simplify code
        while let Some(&b) = typed.peek() {
            // vtkgn    vttkgnn
            //      ^        ^
            if b != last_b {
                break;
            } else {
                typed.next().unwrap();
            }
        }
        if name.next().is_some() && typed.next().is_none() {
            // pyplrz ppyypllr
            //     ^         ^
            return false;
        }
        if name.next().is_none() && typed.next().is_some() {
            // alex     alexxr
            //     ^        ^
            return false;
        }
        true
    }
}

#[cfg(test)]
const TEST_CASES: [(&str, &str, bool); 6] = [
    ("alex", "alexxr", false),
    ("pyplrz", "ppyypllr", false),
    ("alex", "aaleex", true),
    // 名字中有两个e相连，但是只打了一个e，所以false
    ("saeed", "ssaaedd", false),
    ("leelee", "lleeelee", true),
    ("laiden", "laiden", true),
];

#[test]
fn test_is_long_pressed_name() {
    for &(name, typed, expected) in &TEST_CASES {
        dbg!(name, typed);
        // dbg!(Solution::is_long_pressed_name(name.to_string(), typed.to_string()));
        assert_eq!(
            Solution::is_long_pressed_name(name.to_string(), typed.to_string()),
            expected
        );
    }
}
