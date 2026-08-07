// Exercise 1 — number sizes.
//
// Print how many bytes an i32 and an i64 take, using size_of.
// The i32 line is done for you. Add the i64 line the same way.
// Expected output:
//   4
//   8
//
// Run it:  rustc 1-starter.rs && ./1-starter

fn main() {
    println!("{}", std::mem::size_of::<i32>());
    // add a line here for i64
}
