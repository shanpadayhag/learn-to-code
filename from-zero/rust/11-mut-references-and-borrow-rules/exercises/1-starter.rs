// A &mut reference lets a function CHANGE a value it doesn't own.
// The value must be `mut`, and both the type and the call wear `&mut`.
fn add_bang(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut text = String::from("hi");

    // Lend text out mutably so add_bang can change it in place.
    // your code here

    println!("{text}");   // should print: hi!
}
