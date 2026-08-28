// thread::spawn starts a SECOND line of execution on its own stack.
// It returns a JoinHandle; .join() waits and hands back what the closure returned.

use std::thread;

fn main() {
    // 1. Spawn a thread whose closure computes 1 + 2 + ... + 10 and RETURNS it.
    // let handle = thread::spawn(|| { ... });

    // 2. .join() the handle (returns a Result), .unwrap() it, and print the value (55).
    //    The returned value comes back THROUGH join.
    // let total = ...;
    // println!("{}", total);
}
