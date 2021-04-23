#[derive(PartialEq, Eq, Debug)]
struct Task {
    created_at: i32,
    processing_time: i32,
    index: i32,
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.processing_time
                .cmp(&other.processing_time)
                .then(self.index.cmp(&other.index))
                .reverse(),
        )
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// https://leetcode.com/problems/single-threaded-cpu/
fn single_threaded_cpu(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let len = tasks.len();
    let mut tasks = tasks
        .into_iter()
        .enumerate()
        .map(|(index, task)| Task {
            created_at: task[0],
            processing_time: task[1],
            index: index as i32,
        })
        .collect::<Vec<_>>();
    tasks.sort_unstable_by_key(|task| std::cmp::Reverse(task.created_at));
    let mut task_queue = std::collections::BinaryHeap::new();
    let mut timestamp = 0;
    let mut ret = Vec::with_capacity(len);
    for _ in 0..len {
        if task_queue.is_empty() {
            timestamp = timestamp.max(tasks.last().unwrap().created_at);
        }
        while !tasks.is_empty() && tasks.last().unwrap().created_at <= timestamp {
            task_queue.push(tasks.pop().unwrap());
        }
        let done_task = task_queue.pop().unwrap();
        timestamp += done_task.processing_time;
        ret.push(done_task.index);
    }
    ret
}

#[test]
fn test_single_threaded_cpu() {
    let test_cases = vec![
        (vec_vec![[1, 2], [2, 4], [3, 2], [4, 1]], vec![0, 2, 3, 1]),
        (
            vec_vec![[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]],
            vec![4, 3, 2, 0, 1],
        ),
    ];
    for (input, output) in test_cases {
        assert_eq!(single_threaded_cpu(input), output);
    }
}
