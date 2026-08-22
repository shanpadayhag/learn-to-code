// Concept 22 · Exercise 1 — count the UNIQUE values

use std::collections::HashSet;

fn main() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    let mut seen: HashSet<i32> = HashSet::new();
    for n in numbers {
        seen.insert(n);
    }

    println!("{}", seen.len());
}
