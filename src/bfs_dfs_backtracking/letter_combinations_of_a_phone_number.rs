/// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
/// 通过last_combs/curr_combs(有点像dp里的滚动数组)，也有点像双队列level_order遍历，生成product笛卡尔积
/// 当然也可以用sentry_node的deque去遍历，只是我觉得频繁popleft性能不太好
fn letter_combinations_bfs(digits: String) -> Vec<String> {
    const KEYMAP: [&[u8]; 10] = [
        &[],                 // 0
        &[],                 // 1
        &[b'a', b'b', b'c'], // 2
        &[b'd', b'e', b'f'],
        &[b'g', b'h', b'i'],
        &[b'j', b'k', b'l'],
        &[b'm', b'n', b'o'],
        &[b'p', b'q', b'r', b's'],
        &[b't', b'u', b'v'],
        &[b'w', b'x', b'y', b'z'], // 9
    ];
    if digits.is_empty() {
        return Vec::with_capacity(0);
    }
    let mut last_combs = vec![vec![]];
    for digit in digits.into_bytes() {
        let mut curr_combs = Vec::with_capacity(last_combs.len() * 4);
        for last_comb in last_combs {
            // 这里必须要克隆，因为例如会将一份last_comb="a"给拼成"ad","ae","af"
            for &letter in KEYMAP[(digit - b'0') as usize] {
                let mut curr_comb = last_comb.clone();
                curr_comb.push(letter);
                curr_combs.push(curr_comb);
            }
        }
        last_combs = curr_combs;
    }
    last_combs
        .into_iter()
        .map(|each| unsafe { String::from_utf8_unchecked(each) })
        .collect()
}

// 用DFS回溯的写法会麻烦很多

#[test]
fn test_letter_combinations() {
    const TEST_CASES: [(&str, &[&str]); 1] = [(
        "23",
        &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
    )];
    for &(input, output) in &TEST_CASES {
        assert_eq!(letter_combinations_bfs(input.into()), output);
    }
}
