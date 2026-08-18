// A generic function works for ANY type. Put <T> after the name to introduce a
// type placeholder, then use T where a real type would go. You can only shuffle a
// bare T around (move it, return it, put it in a tuple) — not compare or print it —
// because Rust knows nothing about T yet. Swapping two values only shuffles them,
// so it works for any T with no extra promises.

fn swap<T>(a: T, b: T) -> (T, T) {
    // Return the two values in the OPPOSITE order.
    // your code here
}

fn main() {
    println!("{:?}", swap(10, 20));            // two i32s
    println!("{:?}", swap("hello", "world"));  // two &str
    // Expected:
    //   (20, 10)
    //   ("world", "hello")
}
