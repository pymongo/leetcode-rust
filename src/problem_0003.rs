pub fn run() {
  println!("{}", length_of_longest_substring(String::from("pwwkew")));
}

use std::collections::BTreeSet;

fn length_of_longest_substring(s: String) -> i32 {
  let mut max : usize = 1;
  let mut uniques : BTreeSet<char> = BTreeSet::new();
  for (i, char1) in s.chars().enumerate() {
    uniques.insert(char1);
    for (j, char2) in s.chars().enumerate() {
      if j <= i {
        continue;
      }
      println!();
      print!("({}, {})", i, j);
      println!("char1 = {}, char2 = {}", char1, char2);
      println!("uniques.len() = {}", uniques.len());
      println!();
      for char in &uniques {
        print!("{}", *char);
      }
      print!("|");
      print!("i = {}, char1 = {}", i, char1);
      if !uniques.contains(&char2) {
        uniques.insert(char2);
      }
    }
    max = std::cmp::max(max, uniques.len());
    uniques.clear();
  }
  max as i32
}