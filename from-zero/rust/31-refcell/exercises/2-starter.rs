// A value can be changed through shared `&` references when it lives in a RefCell.
// Write a function that bumps a counter it only borrows shared-ly.

use std::cell::RefCell;

// Takes a SHARED &RefCell<i32> yet still mutates the value inside.
fn bump(counter: &RefCell<i32>) {
    // Add 1 to the value inside `counter` using .borrow_mut().
    // your code here
}

fn main() {
    let cell = RefCell::new(0);

    // Call bump three times, each time passing a shared &cell.
    // your code here

    // Print the final count (should be 3).
    println!("{}", cell.borrow());
}
