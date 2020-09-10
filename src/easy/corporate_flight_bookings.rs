struct Solution;

// 上车站点的下标
const GET_ON_STATION: usize = 0;
// 下车站点的下标
const GET_OFF_STATION: usize = 1;
// 上下车的人数
const COUNT: usize = 2;

impl Solution {
    pub fn corp_flight_bookings(data: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut stations = vec![0; n];
        for each in data {
            // 注意入参的车站从1开始编号
            stations[(each[GET_ON_STATION]-1) as usize] += each[COUNT];
            // 注意公交车下车数据的小标是j+1
            let get_off_station = each[GET_OFF_STATION] as usize;
            if get_off_station < n {
                stations[get_off_station] -= each[COUNT];
            }
        }
        for i in 1..n {
            // 从差分数组(每站净上车数)推出车运行到每站时各有几人
            stations[i] += stations[i-1];
        }
        stations
    }

    pub fn func2(records: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut stations = vec![0; n + 1];
        for record in records {
            let cnt = record[2];
            stations[record[0] as usize - 1] += cnt;
            stations[record[1] as usize] -= cnt;
        }
        let mut curr = 0;
        // 根据差分数组还原原数组
        for i in 0..=n as usize {
            curr += stations[i];
            stations[i] = curr;
        }
        stations.pop();
        stations
    }

}

#[cfg(test)]
const TEST_CASES: [(&[&[i32]], i32, &[&[i32]]); 1] = [
    (&[&[1, 2, 10], &[2, 3, 20], &[2, 5, 25]], 5, &[&[1, 2, 10], &[2, 3, 20], &[2, 5, 25]]),
];

#[test]
fn test() {
    let data = vec![
        vec![1, 2, 10],
        vec![2, 3, 20],
        vec![2, 5, 25]
    ];
    Solution::corp_flight_bookings(data, 5);
}
