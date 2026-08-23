// Shadowing can change a variable's TYPE, because each `let` makes a fresh box —
// something `mut` can never do (a mut box keeps its type). Here you'll turn a word
// into its length: the name `word` starts as a &str and ends as a number (usize),
// all under the same name.

fn main() {
    let word = "hello";

    // Shadow `word` with its length: `let word = word.len();`  (now it's a number)
    // Then print it.
    // your code here

    // Expected:
    //   5
}
