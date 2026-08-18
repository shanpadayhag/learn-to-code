use std::collections::HashMap;

fn main() {
    let sentence = "the cat sat on the mat";

    let mut counts = HashMap::new();
    for word in sentence.split(' ') {
        *counts.entry(word).or_insert(0) += 1;
    }

    for (word, count) in &counts {
        println!("{word}: {count}");
    }
}
