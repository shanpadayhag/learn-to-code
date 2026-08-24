// The exact shape from the Longest Common Prefix challenge:
// `&[&'a str]` = a borrowed list of borrowed strings, all valid for 'a;
// `-> &'a str` = the returned slice borrows from that same text.

// Return the first word, or "" if the list is empty.
fn first<'a>(words: &[&'a str]) -> &'a str {
    // if words.first() is Some(word), return word; otherwise return ""
    // your code here
}

fn main() {
    let words = ["flower", "flow", "flight"];
    println!("{}", first(&words));      // should print: flower

    let empty: [&str; 0] = [];
    println!("[{}]", first(&empty));    // should print: []
}
