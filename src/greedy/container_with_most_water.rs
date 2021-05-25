/// https://leetcode.com/problems/container-with-most-water/
/// 贪心+相向双指针，每次贪心的移动height[left],height[right]的较小值
fn container_with_most_water(height: Vec<i32>) -> i32 {
    let len = height.len();
    let (mut left, mut right) = (0, len - 1);
    let mut max_area = 0;
    while left < right {
        if height[left] < height[right] {
            // 容器的高度等于左右挡板的最小值(木桶短板原理)，宽度等于right-left
            max_area = max_area.max((right - left) as i32 * height[left]);
            left += 1;
        } else {
            max_area = max_area.max((right - left) as i32 * height[right]);
            right -= 1;
        }
    }
    max_area
}

#[test]
fn test_container_with_most_water() {
    const TEST_CASES: [(&[i32], i32); 1] = [(&[1, 8, 6, 2, 5, 4, 8, 3, 7], 49)];
    for (input, output) in TEST_CASES {
        assert_eq!(container_with_most_water(input.to_vec()), output);
    }
}
