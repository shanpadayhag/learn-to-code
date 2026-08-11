fn main() {
    let s = String::from("Rustacean");

    let front = &s[..4];
    let back = &s[4..];

    println!("{front} {back}");
}
