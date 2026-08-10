fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let text = String::from("hello");

    let n = length(&text);

    println!("{text} {n}");
}
