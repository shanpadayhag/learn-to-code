fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let total: i32 = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n * 3)
        .sum();

    println!("{total}");
}
