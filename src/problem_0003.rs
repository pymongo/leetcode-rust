pub fn run() {
  println!("{}", length_of_longest_substring(String::from("pwwkew")));
}

pub fn current(s: String) -> i32 {
  1
}

// 164 ms, faster than 12.34%
// 这个方法太慢了，其实一层遍历就够了
#[cfg(feature = "unused")]
fn length_of_longest_substring_slow(s: String) -> i32 {
  let mut max : usize = 0;
  let mut uniques : std::collections::BTreeSet<char> = std::collections::BTreeSet::new();
  for (i, char1) in s.chars().enumerate() {
    uniques.clear();
    uniques.insert(char1);
    for char2 in s[i+1..].chars() {
      if uniques.contains(&char2) {
        // 保证连续不重复
        break;
      } else {
        uniques.insert(char2);
      }
    }
    max = std::cmp::max(max, uniques.len());
  }
  max as i32
}