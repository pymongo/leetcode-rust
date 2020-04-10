pub fn run() {
  println!("{}", is_palindromic(String::from("aba")));
}

#[cfg(feature = "unused")]
fn solve(s : String) -> String {
  // let i:usize =0;
  // let j:usize;
  // let bytes = s.bytes();
  // let len = bytes.len();
  // let mut start:usize = 0;
  // let mut end:usize = len;
  // let mut vec = Vec::new();
  // let max_len = 0;
  // while i < len {
  //   start = 0;
  //   end = len;
  //   while start < end {
  //     if bytes[start] != bytes[end] {
  //       break;
  //     } else {
  //       vec.push()
  //       start += 1;
  //       end -= 1;
  //     }
  //   }
  // }

  String::from("sfd")
}

fn is_palindromic(s : String) -> bool {
  let bytes = s.as_bytes();
  let mut start:usize = 0;
  let mut end:usize = bytes.len()-1;
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