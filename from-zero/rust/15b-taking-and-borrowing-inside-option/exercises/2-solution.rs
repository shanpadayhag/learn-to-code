// `.as_ref()` borrows the inside to READ it (Option<&T>); `.as_mut()` borrows the
// inside to CHANGE it in place (Option<&mut T>). Neither moves the value out.

fn main() {
    let mut count: Option<i32> = Some(41);

    // 1. Read the inside without moving it — `n` is a &i32 borrowed from `count`.
    if let Some(n) = count.as_ref() {
        println!("before: {n}"); // before: 41
    }

    // 2. Edit the inside in place — `n` is a &mut i32 pointing into `count`.
    if let Some(n) = count.as_mut() {
        *n += 1;
    }

    // 3. Same Option, now holding 42.
    println!("{count:?}"); // Some(42)
}
