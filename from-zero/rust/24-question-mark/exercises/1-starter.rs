// `?` after a Result means: if Ok, unwrap the value and keep going;
// if Err, STOP and return that error from this whole function.
// It only works because `add_two` itself returns a Result for the error
// to travel out through.

fn add_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    // Parse both strings with `?`, then return Ok of their sum:
    //   let x = a.parse::<i32>()?;
    //   let y = b.parse::<i32>()?;
    //   Ok(x + y)
    // your code here
}

fn main() {
    match add_two("20", "22") {
        Ok(total) => println!("total: {total}"),   // should print: total: 42
        Err(_) => println!("couldn't parse"),
    }

    match add_two("20", "oops") {
        Ok(total) => println!("total: {total}"),
        Err(_) => println!("couldn't parse"),        // should print: couldn't parse
    }
}
