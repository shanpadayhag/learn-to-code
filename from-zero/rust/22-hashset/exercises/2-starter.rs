// Concept 22 · Exercise 2 — the first value that repeats
//
// `insert` returns a bool: `true` if the value was NEW, `false` if the set
// already had it. That one bool is a ready-made "have I seen this before?"
// Walk the stream, and the first time insert returns false, you've found
// the first repeat.
//
// stream = [5, 2, 8, 2, 9, 5]
//   5 new, 2 new, 8 new, 2 ALREADY THERE  ->  first repeat is 2
//
// Expected output:
//   first repeat: 2

use std::collections::HashSet;

// your code here: return Some(first repeated value), or None if all unique.
fn first_duplicate(items: &[i32]) -> Option<i32> {
    None
}

fn main() {
    let stream = [5, 2, 8, 2, 9, 5];
    match first_duplicate(&stream) {
        Some(value) => println!("first repeat: {}", value),
        None => println!("all unique"),
    }
}
