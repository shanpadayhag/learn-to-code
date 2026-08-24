// The standard library hands you Results all the time. Turning text into a
// number can fail (what if the text is "nope"?), so `.parse()` returns a
// `Result<i32, _>`: Ok(the number) when it works, Err(...) when it doesn't.

fn main() {
    // `"20".parse::<i32>()` gives a `Result<i32, ParseIntError>`.
    // `match` on it: print "parsed: {n}" for Ok, "not a number" for Err.
    // your code here            // should print: parsed: 20

    // Do the same for "nope".
    // your code here            // should print: not a number
}
