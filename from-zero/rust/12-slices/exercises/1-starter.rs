fn main() {
    let s = String::from("hello world");

    // A slice is a reference to PART of the string: &s[start..end]
    // (end is exclusive). Take "hello" and "world" out as slices —
    // no copying, just windows into s.
    // your code here

    println!("{hello} {world}");   // should print: hello world
}
