fn add_ten(mut n: i32) -> i32 {
    n = n + 10;   // this changes the function's OWN copy
    n
}

fn main() {
    let score = 5;
    let bigger = add_ten(score);

    // Print score, then bigger.
    // Guess first: did calling add_ten change score back in main?
    // your code here
}
