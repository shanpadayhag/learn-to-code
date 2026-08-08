fn main() {
    // .len() tells you how many bytes the String currently holds.
    // Watch the length change as the text grows on the heap.
    let mut name = String::from("Ann");

    println!("{}", name.len());   // how long now?

    name.push_str("ie");          // Ann -> Annie

    println!("{}", name.len());   // how long now?
}
