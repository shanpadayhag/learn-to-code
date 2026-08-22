// Concept 22 · Exercise 2 — the first value that repeats

use std::collections::HashSet;

fn first_duplicate(items: &[i32]) -> Option<i32> {
    let mut seen = HashSet::new();
    for &item in items {
        if !seen.insert(item) {
            return Some(item);
        }
    }
    None
}

fn main() {
    let stream = [5, 2, 8, 2, 9, 5];
    match first_duplicate(&stream) {
        Some(value) => println!("first repeat: {}", value),
        None => println!("all unique"),
    }
}
