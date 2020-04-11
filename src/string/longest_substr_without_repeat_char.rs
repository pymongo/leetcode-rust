#[cfg(feature = "unused")]
// 最长无重复子串 - 字典存索引的解法
// 内存和速度都排第一，为数不多能跑进0ms的解答
pub fn dict_index(s: String) -> i32 {
  let len = s.len();
  if len <= 1 {
    return len as i32;
  }
  let mut dict = [usize::max_value(); 256];
  let mut sliders = (0, 0);
  let bytes = s.as_bytes();
  let mut max = 0;
  let mut temp_max = 0;
  let mut current_char;
  while sliders.1 < len {
    current_char = bytes[sliders.1] as usize;
    if dict[current_char] != usize::max_value() {
      // 例如abba的用例，slider.1指到2时，slider.0会跳到2
      // slider.1指到3时(最后一个a)，第二次出现重复时，如果不进行判断slider.0会后退到1，导致输出结果变大
      // max() prevent sliders.0's index go back (test case: abba)
      sliders.0 = std::cmp::max(sliders.0, dict[current_char] + 1);
    }
    dict[current_char] = sliders.1;
    temp_max = sliders.1 - sliders.0;
    if temp_max > max {
      max = sliders.1 - sliders.0;
    }
    sliders.1 += 1;
  }
  (max+1) as i32
}