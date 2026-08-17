// fahrenheit = celsius * 9/5 + 32
// Print the result like this:
// [celsius] Celsius = [fahrenheit] Fahrenheit
//
// The task wants the Fahrenheit value shown with a decimal (e.g. 77.0, not 77).
// Two separate fixes are at work here:
//   1. Type:   celsius is f32 and the literals are floats (9.0 / 5.0 = 1.8),
//              so the *math* keeps its decimal instead of truncating to int.
//   2. Format: `{:.1}` prints the number with exactly one digit after the dot,
//              so 77 shows as "77.0". This controls the *display*, not the value.
//
// Run it:  rustc solution.rs && echo 25 | ./solution
// Prints:  25 Celsius = 77.0 Fahrenheit

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let celsius: f32 = input.trim().parse().unwrap();

    // Calculate fahrenheit
    let fahrenheit: f32 = (celsius * (9.0 / 5.0)) + 32.0;

    // Print the results ({:.1} = one digit after the decimal point)
    println!("{} Celsius = {:.1} Fahrenheit", celsius, fahrenheit);
}
