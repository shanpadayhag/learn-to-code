fn main() {
    let mut a = 5;
    let b = a;   // b gets its own copy of the 5
    a = 99;      // we change a AFTER the copy

    // Print a on one line, then b on the next.
    // Guess first: will b be 99 or 5?
    // your code here
}
