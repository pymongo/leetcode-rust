mod gray_code;
mod shuffle_the_array;
mod squares_of_a_sorted_array;
mod sum_of_all_odd_length_subarrays;

pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut res = a.iter().map(|&x| x * x).collect::<Vec<i32>>();
    res.sort_unstable();
    res
}
