
//! https://leetcode.com/problems/seat-reservation-manager/

struct SeatManagerBruteForce {
    // seat[i]=true，表示座位i可以被预约
    seat: Vec<bool>,
    len: usize,
}

impl SeatManagerBruteForce {
    fn new(n: i32) -> Self {
        let len = n as usize + 1;
        Self {
            seat: vec![true; len],
            len,
        }
    }

    /// 优化: 动态数据中频繁取最值可以用`BinaryHeap`，取最小值耗时O(1)，插入耗时O(logn)
    /// 当然这题用BTreeSet也能AC,只不过first_entry还在nightly不方便操作
    fn reserve(&mut self) -> i32 {
        for i in 1..self.len {
            if self.seat[i] {
                self.seat[i] = false;
                return i as i32;
            }
        }
        unreachable!()
    }


    fn unreserve(&mut self, seat_number: i32) {
        self.seat[seat_number as usize] = true;
    }
}

struct SeatManager(std::collections::BinaryHeap<i32>);

impl SeatManager {
    fn new(n: i32) -> Self {
        let mut heap = std::collections::BinaryHeap::new();
        for i in 1..=n {
            heap.push(-i);
        }
        Self(heap)
    }

    fn reserve(&mut self) -> i32 {
        -self.0.pop().unwrap()
    }


    fn unreserve(&mut self, seat_number: i32) {
        self.0.push(-seat_number);
    }
}


#[test]
fn test_seat_reservation_manager() {
    let mut seat = SeatManager::new(5);
    assert_eq!(seat.reserve(), 1); // 所有座位都可以预约，所以返回最小编号的座位，也就是 1 。
    assert_eq!(seat.reserve(), 2); // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
    seat.unreserve(2); // 将座位 2 变为可以预约，现在可预约的座位为 [2,3,4,5] 。
    assert_eq!(seat.reserve(), 2); // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
    assert_eq!(seat.reserve(), 3); // 可以预约的座位为 [3,4,5] ，返回最小编号的座位，也就是 3 。
    assert_eq!(seat.reserve(), 4); // 可以预约的座位为 [4,5] ，返回最小编号的座位，也就是 4 。
    assert_eq!(seat.reserve(), 5); // 唯一可以预约的是座位 5 ，所以返回 5 。
    seat.unreserve(5); // 将座位 5 变为可以预约，现在可预约的座位为 [5] 。
}
