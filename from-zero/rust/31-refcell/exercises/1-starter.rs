// Interior mutability: a RefCell lets you change its contents through a shared
// reference — so the outer binding does NOT need `mut`. Prove it.

use std::cell::RefCell;

fn main() {
    // Note: plain `let`, no `mut`.
    let cell = RefCell::new(10);

    // 1. Use .borrow_mut() to add 7 to the value inside (remember to dereference with *).
    // your code here

    // 2. Use .borrow() to print the new value (should be 17).
    // your code here
}
