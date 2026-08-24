use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let owned_words: Vec<String> = (0..n).map(|_| lines.next().unwrap().unwrap()).collect();
    let words: Vec<&str> = owned_words.iter().map(|word| word.trim()).collect();

    println!("{}", longest_common_prefix(&words));
}

fn longest_common_prefix<'a>(words: &[&'a str]) -> &'a str {
    let Some(first) = words.first() else {
        return "";
    };

    let mut length = first.len();
    for word in &words[1..] {
        length = first
            .bytes()
            .zip(word.bytes())
            .take(length)
            .take_while(|(a, b)| a == b)
            .count();
        if length == 0 {
            break;
        }
    }

    &first[..length]
}
