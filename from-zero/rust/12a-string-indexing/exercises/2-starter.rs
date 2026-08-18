// "Length" is ambiguous for text: `.len()` counts BYTES, `.chars().count()` counts
// CHARACTERS. They differ whenever a character takes more than one byte — which is
// exactly why "the character at position i" isn't a thing Rust can index.

fn main() {
    let word = "café";   // 4 characters, but é takes 2 bytes

    // Print two lines: the byte length, then the character count.
    //   line 1: word.len()
    //   line 2: word.chars().count()
    // your code here

    // Expected:
    //   5
    //   4
}
