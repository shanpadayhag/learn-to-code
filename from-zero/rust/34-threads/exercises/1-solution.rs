// thread::spawn starts a SECOND line of execution on its own stack.
// It returns a JoinHandle; .join() waits and hands back what the closure returned.

use std::thread;

fn main() {
    // The new thread runs this closure and returns the sum.
    let handle = thread::spawn(|| {
        let mut total = 0;
        for n in 1..=10 {
            total += n;
        }
        total
    });

    // .join() pauses main until the thread finishes; the return value comes
    // back inside a Result. .unwrap() opens the Ok.
    let total = handle.join().unwrap();
    println!("{}", total); // 55
}
