/*
LeetCode上的解答基本就一种双游标sliding window算法
不同的是有人用Set、Map、Array<boolean>等等数据结构牺牲空间换取时间而已
*/
#[cfg(feature = "unused")]
pub fn run() {
    // println!("{}", current(String::from("pwwkew")));
    // println!("{}", sliding_window_ascii2(String::from("dvdk")));
    // println!("{}", sliding_window_ascii2(String::from("abcabcbb")));
    // println!("{}", dict_index(String::from("abba")));
}

// [Link] 最佳解法(耗时0ms)的在`src/string/longest_substr_without_repeat_char.rs`

// 失败在dvdf的用例上，没想到最佳实践，反正我暴力求解已经pass了，看看大佬的答案吧
#[cfg(feature = "unused")]
fn current(s: String) -> i32 {
    let size: usize = s.len();
    let mut index: usize = 0;
    let mut char: char;
    let mut max: usize = 0;
    let mut unique: std::collections::BTreeSet<char> = std::collections::BTreeSet::new();

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
    let mut max: usize = 0;
    let mut uniques: std::collections::BTreeSet<char> = std::collections::BTreeSet::new();
    for (i, char1) in s.chars().enumerate() {
        uniques.clear();
        uniques.insert(char1);
        for char2 in s[i + 1..].chars() {
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
/*
  int n = s.length(), ans = 0;
  Map<Character, Integer> map = new HashMap<>();//key出现的字符，value对应的最新的位置
  // try to extend the range [i, j]
  for (int end = 0, start = 0; end < n; end++) {
      if (map.containsKey(s.charAt(end))) {
          start = Math.max(map.get(s.charAt(end)) + 1, start);//由于重复的坐标不知道在start的前方还是后方，所以要取个最大值
      }
      ans = Math.max(ans, end - start + 1);
      map.put(s.charAt(end), end);
  }
  return ans;
*/

#[cfg(feature = "unused")]
fn sliding_window_ascii(s: String) -> i32 {
    // sliding window rear
    let mut rear: usize = 0;
    // sliding window front
    let mut front_need_to_move: usize = 0;
    let mut ascii_dict: [usize; 256] = [0; 256];
    let mut max: usize = 0;
    let length: usize = s.len();
    for (front, uint8) in s.bytes().enumerate() {
        println!("{}", uint8 as usize);
        if ascii_dict[uint8 as usize] > 0 {
            // 由于重复的index不知道在rear的前方还是后方，所以要取个最大值
            println!("{}", ascii_dict[uint8 as usize]);
            rear = std::cmp::max(ascii_dict[uint8 as usize] + 1, rear);
        }
        max = std::cmp::max(max, front - rear + 1);
        ascii_dict[uint8 as usize] = front + 1;
    }
    println!("{}", rear);
    max as i32
}

// 又是错在"abcabcbb"这个用例
/*
执行用时 :
32 ms, 在所有 Rust 提交中击败了 25.54%的用户
内存消耗 :
2 MB, 在所有 Rust 提交中击败了95.65%的用户
*/
#[cfg(feature = "unused")]
fn sliding_window_ascii2(s: String) -> i32 {
    // 不建议在dict中记录/更新read的index，一是难以理解，二是速度不快，内存使用也大
    let mut ascii_seen: [bool; 256] = [false; 256];
    let bytes: &[u8] = s.as_bytes();
    let length: usize = bytes.len();
    let mut rear: usize = 0;
    let mut front: usize = 0;
    let mut max_answer: usize = 0;
    while front < length {
        if ascii_seen[bytes[front] as usize] {
            println!("before clear front = {}, rear = {}", front, rear);
            while bytes[rear] != bytes[front] {
                ascii_seen[bytes[rear] as usize] = false;
                rear += 1;
            }
            // 例如tmmzuxt的front走到2时，rear先把't'踢出已见清单，rear: 0->1, rear=1时跳出while,rear自增
            rear += 1;
            // println!("after clear = {}, rear = {}", front, rear);
            // ascii_dict[byte_cache] = false;
        }
        ascii_seen[bytes[front] as usize] = true;
        front += 1;
        if front - rear > max_answer {
            // println!("front = {}, rear = {}", front, rear);
            max_answer = front - rear;
        }
    }
    // println!("front = {}, rear = {}", front, rear);
    max_answer as i32
}