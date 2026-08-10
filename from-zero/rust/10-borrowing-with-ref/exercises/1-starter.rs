// This function only READS the string, so it borrows it (&String)
// instead of taking ownership.
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let text = String::from("hello");

    // Call length WITHOUT giving `text` away: pass a reference with &.
    // your code here

    // Because we only borrowed, text is still ours here:
    println!("{text} {n}");   // should print: hello 5
}
