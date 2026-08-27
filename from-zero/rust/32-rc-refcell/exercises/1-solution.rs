// Rc<RefCell<T>> = many owners who can all CHANGE one shared value.
// Rc::clone to make owners; .borrow_mut() to edit through any of them.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // One shared counter at 0.
    let counter = Rc::new(RefCell::new(0));

    // A second owner of the SAME RefCell — cheap count bump, not a copy.
    let other = Rc::clone(&counter);

    // Edit through each owner; both changes land in the one shared cell.
    *counter.borrow_mut() += 1;
    *other.borrow_mut() += 1;

    // The original sees every change.
    println!("{}", counter.borrow()); // 2
}
