// Exercise 2 — the tidy way.
//
// This works, but it uses `return`. Rewrite `add_one` the idiomatic way:
// the last line, with no `return` and no semicolon. It should still print 6.
//
// Run it:  rustc 2-starter.rs && ./2-starter

fn add_one(n: i32) -> i32 {
    return n + 1;
}

fn main() {
    println!("{}", add_one(5));
}
