use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number = input.trim().to_string();

    let mut reversed = String::new();
    for character in number.chars().rev() {
        reversed.push(character);
    }

    if number == reversed {
        println!("Yes");
    } else {
        println!("No");
    }
}
