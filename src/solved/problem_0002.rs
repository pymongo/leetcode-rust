// match两个的用法1, 定义了一个匿名函数node_val用于取出当前节点的值，很新颖
#[cfg(feature = "unused")]
fn add_two_numbers_1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cf = 0;
    let node_val = |x: &Option<Box<ListNode>>| {
        if let Some(node) = x {
            Some(node.val)
        } else {
            None
        }
    };
    let mut res: Option<Box<ListNode>> = None;
    let (mut p1, mut p2, mut pr) = (&l1, &l2, &mut res);

    loop {
        let mut sum = 0;
        match (node_val(p1), node_val(p2)) {
            (Some(val1), Some(val2)) => {
                sum = val1 + val2 + cf;
            }
            (Some(val1), None) => {
                sum = val1 + cf;
            }
            (None, Some(val2)) => {
                sum = val2 + cf;
            }
            (None, None) => {
                if cf == 1 {
                    *pr = Some(Box::new(ListNode::new(1)));
                }
                break;
            }
        };

        cf = sum / 10;
        sum = sum - 10 * cf;
        *pr = Some(Box::new(ListNode::new(sum)));
        // println!("p1: {:?}, p2: {:?}, pr: {:?}, sum: {}", node_val(p1), node_val(p2), node_val(pr), sum);
        // p1 = &p1.as_ref().unwrap().next; 另一种写法，要求p1不为None；
        if let Some(node) = p1 {
            p1 = &node.next;
        }
        if let Some(node) = p2 {
            p2 = &node.next;
        }
        if let Some(node) = pr {
            pr = &mut node.next;
        }
    }

    res
}

// 「最佳答案」与解答1类似
#[cfg(feature = "unused")]
pub fn add_two_numbers_2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sum = 0;
    let (mut l1, mut l2) = (l1, l2);
    let mut l = None;  // 在上下文中可以推断类型
    let mut p = &mut l;

    loop {
        match (l1, l2) {   // l1,l2是`move`语义匹配，每次匹配前都需要初始化值
            (Some(v1), Some(v2)) => {
                sum += v1.val + v2.val;
                l1 = v1.next;
                l2 = v2.next;
            }
            (Some(v1), None) => {
                sum += v1.val;
                l1 = v1.next;
                l2 = None;
            }
            (None, Some(v2)) => {
                sum += v2.val;
                l2 = v2.next;
                l1 = None;
            }
            (None, None) => {
                break;
            }
        }
        *p = Some(Box::new(ListNode::new(sum % 10))); //不管sum是否大于10，都可以使用sum%10的值来构建新“节点“
        sum /= 10; // 获取进位值，否则初始为0
        if let Some(p_box_node) = p {
            // 遍历思想：赋值好当前节点后，把p指向下一个节点(None)
            p = &mut p_box_node.next
        }
        /* 批注 if let(p_box_node) = p { f(p_box_node) } 等价于
        match p {
          Some(i) => f(p_box_node),
          _ => {},
        };
        */
    }
    if sum != 0 {
        *p = Some(Box::new(ListNode::new(sum)));
    }

    l
}

// 「美服第一」优化很多
#[cfg(feature = "unused")]
pub fn add_two_numbers_3(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut curr = &mut head;
    let mut carry = 0;

    loop {
        match (l1, l2) {
            (Some(mut v1), Some(mut v2)) => {
                let mut val = v1.val + v2.val + carry;

                if val >= 10 {
                    val -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }

                let node = ListNode::new(val);
                curr.next = Some(Box::new(node));
                l1 = v1.next.take();
                l2 = v2.next.take();
            }
            (Some(mut v1), None) => {
                let mut val = v1.val + carry;

                if val >= 10 {
                    val -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }

                let node = ListNode::new(val);
                curr.next = Some(Box::new(node));
                l1 = v1.next.take();
                l2 = None;
            }
            (None, Some(mut v2)) => {
                let mut val = v2.val + carry;

                if val >= 10 {
                    val -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }

                let node = ListNode::new(val);
                curr.next = Some(Box::new(node));
                l2 = v2.next.take();
                l1 = None;
            }
            (None, None) => {
                if carry == 0 {
                    break;
                }

                let node = ListNode::new(1);
                curr.next = Some(Box::new(node));
                break;
            }
        }

        curr = curr.next.as_mut().unwrap();
    }

    head.next
}

