// An `if`/`else` is an expression: it hands back the value of whichever arm runs.
// So you can store it straight into a variable — no `mut`, and no assigning inside
// each branch. Both arms must produce the same type, and the `else` is required when
// you use the `if` for its value.

fn main() {
    let n = 7;

    // Store "Even" or "Odd" into `parity` using a single `let parity = if ...`:
    // your code here

    println!("{parity}");
    // Expected:
    //   Odd
}
