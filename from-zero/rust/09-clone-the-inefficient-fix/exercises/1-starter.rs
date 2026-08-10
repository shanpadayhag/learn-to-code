fn main() {
    let s1 = String::from("hello");

    // A move (let s2 = s1;) would retire s1.
    // Instead, make a full independent copy so BOTH stay usable.
    // (hint: .clone())
    // your code here

    println!("{s1} {s2}");   // should print: hello hello
}
