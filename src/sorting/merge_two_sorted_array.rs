//! https://leetcode.com/problems/merge-sorted-array/
//! |88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/merge_two_sorted_array.rs)||

#[cfg(test)]
struct TestCase {
    nums1: Vec<i32>,
    m: i32,
    nums2: Vec<i32>,
    n: i32,
    result: Vec<i32>,
}

#[cfg(test)]
lazy_static::lazy_static! {
    static ref TEST_CASES: Vec<TestCase> = vec![
        TestCase{
            nums1: vec![1,2,3,0,0,0],
            m: 3,
            nums2: vec![2,5,6],
            n: 3,
            result: vec![1,2,2,3,5,6]
        }
    ];
}

#[test]
fn test_my_first_solution() {
    for test_case in &*TEST_CASES {
        let mut nums1 = test_case.nums1.clone();
        let mut nums2 = test_case.nums2.clone();
        my_first_solution(&mut nums1, test_case.m, &mut nums2, test_case.n);
        assert_eq!(nums1, test_case.result);
    }
}

fn my_first_solution(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

}
