use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let words: Vec<&str> = lines.take(n).map(str::trim).collect();

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
