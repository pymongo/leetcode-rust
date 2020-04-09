// use std::collections::BTreeSet;

pub fn run() {
  print!("{}", find_median2(vec![1,2], vec![1]))
  // let mut a : BTreeSet<i32> = BTreeSet::new();
  // a.insert(3);
  // a.insert(1);
  // a.insert(2);
  // a.insert(3);
  // for i in &a {
  //   println!("{}", i)
  // }
}

// 隐含条件nums1和nums2是有序的
fn find_median2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let mut ans : f64;
  let mut len1 = nums1.len();
  let len2 = nums2.len();
  let total_sum = nums1.len() + nums2.len();
  let stop_index= total_sum / 2;
  let is_odd = total_sum % 2 == 0;
  let mut i = 0;
  let mut j = 0;
  let mut vec = nums1;
  // 二分插入相关变量
  //let left_index = 0;
  // 奇偶数判断
  while j < len2 {
    // 既然第二个数组是有序的，我就不用二分插入了
    while vec[i] < nums2[j] {
      if i < len1-1 {
        i += 1;
      } else {
        i += 1;
        break;
      }
    }
    vec.insert(i, nums2[j]);
    j += 1;
    len1 += 1;
  }
  for i in &vec {
    println!("{}", *i)
  }
  if is_odd {
    ans = (vec[stop_index] + vec[stop_index-1]) as f64 / 2 as f64;
  } else {
    ans = vec[stop_index] as f64;
  }
  ans
}


// 隐含条件nums1和nums2是有序的
// 弊端：TreeSet会去重，需要手写排序算法
// 2064 / 2085 test cases passed
#[cfg(feature = "unused")]
fn find_median(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  use std::collections::BTreeSet;
  let mut ans : f64 = 0 as f64;
  let mut sorted_set : BTreeSet<i32> = BTreeSet::new();
  let (mut nums1, mut nums2) = (nums1, nums2);
  nums1.append(&mut nums2);
  for num in nums1 {
    sorted_set.insert(num);
  }
  let len : usize = sorted_set.len();

  let mut index : usize = 0;

  if len % 2 == 0 {
    let mut middle_left = 0;
    let mut middle_right = 0;
    for each in sorted_set {
      if index == (len/2 -1) {
        middle_left = each;
      } else if index == len/2 {
        middle_right = each;
        break;
      }
      index += 1;
    }
    ans = (middle_left + middle_right) as f64 / 2 as f64
  } else {
    for each in sorted_set {
      if index == len/2 {
        ans = each as f64;
        break;
      }
      index += 1;
    }
  }
  ans
}

