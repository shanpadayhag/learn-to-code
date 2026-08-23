// This is the exact pattern that started the lesson. `.trim()` borrows the String
// (it takes &self) and hands back a &str window over the same text with the outer
// whitespace skipped. Shadow `raw` with that &str so the name now means the trimmed
// slice. Nothing is moved, and the original String stays alive underneath.

fn main() {
    let raw = String::from("   hello   ");

    // Shadow `raw` with its trimmed &str: `let raw = raw.trim();`
    // Then print it inside [brackets] so the trimming is visible.
    // your code here

    // Expected:
    //   [hello]
}
