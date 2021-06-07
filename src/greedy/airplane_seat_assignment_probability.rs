/** https://leetcode.com/problems/airplane-seat-assignment-probability/comments/
persion_1' seat:
- 1/n% correctly to seat_1, then person 2...n can seat correctly, so person_n 100% seat_n
- 1/n% seat to seat_n, person_n 0% seat_n
- n-2/n seat to k in 2...n,
  //assume person_1 and seat_k is not exist
  **reduce** problem to f(n-1), reduce problem to probability of `person k seat correctly to seat_1`
*/
#[allow(clippy::suboptimal_flops)]
fn nth_person_gets_nth_seat(n: i32) -> f64 {
    // if n == 1 { 1.0 } else { 0.5 }
    let n_f64 = f64::from(n);
    if n == 1 {
        return 1.0;
    }
    // person_1_seat_1 + person_1_seat_1*f(n-1)
    1.0 / n_f64 + (f64::from(n - 2) / n_f64) * nth_person_gets_nth_seat(n - 1)
}
