// `.as_ref()` borrows the inside to READ it (Option<&T>); `.as_mut()` borrows the
// inside to CHANGE it in place (Option<&mut T>). Neither moves the value out, so the
// same Option can be read and then edited.

fn main() {
    let mut count: Option<i32> = Some(41);

    // 1. Use .as_ref() in an `if let Some(n)` to PRINT the value (n is &i32),
    //    without moving it out of `count`.
    // your code here

    // 2. Use .as_mut() in an `if let Some(n)` to add 1 to the value in place
    //    (n is &mut i32 — remember to dereference with *).
    // your code here

    // 3. Print `count` — it should be Some(42), the same Option, edited.
    println!("{count:?}");
}
