#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator(std::iter::Peekable<std::vec::IntoIter<i32>>);

fn traverse_nested_list(nested_integer: NestedInteger) -> Vec<i32> {
    match nested_integer {
        NestedInteger::Int(int) => vec![int],
        NestedInteger::List(list) => {
            let mut nums = vec![];
            for item in list {
                nums.append(&mut traverse_nested_list(item));
            }
            nums
        }
    }
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        // use flattern to merge multi arrays?
        let nums: Vec<i32> =
            nested_list
                .into_iter()
                .fold(Vec::new(), |mut accumulate, list_item| {
                    accumulate.append(&mut traverse_nested_list(list_item));
                    accumulate
                });
        Self(nums.into_iter().peekable())
    }

    fn next(&mut self) -> i32 {
        self.0.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.0.peek().is_some()
    }
}
