use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let celsius: f32 = input.trim().parse().unwrap();

    let fahrenheit = (celsius * (9.0 / 5.0)) + 32.0;

    println!("{} Celsius = {:.1} Fahrenheit", celsius, fahrenheit);
}
