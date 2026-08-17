// fahrenheit = celsius * 9/5 + 32
// Print the result like this:
// [celsius] Celsius = [fahrenheit] Fahrenheit
//
// First attempt — has a bug. Everything is i32, so `9 / 5` is integer
// division and evaluates to 1 (the .8 is thrown away). The formula quietly
// collapses to `celsius * 1 + 32`, i.e. just `celsius + 32`.
//
// Run it:  rustc initial.rs && echo 100 | ./initial
// Prints:  100 Celsius = 132 Fahrenheit   (wrong — should be 212)

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let celsius: i32 = input.trim().parse().unwrap();

    // Calculate fahrenheit
    let fahrenheit: i32 = (celsius * (9 / 5)) + 32;

    // Print the results
    println!("{celsius} Celsius = {fahrenheit} Fahrenheit");
}
