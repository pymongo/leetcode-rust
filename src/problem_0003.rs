pub fn run() {
  // println!("{}", current(String::from("pwwkew")));
  println!("{}", current(String::from(" ")));
}

// 过了一半测试用例，遇到一个空格" "挂了，原因是只匹配在中间出现时的情况
pub fn current(s: String) -> i32 {
  let size : usize = s.len();
  let mut index : usize = 0;
  let mut char : char;
  let mut max : usize = 0;
  let mut unique : std::collections::BTreeSet<char> = std::collections::BTreeSet::new();

  while index < size {
    char = s.chars().nth(index).unwrap();
    if unique.contains(&char) {
      println!(" same! {}", char);
      max = std::cmp::max(max, unique.len());
      unique.clear();
    } else {
      print!("unique {}", char);
      unique.insert(char);
      index += 1;
    }
  }
  // 防止连续不重复的字符串出现在最后
  max = std::cmp::max(max, unique.len());
  max as i32
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