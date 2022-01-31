use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut history = HashMap::new(); // value -> index

    for (i, n) in nums.into_iter().enumerate() {
        let remaining = target - n;

        if let Some(j) = history.get(&remaining) {
            return vec![i as i32, *j];
        } else {
            history.insert(n, i as i32);
        }
    }

    unreachable!();
}
