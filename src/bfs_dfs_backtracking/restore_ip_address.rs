/*! https://leetcode.com/problems/restore-ip-addresses/
输入一个全是数字的字符串，用回溯算法去摆放3个IP地址的点分隔符，不是MySQL的InetAddr这样ip字符串和u32互转
*/

/// 方便回溯函数传参以及代码维护，传一个结构体等于传多个参数
struct Args {
    s: Vec<u8>,
    s_len: usize,
    s_index: usize,
    /// 既然IP地址固定是4段，回溯算法可以用原生数组小技巧:
    /// 回溯过程中当前的已选择cur不需要push或pop，「新的选择会覆盖掉老的选择」，用原生数组提升性能
    segments: [u8; 4],
    segments_index: usize,
    res: Vec<String>,
}

impl Args {
    fn new(s: String) -> Self {
        let s_len = s.len();
        Self {
            s: s.into_bytes(),
            s_len,
            s_index: 0,
            segments: [0, 0, 0, 0],
            segments_index: 0,
            res: Vec::new(),
        }
    }

    fn add_new_addr_segments_answer_to_res(&mut self) {
        self.res.push(format!(
            "{}.{}.{}.{}",
            self.segments[0], self.segments[1], self.segments[2], self.segments[3]
        ));
    }

    fn swap_index(&mut self, new_s_index: usize, new_seg_index: usize) -> (usize, usize) {
        let old_index = (self.s_index, self.segments_index);
        self.s_index = new_s_index;
        self.segments_index = new_seg_index;
        old_index
    }

    fn restore_index(&mut self, index: (usize, usize)) {
        self.s_index = index.0;
        self.segments_index = index.1;
    }
}

fn restore_ip_addresses(s: String) -> Vec<String> {
    fn helper(args: &mut Args) {
        if args.segments_index == 4 {
            if args.s_index == args.s_len {
                args.add_new_addr_segments_answer_to_res();
            }
            return;
        }

        // 还没找全4段ip就遍历完字符串，那么字符串s一定不是合法IP地址，提前回溯
        if args.s_index == args.s_len {
            return;
        }

        // 由于leetcode的IP地址要求不能有前置0，所以遇到第一个0那么这段就一定是0
        if args.s[args.s_index] == b'0' {
            args.segments[args.segments_index] = 0;
            let backup_index = args.swap_index(args.s_index + 1, args.segments_index + 1);
            helper(args);
            args.restore_index(backup_index);
        }

        let mut ip_addr_seg = 0;
        for i in args.s_index..args.s_len {
            fn checked_addr_seg_add(old_val: u8, new_digit: u8) -> Option<u8> {
                old_val.checked_mul(10)?.checked_add(new_digit - b'0')
            }
            match checked_addr_seg_add(ip_addr_seg, args.s[i]) {
                Some(new_val) => {
                    ip_addr_seg = new_val;
                    if ip_addr_seg == 0_u8 {
                        // 上一个if语句已经处理了IPv4为0的情况，这里就直接跳过
                        // 直接break，避免0的情况被重复记入
                        break;
                    }
                    args.segments[args.segments_index] = ip_addr_seg;
                    let backup_index = args.swap_index(i + 1, args.segments_index + 1);
                    helper(args);
                    args.restore_index(backup_index);
                }
                None => return,
            }
        }
    }
    let mut args = Args::new(s);
    helper(&mut args);
    args.res
}

#[test]
fn test_restore_ip_addresses() {
    let test_cases = vec![
        ("0000", vec!["0.0.0.0".to_string()]),
        (
            "25525511135",
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()],
        ),
    ];
    for (s, expected) in test_cases {
        assert_eq!(restore_ip_addresses(s.to_string()), expected);
    }
}
