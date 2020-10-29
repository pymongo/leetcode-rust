//! https://leetcode.com/problems/to-lower-case/

struct Solution;

impl Solution {
    // noinspection RsSelfConvention
    #[allow(clippy::wrong_self_convention)]
    fn to_lower_case(s: String) -> String {
        // 既然是ASCII编码，更高效的做法可能是u8数组判断在大写范围的挨个-32
        s.to_ascii_lowercase()
    }
}

#[test]
fn test_my_solution() {
    assert_eq!(Solution::to_lower_case("Apple".to_string()), "apple");
}
