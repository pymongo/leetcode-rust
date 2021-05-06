mod array;
mod codeforces_easy;
mod grid_or_matrix;
mod leetcode_easy;
mod leetcode_very_easy;
mod string;

fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    let mut ret = std::i32::MAX;
    for (i, num) in nums.into_iter().enumerate() {
        if num == target {
            ret = ret.min((i as i32 - start).abs());
        }
    }
    ret
}

#[test]
fn test_get_min_distance() {
    const TEST_CASES: [(&[i32], i32, i32, i32); 3] = [
        (&[5, 3, 6], 5, 2, 2),
        (&[1, 2, 3, 4, 5], 5, 3, 1),
        (&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0, 0),
    ];
    for &(nums, target, start, output) in TEST_CASES.iter() {
        assert_eq!(get_min_distance(nums.to_vec(), target, start), output);
    }
}
