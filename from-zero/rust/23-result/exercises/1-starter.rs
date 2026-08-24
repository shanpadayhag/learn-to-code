// A `Result<i32, String>` is EITHER `Ok(a number)` OR `Err(a reason it failed)`.
// Unlike Option's empty `None`, the `Err` side CARRIES the explanation.
// The compiler won't let you use the number without opening the Result first.

fn half(number: i32) -> Result<i32, String> {
    // If `number` is odd (number % 2 != 0), return an Err with a message like
    //   format!("{number} is odd, can't halve it evenly")
    // Otherwise return Ok(number / 2).
    // your code here
}

fn main() {
    match half(8) {
        Ok(value) => println!("worked: {value}"),   // should print: worked: 4
        Err(reason) => println!("failed: {reason}"),
    }

    match half(7) {
        Ok(value) => println!("worked: {value}"),
        Err(reason) => println!("failed: {reason}"), // should print: failed: 7 is odd, ...
    }
}
