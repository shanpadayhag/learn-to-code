fn main() {
    // Range shorthands: [..n] means "from the start", [n..] means "to the end".
    let s = String::from("Rustacean");

    // Slice out "Rust" (the first 4 bytes) and "acean" (the rest).
    // your code here

    println!("{front} {back}");   // should print: Rust acean
}
