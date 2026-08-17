fn main() {
    let scores = vec![88, 92, 79];

    for index in [1, 5] {
        match scores.get(index) {
            Some(score) => println!("score at {index}: {score}"),
            None => println!("no score at index {index}"),
        }
    }
}
