use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(text: String) -> i32 {
        let mut last_seen_index: HashMap<char, usize> = HashMap::new();
        let mut window_start = 0;
        let mut longest = 0;

        for (current_index, current_char) in text.chars().enumerate() {
            if let Some(&previous_index) = last_seen_index.get(&current_char) {
                if previous_index >= window_start {
                    window_start = previous_index + 1;
                }
            }
            last_seen_index.insert(current_char, current_index);
            longest = longest.max(current_index - window_start + 1);
        }

        longest as i32
    }
}

fn main() {
    check("abcabcbb", 3);
    check("bbbbb", 1);
    check("pwwkew", 3);
    check("", 0);
    check(" ", 1);
    check("dvdf", 3);
    check("abcdefg", 7);
    check("aabbaa", 2);
}

fn check(text: &str, expected: i32) {
    let longest = Solution::length_of_longest_substring(text.to_string());
    assert_eq!(longest, expected);
    println!("length_of_longest_substring({:?}) = {}", text, longest);
}
