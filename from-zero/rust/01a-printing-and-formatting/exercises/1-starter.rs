// Exercise 1 — same value, two labels.
//
// Store one price (a float), then print it TWICE from the same variable:
//   - first plainly
//   - then forced to exactly two decimal places
// The value never changes — only the text does.
//
// Expected output:
//   7
//   7.00
//
// Run it:  rustc 1-starter.rs && ./1-starter

fn main() {
    let price = 7.0;
    println!("{price}");
    // add a second line that prints `price` with two decimal places
}
