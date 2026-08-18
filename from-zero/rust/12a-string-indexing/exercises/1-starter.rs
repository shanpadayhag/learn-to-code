// You can't grab a character with `name[0]` — a string is UTF-8 bytes, and byte 0
// isn't always a whole character. To get the first CHARACTER, walk the characters
// with `.chars()` and take the first: `.chars().next().unwrap()`. Then uppercase it.

fn main() {
    let name = "kai";

    // Print the first character of `name` in uppercase.
    // Hint: name.chars().next().unwrap() gives the first char; .to_uppercase() uppercases it.
    // your code here

    // Expected:
    //   K
}
