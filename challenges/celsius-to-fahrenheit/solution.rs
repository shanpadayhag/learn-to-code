use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let celsius_temperature: f32 = input_line.trim().parse().unwrap();

    let fahrenheit_temperature = (celsius_temperature * (9.0 / 5.0)) + 32.0;

    println!(
        "{} Celsius = {:.1} Fahrenheit",
        celsius_temperature, fahrenheit_temperature
    );
}
