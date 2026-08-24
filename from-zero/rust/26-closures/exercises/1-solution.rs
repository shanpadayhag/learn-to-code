fn main() {
    let tax_rate = 0.2_f64;

    let with_tax = |price: f64| price + price * tax_rate;

    println!("{:.1}", with_tax(100.0));
    println!("{:.1}", with_tax(50.0));
}
