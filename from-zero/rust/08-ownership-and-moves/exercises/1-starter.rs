fn main() {
    let s1 = String::from("hello");
    let s2 = s1;   // ownership of the text MOVES from s1 to s2

    // Print s2 — it's the owner now.
    // your code here

    // Curious? Add a line that prints s1 and try to compile.
    // Rust will stop you: "value borrowed here after move".
}
