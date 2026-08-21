// The factorial of n is 1 x 2 x 3 x ... x n. A range isn't only for looping — it can
// also do math over all its numbers. `(1..=n).product()` multiplies every number in
// the range together. (And `(1..=n).sum()` would add them.)
//
// Fill in `factorial` so it returns n! using a range and `.product()`.
// (Factorial of 0 is 1 — and an empty range's product is conveniently 1, so this
// handles n = 0 for free.)

fn factorial(n: u64) -> u64 {
    // your code here
}

fn main() {
    let n = 5;
    println!("{n}! = {}", factorial(n));   // 5! = 120
}
