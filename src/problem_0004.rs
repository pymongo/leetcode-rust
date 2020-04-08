// use std::collections::BTreeSet;

pub fn run() {
  print!("{}", find_median(vec![1,2], vec![3,4]))
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
// 弊端：hashSet会去重，需要手写排序算法
// 2064 / 2085 test cases passed
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

