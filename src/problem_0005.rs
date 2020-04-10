pub fn run() {
  // println!("{}", is_palindromic(String::from("aba")));
  println!("{}", solve(String::from("ac")));
  // println!("{}", solve(String::from("babad")));
}

/*
执行用时: 648 ms, 在所有 Rust 提交中击败了5.69%的用户
内存消耗: 2 MB, 在所有 Rust 提交中击败了66.67%的用户
*/
// #[cfg(feature = "unused")]
fn solve(s: String) -> String {
  let len = s.len();
  if len <= 1 {
    return s;
  }
  let sub_string = &mut String::new();
  let mut result = String::new();
  let mut max_len = 0;

  for i in 0..len {
    for j in i..len {
      // println!("i={},j={}",i,j);
      if j-i < max_len {
        continue;
      }
      *sub_string = (&s[i..=j]).parse().unwrap();
      if is_palindromic(sub_string) {
        if sub_string.len() > max_len {
          result = sub_string.clone();
          max_len = sub_string.len();
        }
      }
    }
  }
  result
}

fn is_palindromic(s: &String) -> bool {
  let bytes = (*s).as_bytes();
  let mut start: usize = 0;
  let mut end: usize = bytes.len() - 1;
  while start < end {
    if bytes[start] != bytes[end] {
      return false;
    } else {
      start += 1;
      end -= 1;
    }
  }
  true
}

