use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(input_numbers: Vec<i32>, target_sum: i32) -> Vec<i32> {
        let mut seen_number_index: HashMap<i32, i32> = HashMap::new();

        for (current_number_index, &current_number_value) in input_numbers.iter().enumerate() {
            let needed_number_value = target_sum - current_number_value;
            if let Some(&existing_number_index) = seen_number_index.get(&needed_number_value) {
                return vec![existing_number_index, current_number_index as i32];
            }
            seen_number_index.insert(current_number_value, current_number_index as i32);
        }

        vec![]
    }
}

fn main() {
    check(vec![2, 7, 11, 15], 9, vec![0, 1]);
    check(vec![3, 2, 4], 6, vec![1, 2]);
    check(vec![3, 3], 6, vec![0, 1]);
    check(vec![-3, 4, 3, 90], 0, vec![0, 2]);
    check(vec![0, 4, 3, 0], 0, vec![0, 3]);
}

fn check(input_numbers: Vec<i32>, target_sum: i32, expected_indexes: Vec<i32>) {
    let matching_number_indexes = Solution::two_sum(input_numbers.clone(), target_sum);
    assert_eq!(matching_number_indexes, expected_indexes);
    println!(
        "two_sum({:?}, {}) = {:?}",
        input_numbers, target_sum, matching_number_indexes
    );
}
