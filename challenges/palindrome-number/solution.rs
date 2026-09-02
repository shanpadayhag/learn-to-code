use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let candidate_number: u64 = input_line.trim().parse().unwrap();

    println!("{}", if is_palindrome(candidate_number) { "Yes" } else { "No" });
}

fn is_palindrome(candidate_number: u64) -> bool {
    if candidate_number % 10 == 0 && candidate_number != 0 {
        return false;
    }

    let mut remaining_front_half = candidate_number;
    let mut reversed_back_half = 0;
    while remaining_front_half > reversed_back_half {
        reversed_back_half = reversed_back_half * 10 + remaining_front_half % 10;
        remaining_front_half /= 10;
    }

    remaining_front_half == reversed_back_half || remaining_front_half == reversed_back_half / 10
}
