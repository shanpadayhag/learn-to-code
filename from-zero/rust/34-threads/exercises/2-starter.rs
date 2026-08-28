// `move` in front of the closure transfers OWNERSHIP of captured values into
// the thread, so they live as long as the thread does and can never dangle.

use std::thread;

fn main() {
    let message = String::from("owned by the thread now");

    // 1. Spawn a thread with a `move` closure that takes ownership of `message`
    //    and prints it. (Without `move` this won't compile — the closure would
    //    borrow main's frame, which may vanish first.)
    // let handle = thread::spawn(...);

    // 2. .join() to wait for it to finish.
    // handle.join().unwrap();

    // 3. Try to use `message` here and see why it no longer compiles:
    //    println!("{}", message); // ERROR: value moved into the thread
}
