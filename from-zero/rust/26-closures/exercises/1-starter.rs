// A closure is `|parameters| body`, and unlike a plain `fn` it can CAPTURE
// variables from the surrounding code.

fn main() {
    let tax_rate = 0.2_f64;

    // Make a closure `with_tax` that takes a price and returns price plus tax:
    //   let with_tax = |price: f64| price + price * tax_rate;
    // It captures `tax_rate` from the line above.
    // your code here

    // println!("{:.1}", with_tax(100.0));   // should print: 120.0
    // println!("{:.1}", with_tax(50.0));    // should print: 60.0
}
