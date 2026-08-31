use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_of_seen_value: HashMap<i32, i32> = HashMap::new();

        for (current_index, &current_value) in numbers.iter().enumerate() {
            let needed_value = target - current_value;
            if let Some(&earlier_index) = index_of_seen_value.get(&needed_value) {
                return vec![earlier_index, current_index as i32];
            }
            index_of_seen_value.insert(current_value, current_index as i32);
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

fn check(numbers: Vec<i32>, target: i32, expected: Vec<i32>) {
    let answer = Solution::two_sum(numbers.clone(), target);
    assert_eq!(answer, expected);
    println!("two_sum({:?}, {}) = {:?}", numbers, target, answer);
}
