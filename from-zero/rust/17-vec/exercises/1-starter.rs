// A `Vec<T>` is a growable list. Start empty with `Vec::new()`, add to the end
// with `.push(value)`, and hand the whole thing back — the Vec owns its contents.

fn squares(n: u32) -> Vec<u32> {
    // Build a Vec holding 1*1, 2*2, ... up to n*n, then return it.
    // Start with `let mut result = Vec::new();`, push each square, return result.
    // your code here
}

fn main() {
    let s = squares(5);
    println!("{:?}", s);   // should print: [1, 4, 9, 16, 25]

    let mut total = 0;
    for &value in &s {     // borrow the Vec to read it without consuming it
        total += value;
    }
    println!("{total}");   // should print: 55
}
