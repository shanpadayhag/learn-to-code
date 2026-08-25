fn main() {
    let words = vec![String::from("a"), String::from("b"), String::from("c")];

    let shouts: Vec<String> = words.into_iter().map(|w| w.to_uppercase()).collect();

    println!("{shouts:?}");
}
