fn main() {
    match "20".parse::<i32>() {
        Ok(number) => println!("parsed: {number}"),
        Err(_) => println!("not a number"),
    }

    match "nope".parse::<i32>() {
        Ok(number) => println!("parsed: {number}"),
        Err(_) => println!("not a number"),
    }
}
