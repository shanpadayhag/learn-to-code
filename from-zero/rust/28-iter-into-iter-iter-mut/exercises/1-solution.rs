fn main() {
    let mut prices = vec![100, 200, 300];

    for p in prices.iter_mut() {
        *p += 5;
    }

    println!("{prices:?}");
}
