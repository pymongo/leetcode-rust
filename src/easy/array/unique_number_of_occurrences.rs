struct Solution;

impl Solution {
    /**
    需要用到第三方库itertools进行unique操作的解法
    ```no_run
    fn unique_occurrences(arr: Vec<i32>) -> bool {
        arr.iter()
            .map(|x| arr.iter().filter(|&y| x==y).count())
            .unique()
            .sum::<usize>()==arr.len()
    }
    ```
    */
    fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counter = std::collections::HashMap::new();
        for num in arr {
            let num_count = counter.entry(num).or_insert(0);
            *num_count += 1;
        }
        let mut seen_count = std::collections::HashSet::new();
        /* Second solution
        for &count in counter.values() {
            seen_count.insert(count)
        }
        return counter.len() == seen_count.len();
        */
        for &count in counter.values() {
            if seen_count.contains(&count) {
                return false;
            } else {
                seen_count.insert(count);
            }
        }
        true
    }
}

#[test]
fn test_unique_occurrences() {
    assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
}