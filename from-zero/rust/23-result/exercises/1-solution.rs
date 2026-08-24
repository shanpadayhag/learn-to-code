fn half(number: i32) -> Result<i32, String> {
    if number % 2 != 0 {
        return Err(format!("{number} is odd, can't halve it evenly"));
    }
    Ok(number / 2)
}

fn main() {
    match half(8) {
        Ok(value) => println!("worked: {value}"),
        Err(reason) => println!("failed: {reason}"),
    }

    match half(7) {
        Ok(value) => println!("worked: {value}"),
        Err(reason) => println!("failed: {reason}"),
    }
}
