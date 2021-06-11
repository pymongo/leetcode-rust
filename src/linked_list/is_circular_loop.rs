//! is_circular_loop use fast slow two pointers or visited: [bool; len]

/// https://leetcode.com/problems/circular-array-loop/
/// similar_problem: |202|[Happy Number]
/// require_1: all num in loop is same sign
/// require_2: circular_loop's len > 2
/// hint: loop's start can be any position on array
/// Wrong Answer
fn circular_array_loop_my_wrong_solution(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let len_i32 = len as i32;
    let mut visited = vec![0_u8; len];
    let mut index = 0;
    for _ in 0..len {
        let index_usize = index as usize;
        if visited[index_usize] == 1 {
            // check circular len >= 2
            let curr_index = index;
            for _ in 0..2 {
                index = (index + nums[index as usize]).rem_euclid(len_i32);
            }
            if index == curr_index {
                return false;
            }
            if cfg!(debug_assertions) {
                dbg!(&visited);
            }
            // check every num in circular is either all positive or all negative
            let is_positive = nums[curr_index as usize] > 0;
            if cfg!(debug_assertions) {
                dbg!(is_positive);
            }
            for (i, num) in nums.into_iter().enumerate() {
                if visited[i] == 0 {
                    continue;
                }
                if cfg!(debug_assertions) {
                    dbg!(num);
                }
                if is_positive && num < 0 {
                    return false;
                }
            }
            return true;
        }
        visited[index_usize] += 1;
        index = (index + nums[index_usize]).rem_euclid(len_i32);
    }
    false
}

/// Runtime: 20 ms
fn circular_array_loop_fast_slow_two_pointers(nums: Vec<i32>) -> bool {
    fn next(curr: i32, nums: &[i32], len: i32) -> Option<i32> {
        let num_curr = nums[curr as usize];
        let next = (curr + num_curr).rem_euclid(len);
        let num_next = nums[next as usize];
        if cfg!(debug_assertions) {
            dbg!(next, num_next);
        }
        if (num_curr > 0 && num_next > 0) || (num_curr < 0 && num_next < 0) {
            return Some(next);
        }
        None
    }
    let len = nums.len() as i32;
    for start in 0..len {
        let mut slow_opt = Some(start);
        let mut fast_opt = next(start, &nums, len);
        // slow and fast must same sign
        while let (Some(slow), Some(fast)) = (slow_opt, fast_opt) {
            if slow == fast {
                // circular's len > 2
                if let Some(slow_next) = next(slow, &nums, len) {
                    if slow_next == slow {
                        // 如果是自身环
                        break;
                    }
                    return true;
                }
            }
            slow_opt = next(slow, &nums, len);
            fast_opt = match next(fast, &nums, len) {
                Some(nxt) => next(nxt, &nums, len),
                None => break,
            };
        }
    }
    false
}

/// Runtime: 0ms
fn circular_array_loop_my_solution2(nums: Vec<i32>) -> bool {
    struct Helper {
        nums: Vec<i32>,
        len_i32: i32,
        /// index which impossible to make a loop
        impossible_index: Vec<bool>,
    }
    impl Helper {
        fn next_index(&self, curr_index: usize) -> Option<usize> {
            let curr_num = self.nums[curr_index];
            let next_index = (curr_index as i32 + curr_num).rem_euclid(self.len_i32) as usize;
            if self.impossible_index[next_index] {
                return None;
            }
            let next_num = self.nums[next_index];
            if (curr_num > 0 && next_num < 0) || (curr_num < 0 && next_num > 0) {
                return None;
            }
            Some(next_index)
        }
    }

    let len = nums.len();
    let mut helper = Helper {
        nums,
        len_i32: len as i32,
        impossible_index: vec![false; len],
    };
    for start_index in 0..len {
        if helper.impossible_index[start_index] {
            continue;
        }
        let mut visited_index = vec![start_index];
        let mut visited = vec![false; len];
        visited[start_index] = true;
        let mut curr_index = start_index;
        while let Some(next_index) = helper.next_index(curr_index) {
            if visited[next_index] {
                if let Some(next_next_index) = helper.next_index(next_index) {
                    if next_next_index == next_index {
                        // next is circular to next, circular_loop' len < 2
                        // curret loop is a bad loop
                        break;
                    }
                    return true;
                }
            }
            visited[next_index] = true;
            visited_index.push(next_index);
            curr_index = next_index;
        }
        // mark current search path is impossible to make a loop, to prevent duplicate search
        for each in visited_index {
            helper.impossible_index[each] = true;
        }
    }
    false
}

#[test]
fn test_circular_array_loop() {
    const TEST_CASES: [(&[i32], bool); 4] = [
        (&[3, 1, 2], true),
        (&[2, -1, 1, -2, -2], false),
        (&[2, -1, 1, 2, 2], true),
        // nums[1]<->nums[2], Every nums[seq[j]] is either all positive or all negative
        (&[-2, 1, -1, -2, -2], false),
    ];
    for (nums, is_circular) in TEST_CASES {
        dbg!(nums);
        assert_eq!(
            circular_array_loop_fast_slow_two_pointers(nums.to_owned()),
            is_circular
        );
        assert_eq!(
            circular_array_loop_my_solution2(nums.to_owned()),
            is_circular
        );
        //assert_eq!(circular_array_loop_solution_loop_start_at_0(nums.to_owned()), is_circular);
    }
}
