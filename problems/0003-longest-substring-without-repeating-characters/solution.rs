use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(input_text: String) -> i32 {
        let mut last_seen_character_index: HashMap<char, usize> = HashMap::new();
        let mut current_substring_start_index = 0;
        let mut longest_substring_length = 0;

        for (current_character_index, current_character) in input_text.chars().enumerate() {
            if let Some(&existing_character_index) =
                last_seen_character_index.get(&current_character)
            {
                if existing_character_index >= current_substring_start_index {
                    current_substring_start_index = existing_character_index + 1;
                }
            }
            last_seen_character_index.insert(current_character, current_character_index);

            let current_substring_length =
                current_character_index - current_substring_start_index + 1;
            longest_substring_length = longest_substring_length.max(current_substring_length);
        }

        longest_substring_length as i32
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

fn check(input_text: &str, expected_length: i32) {
    let longest_substring_length = Solution::length_of_longest_substring(input_text.to_string());
    assert_eq!(longest_substring_length, expected_length);
    println!(
        "length_of_longest_substring({:?}) = {}",
        input_text, longest_substring_length
    );
}
