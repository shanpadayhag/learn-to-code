fn add_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    match add_two("20", "22") {
        Ok(total) => println!("total: {total}"),
        Err(_) => println!("couldn't parse"),
    }

    match add_two("20", "oops") {
        Ok(total) => println!("total: {total}"),
        Err(_) => println!("couldn't parse"),
    }
}
