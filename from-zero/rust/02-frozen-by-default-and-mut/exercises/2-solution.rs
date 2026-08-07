// Exercise 2 — solution.
// The one-word fix: add `mut` so the box is allowed to change.

fn main() {
    let mut score = 10;
    score = score + 5;
    println!("{score}");
}
