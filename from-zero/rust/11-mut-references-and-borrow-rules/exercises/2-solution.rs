fn append_domain(email: &mut String) {
    email.push_str("@example.com");
}

fn main() {
    let mut user = String::from("sam");

    append_domain(&mut user);

    println!("{user}");
}
