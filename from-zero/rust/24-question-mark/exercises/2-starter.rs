// `main` is allowed to return a Result — which is exactly what lets you use
// `?` at the top level. `Ok(())` at the end means "finished, nothing to report"
// (`()` is the empty value).

fn add_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() -> Result<(), std::num::ParseIntError> {
    // Call add_two("15", "27") with `?` to get the total, print it, then Ok(()).
    //   let total = add_two("15", "27")?;
    //   println!("total: {total}");
    //   Ok(())
    // your code here
}
