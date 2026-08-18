use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 88);
    scores.insert("Bob", 92);
    scores.insert("Dana", 75);

    for name in ["Alice", "Carol"] {
        match scores.get(name) {
            Some(score) => println!("{name}: {score}"),
            None => println!("no score for {name}"),
        }
    }
}
