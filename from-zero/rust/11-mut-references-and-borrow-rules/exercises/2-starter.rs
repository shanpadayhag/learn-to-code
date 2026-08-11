// The change happens to the OWNER's value, so it's visible after the call.
fn append_domain(email: &mut String) {
    email.push_str("@example.com");
}

fn main() {
    let mut user = String::from("sam");

    // Pass a mutable borrow of user.
    // your code here

    println!("{user}");   // should print: sam@example.com
}
