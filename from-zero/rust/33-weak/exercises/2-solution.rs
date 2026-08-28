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

    // The child points BACK at the parent with a Weak — a view, not ownership.
    let child = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Rc::downgrade(&parent)),
        children: RefCell::new(vec![]),
    });

    // The parent OWNS the child with an Rc.
    parent.children.borrow_mut().push(Rc::clone(&child));

    // Walk up from the child: upgrade the Weak to reach the living parent.
    let up = child.parent.borrow().upgrade();
    println!("{}", up.unwrap().value); // 1

    // Still 1: the child's Weak back-link never bumped the parent's owner count.
    println!("{}", Rc::strong_count(&parent)); // 1
}
