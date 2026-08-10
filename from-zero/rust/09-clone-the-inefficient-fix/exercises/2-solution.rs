fn main() {
    let original = String::from("cat");
    let mut copy = original.clone();

    copy.push_str("s");

    println!("{original} {copy}");
}
