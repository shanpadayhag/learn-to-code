fn greet(name: Option<&str>) {
    match name {
        Some(actual_name) => println!("Hello, {actual_name}!"),
        None => println!("Hello, stranger!"),
    }
}

fn main() {
    greet(Some("Ada"));
    greet(None);
}
