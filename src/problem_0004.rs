pub fn run() {
  print!("{}", find_median(vec![1,2], vec![3,4]))
}

// 隐含条件nums1和nums2是有序的
fn find_median(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let ans : f64;
  let sum_len = num1.len()+nums2.len();
  if sum_len % 2 == 0 {
    let traverse_end = (sum_len / 2) - 1;
    let mut middle_left;
    let mut middle_right;
    let mut nums1_iter = nums1.iter();
    let mut nums2_iter = nums2.iter();
    let mut index;
    loop {
      // [1 2 3 4] 2
      match (nums1_iter.next(), nums2_iter.next()) {
        (Some(n1), Some(n2)) => {
          index += 2;
          if index < traverse_end-1 {
            continue;
          }
          if index == traverse_end {
            ans = (n1+n2)/2
          }
        },
        (Some(n1), None) => {
          index += 1;
          if index < traverse_end-1 {
            continue;
          }
        },
        (None, Some(n2)) => {
          index += 1;
          if index < traverse_end-1 {
            continue;
          }
        },
        (None, None) => {
          break;
        }
      }
      index += 1;
      if index > traverse_end+2 {
        break;
      }
    }
  } else {

  }
  1 as f64
}

