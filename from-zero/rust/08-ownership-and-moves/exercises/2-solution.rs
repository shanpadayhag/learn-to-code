fn shout(mut s: String) -> String {
    s.push('!');
    s
}

fn main() {
    let message = String::from("hi");

    let message = shout(message);

    println!("{message}");
}
