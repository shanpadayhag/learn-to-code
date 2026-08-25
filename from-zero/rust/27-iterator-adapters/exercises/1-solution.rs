fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let squares: Vec<i32> = numbers.iter().map(|n| n * n).collect();

    println!("{squares:?}");
}
