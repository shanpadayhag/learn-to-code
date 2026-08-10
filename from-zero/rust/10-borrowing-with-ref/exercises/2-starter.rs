// A borrow doesn't consume the value, so you can lend it out
// as many times as you like and still own it at the end.
fn size(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Sam");

    // Borrow `name` twice, into `a` then `b`, then print all three.
    // your code here

    println!("{name} {a} {b}");   // should print: Sam 3 3
}
