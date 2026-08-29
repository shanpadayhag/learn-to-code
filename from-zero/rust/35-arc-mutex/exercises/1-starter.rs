// Arc<Mutex<T>> = many threads own one value (Arc) and take turns changing it (Mutex).
// It is the thread-safe twin of Rc<RefCell<T>>.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. Make the shared counter: an Arc wrapping a Mutex wrapping 0.
    // let counter = Arc::new(Mutex::new(0));

    // JoinHandle<()> because each thread's closure returns nothing.
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for _ in 0..4 {
        // 2. Clone the Arc BEFORE the closure — one extra owner per thread.
        //    (Without the clone, `move` would consume `counter` on the first pass.)
        // let counter_handle = Arc::clone(&counter);

        // 3. Spawn a thread with a `move` closure that locks the mutex and
        //    adds 1, ten times. `.lock().unwrap()` hands back a guard you use
        //    like a &mut; the lock frees when the guard drops.
        // handles.push(thread::spawn(move || { ... }));
    }

    // 4. Join every handle so all four threads have finished.
    for handle in handles {
        handle.join().unwrap();
    }

    // 5. Lock once more and print the total — it is always 40.
    // println!("{}", *counter.lock().unwrap());
}
