fn main() {
    let x = 42;
    let r = &x;

    let value = *r;
    println!("{}", value);
}
