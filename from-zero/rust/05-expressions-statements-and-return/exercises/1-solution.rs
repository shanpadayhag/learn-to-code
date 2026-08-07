// Exercise 1 — solution.
// The fix: drop the semicolon so the last line is the value handed back.

fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", double(5));
}
