fn main() {
    // Two clones are truly independent: changing one leaves the other alone,
    // because each owns its own buffer on the heap.
    let original = String::from("cat");
    let mut copy = original.clone();

    // Add an "s" to `copy` only.
    // your code here

    println!("{original} {copy}");   // should print: cat cats
}
