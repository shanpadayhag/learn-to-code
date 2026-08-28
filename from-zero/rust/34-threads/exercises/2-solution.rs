// `move` in front of the closure transfers OWNERSHIP of captured values into
// the thread, so they live as long as the thread does and can never dangle.

use std::thread;

fn main() {
    let message = String::from("owned by the thread now");

    // `move` moves `message` INTO the thread — the owner (ptr/len/cap) crosses
    // to the new stack; the heap text is not copied, just re-owned.
    let handle = thread::spawn(move || {
        println!("{}", message);
    });

    // Wait for the thread to finish before main ends.
    handle.join().unwrap();

    // `message` can't be used here any more — ownership left for the thread.
    // Uncommenting the next line is a compile error (use-after-move):
    // println!("{}", message);
}
