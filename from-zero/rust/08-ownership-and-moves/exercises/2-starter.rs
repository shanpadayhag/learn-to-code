// Passing a String to a function MOVES it in. To keep using it,
// the function hands ownership back by returning it.
fn shout(mut s: String) -> String {
    s.push('!');
    s               // give ownership back to the caller
}

fn main() {
    let message = String::from("hi");

    // Call shout, and catch the returned String back into `message`.
    // your code here

    println!("{message}");   // should print: hi!
}
