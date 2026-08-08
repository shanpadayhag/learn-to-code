fn add_ten(mut n: i32) -> i32 {
    n = n + 10;
    n
}

fn main() {
    let score = 5;
    let bigger = add_ten(score);

    println!("{score}");
    println!("{bigger}");
}
