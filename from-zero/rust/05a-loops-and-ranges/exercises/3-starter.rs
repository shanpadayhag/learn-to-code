// A range like 1..=10 only counts UPWARD in steps of 1. When you instead want
// "keep going while some condition holds" — like subtracting 3 over and over until
// you'd go below zero — a `while` loop is the natural fit.
//
// Start at 10 and, as long as n is still >= 0, print n and then subtract 3.
// Expected: 10, 7, 4, 1  (the next value, -2, is below 0, so the loop stops).

fn main() {
    let mut n = 10;

    // while n >= 0 { print n, then n -= 3 }
    // your code here
}
