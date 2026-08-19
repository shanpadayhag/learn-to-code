use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u64 = input.trim().parse().unwrap();

    println!("{}", if is_palindrome(number) { "Yes" } else { "No" });
}

fn is_palindrome(number: u64) -> bool {
    if number % 10 == 0 && number != 0 {
        return false;
    }

    let mut remaining_number = number;
    let mut reversed_half = 0;
    while remaining_number > reversed_half {
        reversed_half = reversed_half * 10 + remaining_number % 10;
        remaining_number /= 10;
    }

    remaining_number == reversed_half || remaining_number == reversed_half / 10
}
