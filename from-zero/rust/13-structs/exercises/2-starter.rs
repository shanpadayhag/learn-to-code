// To change a field, the whole struct value must be `mut`.
struct Counter {
    count: i32,
}

fn main() {
    let mut c = Counter { count: 0 };

    // Add 5, then add 1, to c.count.
    // your code here

    println!("{}", c.count);   // should print: 6
}
