fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for num in nums {
        let mut n = num;
        let mut digit_count = 0;
        while n != 0 {
            n /= 10;
            digit_count += 1;
        }
        if digit_count % 2 == 0 {
            res += 1;
        }
    }
    res
}
