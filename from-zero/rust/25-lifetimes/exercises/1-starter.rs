// When a function returns a reference and has more than one reference input,
// the compiler can't guess which input the result borrows from — so you name a
// lifetime and share it. `<'a>` introduces the name; `&'a str` labels a borrow
// with it; the same `'a` on both inputs and the output ties them together.

// Return the longer of the two strings (by .len()).
// Tip: remove the `<'a>` and the `'a`s to see `error[E0106]` for yourself.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // if x is at least as long as y, return x; otherwise return y
    // your code here
}

fn main() {
    println!("{}", longest("hello", "hi"));   // should print: hello
    println!("{}", longest("cat", "zebra"));  // should print: zebra
}
