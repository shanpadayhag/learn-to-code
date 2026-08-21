fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn main() {
    let n = 5;
    println!("{n}! = {}", factorial(n));
}
