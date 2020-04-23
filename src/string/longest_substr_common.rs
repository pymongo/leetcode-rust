/// ## 两个字符串之间的最长公共子串
/// 递推思路：如果s1[x]==s2[y]且s1[x-1]==s2[y-1]，说明当前子串是公共子串
fn dp(s1: String, s2: String) -> usize {
  let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
  let (s1_len, s2_len) = (s1.len(), s2.len());
  let mut max: usize = 0;
  let mut table: Vec<Vec<bool>> = vec![vec![false; s1_len]; s2_len];
  for y in 0..s1_len {
    for x in 0..s2_len {
      if s1[x] == s2[y] {
        table[x][y] = true;
        let (i, j) = (x - 1, y - 1);
        // 往左上角扫，看看上一项是否也相等
        while (i >= 0) && (j >= 0) {
          if s1[i] != s2[j] {
            break;
          }
        }
        if (x-i) > max {
          max = x-i;
        }
      }
    }
  }
  max
}

#[test]
pub fn test() {
  println!("{}", dp(String::from("caba"), String::from("abac")));
}