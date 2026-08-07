// Exercise 2 — make it compile.
//
// This program tries to change a frozen variable, so it won't compile.
// Fix it by changing exactly ONE word. It should then print: 15
//
// Run it:  rustc 2-starter.rs && ./2-starter

fn main() {
    let score = 10;
    score = score + 5;
    println!("{score}");
}
