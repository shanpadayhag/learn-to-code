// An `Option<&str>` is EITHER `Some(a name)` OR `None` (no name given).
// The compiler won't let you use the name without opening the Option first.

fn greet(name: Option<&str>) {
    // `match` on `name`. Handle both cases:
    //   Some(actual_name) => print "Hello, {actual_name}!"
    //   None              => print "Hello, stranger!"
    // your code here
}

fn main() {
    greet(Some("Ada"));   // should print: Hello, Ada!
    greet(None);          // should print: Hello, stranger!
}
