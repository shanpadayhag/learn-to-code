// A value can be changed through shared `&` references when it lives in a RefCell.

use std::cell::RefCell;

// Takes a SHARED &RefCell<i32> yet still mutates the value inside — interior mutability.
fn bump(counter: &RefCell<i32>) {
    *counter.borrow_mut() += 1;
}

fn main() {
    let cell = RefCell::new(0);

    bump(&cell);
    bump(&cell);
    bump(&cell);

    println!("{}", cell.borrow()); // 3
}
