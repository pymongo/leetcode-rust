//! https://leetcode.cn/problems/minimum-falling-path-sum

/*
here is a leetcode question: Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.

A falling path starts at any element in the first row and chooses the element in the next row that is either directly below or diagonally left/right. Specifically, the next element from position (row, col) will be (row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).

below is my Rust code, pass 70/100 test case but time limit exceed

I don't know how to solve this problem using dynamtic programing

but I have a optimize idea is add a cache arg in search function, to cache each input/output of search, can you help me to impl it?
*/

/// time limit exceed
fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut cache = vec![vec![None; n]; m]; // Create a cache with the same dimensions as matrix

    let mut min = i32::MAX;
    for j in 0..n {
        min = min.min(search(0, j, &matrix, &mut cache));
    }

    min
}

fn search(row: usize, col: usize, matrix: &Vec<Vec<i32>>, cache: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if row == matrix.len() {
        return 0; // We've reached the bottom of the matrix, no more values to add
    }

    // If we have a cached value, return it
    if let Some(cached) = cache[row][col] {
        return cached;
    }

    let cur_val = matrix[row][col];
    let mut min_path = cur_val + search(row + 1, col, matrix, cache); // Vertical

    if col > 0 {
        min_path = min_path.min(cur_val + search(row + 1, col - 1, matrix, cache)); // Diagonally left
    }
    if col + 1 < matrix[0].len() {
        min_path = min_path.min(cur_val + search(row + 1, col + 1, matrix, cache)); // Diagonally right
    }

    // Store the result in the cache before returning
    cache[row][col] = Some(min_path);
    min_path
}

/*
## DP solution
assume m,j is rows,columns of matrix
answer = min(sum[m-1][0], sum[m-1][1], ..., sum[m-1][n-1])
sum[m-1][0]=matrix[m-1][0] + min(sum[m-2][0], sum[m-2][1])
*/

#[test]
fn test() {
    for (matrix, min_path_sum) in [
        (
            vec_vec![
                [-19,57],[-40,-5]
            ],
            -59
        ),
        (
            vec_vec![
                [2,1,3],[6,5,4],[7,8,9]
            ],
            13
        )
    ] {
        assert_eq!(min_falling_path_sum(matrix), min_path_sum);
    }
}
