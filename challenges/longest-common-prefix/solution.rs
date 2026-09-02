use std::io::{self, BufRead};

fn main() {
    let standard_input = io::stdin();
    let mut input_lines = standard_input.lock().lines();

    let input_word_count: usize = input_lines.next().unwrap().unwrap().trim().parse().unwrap();
    let owned_input_words: Vec<String> = (0..input_word_count)
        .map(|_| input_lines.next().unwrap().unwrap())
        .collect();
    let candidate_words: Vec<&str> = owned_input_words.iter().map(|word| word.trim()).collect();

    println!("{}", longest_common_prefix(&candidate_words));
}

fn longest_common_prefix<'a>(candidate_words: &[&'a str]) -> &'a str {
    if candidate_words.is_empty() {
        return "";
    }
    let first_word = candidate_words[0];

    let mut common_prefix_length = first_word.len();
    for current_word in &candidate_words[1..] {
        common_prefix_length = first_word
            .bytes()
            .zip(current_word.bytes())
            .take(common_prefix_length)
            .take_while(|(first_word_byte, current_word_byte)| first_word_byte == current_word_byte)
            .count();
        if common_prefix_length == 0 {
            break;
        }
    }

    &first_word[..common_prefix_length]
}
