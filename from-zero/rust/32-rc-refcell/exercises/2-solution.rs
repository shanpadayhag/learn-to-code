// A shared list that two owners both append to. The Vec lives in one RefCell,
// shared by many Rc owners.

use std::cell::RefCell;
use std::rc::Rc;

// Push `n` onto the shared list through a shared &Rc<...>.
fn push(list: &Rc<RefCell<Vec<i32>>>, n: i32) {
    list.borrow_mut().push(n);
}

fn main() {
    let list = Rc::new(RefCell::new(Vec::new()));

    // A second owner of the same Vec.
    let writer = Rc::clone(&list);

    push(&list, 1);
    push(&writer, 2);

    // One shared Vec, appended through two owners.
    println!("{:?}", list.borrow()); // [1, 2]
}
