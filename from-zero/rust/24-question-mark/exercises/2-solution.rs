fn add_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() -> Result<(), std::num::ParseIntError> {
    let total = add_two("15", "27")?;
    println!("total: {total}");
    Ok(())
}
