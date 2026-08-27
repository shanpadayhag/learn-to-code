// Rc<RefCell<T>> = many owners who can all CHANGE one shared value.
// Rc::clone to make owners; .borrow_mut() to edit through any of them.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // One shared counter at 0.
    let counter = Rc::new(RefCell::new(0));

    // 1. Make a second owner of the SAME RefCell with Rc::clone.
    // let other = ...;

    // 2. Add 1 through `counter`, and add 1 through `other`.
    //    (auto-dereference reaches through the Rc to call .borrow_mut())
    // your code here

    // 3. Print the value via the original (should be 2).
    println!("{}", counter.borrow());
}
