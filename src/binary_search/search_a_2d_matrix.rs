/// https://leetcode.com/problems/search-a-2d-matrix/
/// check target is_exists in a sorted 2d vector
/// 既然二维数组是有序的，可以将其编号，这样就能看作是对普通数组进行二分，然后根据编号对n的余数和模又能得知编号在二维数组的坐标
fn search_a_2d_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut start, mut end) = (0, m * n - 1);
    while start <= end { // 注意这种二分搜索模板是start<=end，start==end时也要进行一次判断
        let mid = start + (end - start) / 2;
        // 注意i=mid/n而不是mid/m
        let (i, j) = (mid / n, mid % n);
        match matrix[i][j].cmp(&target) {
            std::cmp::Ordering::Less => start = mid + 1,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => {
                // check end=mid-1 overflow
                if mid == 0 {
                    return false;
                }
                end = mid - 1;
            },
        }
    }
    return false;
}

#[test]
fn test_search_a_2d_matrix() {
    let test_cases = vec![
        (vec_vec![[1]], 2, false),
        (vec_vec![[1]], 0, false),
        (vec_vec![[1, 1]], 0, false),
        (vec_vec![[1, 3]], 1, true),
        (
            vec_vec![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
            3,
            true,
        ),
        (
            vec_vec![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
            13,
            false,
        ),
    ];
    for (matrix, target, matrix_contains_target) in test_cases {
        assert_eq!(search_a_2d_matrix(matrix, target), matrix_contains_target);
    }
}
