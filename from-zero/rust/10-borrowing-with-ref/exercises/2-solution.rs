fn size(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Sam");

    let a = size(&name);
    let b = size(&name);

    println!("{name} {a} {b}");
}
