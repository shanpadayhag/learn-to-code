// The classic cycle-breaker: a parent OWNS its child (Rc), and the child
// points BACK at the parent with a non-owning Weak. No cycle, no leak.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Weak: does NOT keep the parent alive
    children: RefCell<Vec<Rc<Node>>>, // Rc: the parent owns its children
}

fn main() {
    // A parent with no parent of its own yet (Weak::new() points at nothing).
    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 1. Make a child whose `parent` field is a Weak pointing back at `parent`.
    // let child = ...;

    // 2. Push the child into the parent's `children` (parent owns it via Rc).
    // your code here

    // 3. From the child, upgrade the weak parent link and print the parent value (1).
    // println!("{}", ...);

    // 4. Print Rc::strong_count(&parent) — still 1, because a Weak never bumps it.
    // println!("{}", Rc::strong_count(&parent));
}
