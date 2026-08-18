fn main() {
    let name = "kai";

    let first = name.chars().next().unwrap();
    println!("{}", first.to_uppercase());
}
