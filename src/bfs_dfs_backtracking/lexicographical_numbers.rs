/// https://leetcode.com/problems/lexicographical-numbers/
/// 字典序排列从1..=n的数字，类似windows file_explorer中文件名会按 1.mp4, 10.mp4, 2.mp4这样的顺序排序
/// 好像跟字典序关系不大，例如python排序后也是这样，因为数字转字符串后排序本身就会是这样的
/// 看测试用例就知道可能要递归，(10进制)10叉树的DFS-回溯
fn lexical_order(n: i32) -> Vec<i32> {
    fn dfs(num: i32, end: i32, ret: &mut Vec<i32>) {
        if num > end {
            return;
        }
        ret.push(num);
        // digit=个位
        for digit in 0..=9 {
            dfs(num * 10 + digit, end, ret);
        }
    }

    let mut ret = Vec::new();
    // 注意10叉树的根节点没有0(第一位不能是0)
    for root in 1..=9 {
        dfs(root, n, &mut ret);
    }
    ret
}

#[test]
fn test_lexical_order() {
    const TEST_CASES: [(i32, &[i32]); 2] = [
        (3, &[1, 2, 3]),
        (13, &[1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]),
    ];
    for (input, output) in TEST_CASES {
        assert_eq!(lexical_order(input), output);
    }
}
