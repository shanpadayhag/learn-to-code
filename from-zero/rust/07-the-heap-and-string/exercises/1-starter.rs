fn main() {
    // A String is text you can grow at runtime.
    // Start from "Hello", then add to it, then print the whole thing.
    let mut greeting = String::from("Hello");

    // Add ", world" to the end, then add a single '!' character.
    // (hint: .push_str("...") adds text, .push('c') adds one char)
    // your code here

    println!("{greeting}");   // should print: Hello, world!
}
