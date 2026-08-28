// Weak<T> = a handle that POINTS at a value without OWNING it.
// Rc::downgrade(&rc) makes one; weak.upgrade() -> Option<Rc<T>> to use it.

use std::rc::{Rc, Weak};

fn main() {
    // An owning handle to a String on the heap.
    let owner = Rc::new(String::from("hello"));

    // A non-owning handle — it does NOT bump the strong (owner) count.
    let peek: Weak<String> = Rc::downgrade(&owner);

    // While an owner is alive, upgrade succeeds and hands back a real Rc.
    if let Some(alive) = peek.upgrade() {
        println!("still here: {}", alive); // still here: hello
    }

    // Drop the last OWNER. The Weak never counted, so the value is really freed.
    drop(owner);

    // Now upgrade fails safely with None instead of a dangling pointer.
    println!("{:?}", peek.upgrade()); // None
}
