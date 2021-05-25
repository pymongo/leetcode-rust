/** https://leetcode.com/problems/valid-triangle-number/
输入一个排序数组，请问从数组中取3个值能组成多少个不同的三角形
由于数组是有序的，满足nums[a] + nums[b] > nums[c]即可，转换成两数之和大于类型问题
这题有点像three sum，和三数之和类似，遍历时可变的双指针要在等式的左边，而固定不变的target放等式右边
如果本题固定a，双指针是b和c，那么一定会出现漏掉解的情况，所以等式右边的target不能是可变的
*/
fn triangle_number(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    if n < 3 {
        return 0;
    }
    let mut ret = 0;

    // c是最长边的下标索引，a和b是较短边的索引
    // nums[a] <= nums[b] <= nums[c]
    for c in 2..n {
        let mut a = 0;
        let mut b = c - 1;
        while a < b {
            if nums[a] + nums[b] > nums[c] {
                // [3,4,6,7]中如果a(3) + b(6) > c(7)，那么a右移一位的解4,6,7也满足条件
                // 这步是不会漏掉解的关键，参考two-sum-less-than-or-equal-to-target
                // 既然nums[left]+nums[right]>target，那么[left..right-1, right]的解都满足条件
                // count会算上固定b之后[a..b-1, b]的所有解，然后b-=1; 而two_sum_le一题是固定a之后[a,a+1..b]所有解，所以a+=1
                ret += b - a; // 批量数
                b -= 1;
            } else {
                // 不会重复计算4,6,7这个解两次，因为4,6,7被记入时，b左移一位，已经退出循环，可以代入一些很小的用例去推敲
                a += 1;
            }
        }
    }

    ret as i32
}

#[test]
fn test_triangle_number() {
    /*
    输入: [2,2,3,4]
    输出: 3
    有效的组合是:
    2,3,4 (使用第一个 2)
    2,3,4 (使用第二个 2)
    2,2,3
    */
    const TEST_CASES: [(&[i32], i32); 1] = [(&[2, 2, 3, 4], 3)];
    for (input, output) in TEST_CASES {
        assert_eq!(triangle_number(input.to_vec()), output);
    }
}
