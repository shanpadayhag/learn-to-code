// `continue 'label` jumps to the next turn of the LABELED loop, not the inner one.

fn main() {
    // Label the outer loop `'outer`. Loop `a` over 1..=3 and `b` over 1..=3.
    // If a == b, `continue 'outer` (skip the rest of this `a`).
    // Otherwise print "{a},{b}".
    // Expected output:
    //   2,1
    //   3,1
    //   3,2
    // your code here
}
