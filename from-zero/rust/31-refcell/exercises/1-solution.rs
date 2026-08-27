// Interior mutability: a RefCell lets you change its contents through a shared
// reference — so the outer binding does NOT need `mut`.

use std::cell::RefCell;

fn main() {
    // Plain `let`, no `mut` — the mutation permission lives inside the RefCell.
    let cell = RefCell::new(10);

    // Take a write handle and change the value through it.
    *cell.borrow_mut() += 7;

    // Take a read handle to see the result.
    println!("{}", cell.borrow()); // 17
}
