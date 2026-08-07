// Exercise 1 — one semicolon too many.
//
// This won't compile: the last line has a semicolon, so the function
// throws its value away and hands back nothing. Remove the one extra
// semicolon so it returns 10.
//
// Run it:  rustc 1-starter.rs && ./1-starter

fn double(n: i32) -> i32 {
    n * 2;
}

fn main() {
    println!("{}", double(5));
}
