// A shared list that two owners both append to. The Vec lives in one RefCell,
// shared by many Rc owners.

use std::cell::RefCell;
use std::rc::Rc;

// Push `n` onto the shared list through a shared &Rc<...>.
fn push(list: &Rc<RefCell<Vec<i32>>>, n: i32) {
    // Use .borrow_mut() to get the Vec, then .push(n).
    // your code here
}

fn main() {
    let list = Rc::new(RefCell::new(Vec::new()));

    // 1. Make a second owner with Rc::clone.
    // let writer = ...;

    // 2. push 1 through `list`, push 2 through `writer`.
    // your code here

    // 3. Print the final vector (should be [1, 2]).
    println!("{:?}", list.borrow());
}
