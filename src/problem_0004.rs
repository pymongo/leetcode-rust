// use std::collections::BTreeSet;

pub fn run() {
  print!("ans = {}", find_median2(vec![3], vec![-2, -1]))
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
  let ans : f64;
  let mut len = nums1.len();
  let len1 = nums1.len();
  let mut len2 = nums2.len();
  let total_sum = nums1.len() + nums2.len();
  let stop_index= total_sum / 2;
  let is_odd = total_sum % 2 == 0;
  let mut i = 0;
  let mut j = 0;
  let mut arr:Vec<i32>;
  let pending_to_insert_arr:Vec<i32>;
  if len1 > len2 {
    arr = nums1;
    pending_to_insert_arr = nums2;
  } else {
    arr = nums2;
    pending_to_insert_arr = nums1;
    len = len2;
    len2 = len1;
  }
  // 二分插入相关变量
  //let left_index = 0;
  // 奇偶数判断
  while j < len2 {
    // 既然第二个数组是有序的，我就不用二分插入了
    while arr[std::cmp::min(i, len-1)] < pending_to_insert_arr[j] {
      if i < len {
        i += 1;
      } else {
        break;
      }
    }
    if i < len {
      arr.insert(i, pending_to_insert_arr[j]);
    } else {
      arr.push(pending_to_insert_arr[j]);
    }
    j += 1;
    len += 1;
  }
  if is_odd {
    ans = (arr[stop_index] + arr[stop_index-1]) as f64 / 2 as f64;
  } else {
    ans = arr[stop_index] as f64;
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

