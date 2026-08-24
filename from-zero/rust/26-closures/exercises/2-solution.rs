fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let min = 3;

    let count = numbers.into_iter().filter(|&n| n >= min).count();

    println!("{count}");
}
