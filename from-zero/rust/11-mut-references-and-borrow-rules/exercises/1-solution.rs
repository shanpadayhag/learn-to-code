fn add_bang(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut text = String::from("hi");

    add_bang(&mut text);

    println!("{text}");
}
