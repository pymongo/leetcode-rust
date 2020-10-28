struct Solution;

impl Solution {
    fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}
