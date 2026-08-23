// `r` is a reference — it holds WHERE the value is, not the value itself.
// Use `*` to follow the reference and get the number it points to.

fn main() {
    let x = 42;
    let r = &x;          // r borrows x: r is a &i32 (an "arrow" to x)

    // Follow the arrow with `*` to pull the i32 out, and bind it to `value`.
    // your code here

    // Then print value.
    // your code here

    // Expected:
    //   42
}
