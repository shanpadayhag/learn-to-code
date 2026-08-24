fn first<'a>(words: &[&'a str]) -> &'a str {
    if let Some(word) = words.first() {
        word
    } else {
        ""
    }
}

fn main() {
    let words = ["flower", "flow", "flight"];
    println!("{}", first(&words));

    let empty: [&str; 0] = [];
    println!("[{}]", first(&empty));
}
