/// https://leetcode.com/problems/majority-element/
/// 寻找数组中出现次数大于len/2次的元素
fn majority_element(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    let half = (n / 2) as u16;
    let mut cnt = std::collections::HashMap::with_capacity(n);
    for num in nums {
        if let Some(count) = cnt.get_mut(&num) {
            if half.eq(count) {
                return num;
            }
            *count += 1;
        } else {
            cnt.insert(num, 1u16);
        }
    }
    unreachable!();
}

fn majority_element_best(nums: Vec<i32>) -> i32 {
    let mut count: u16 = 0;
    let mut candidate = 0;
    for num in nums {
        if count == 0 {
            candidate = num;
        }
        if num == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}
