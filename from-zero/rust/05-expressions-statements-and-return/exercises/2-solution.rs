// Exercise 2 — solution.
// Same result, the idiomatic way: the last line is the value, no `return`, no `;`.

fn add_one(n: i32) -> i32 {
    n + 1
}

fn main() {
    println!("{}", add_one(5));
}
