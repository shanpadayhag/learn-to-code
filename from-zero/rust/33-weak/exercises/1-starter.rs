// Weak<T> = a handle that POINTS at a value without OWNING it.
// Rc::downgrade(&rc) makes one; weak.upgrade() -> Option<Rc<T>> to use it.

use std::rc::{Rc, Weak};

fn main() {
    // An owning handle to a String on the heap.
    let owner = Rc::new(String::from("hello"));

    // 1. Downgrade it to a non-owning Weak handle.
    // let peek: Weak<String> = ...;

    // 2. While `owner` is alive, upgrade the Weak and print the value.
    //    (upgrade() gives Some(rc) while the value lives)
    // your code here

    // 3. Drop the last owner, then upgrade again and print the Option.
    //    It should now be None — the value is gone, but safely so.
    // drop(owner);
    // println!("{:?}", ...);
}
