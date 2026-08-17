// `match` compares a value against patterns, top to bottom, and runs the first
// arm that fits. It must cover EVERY possibility — `_` is the catch-all "anything
// else". It's also an expression, so each arm produces the value `grade` returns.

fn grade(score: u32) -> char {
    // Match `score` against ranges (written `low..=high`, inclusive):
    //   90..=100 => 'A'
    //   80..=89  => 'B'
    //   70..=79  => 'C'
    //   60..=69  => 'D'
    //   anything else => 'F'
    // your code here
}

fn main() {
    println!("{}", grade(95));   // should print: A
    println!("{}", grade(83));   // should print: B
    println!("{}", grade(42));   // should print: F
}
