//! 数学相关问题或位运算
mod count_primes;
mod excel_sheet_column_title;
mod perfect_number;
mod pow;
mod sqrt;

/// https://leetcode.com/problems/ugly-number/
const fn is_ugly(mut num: i32) -> bool {
    if num == 0 {
        return false;
    }
    while num % 2 == 0 {
        num /= 2;
    }
    while num % 3 == 0 {
        num /= 3;
    }
    while num % 5 == 0 {
        num /= 5;
    }
    num == 1
}
