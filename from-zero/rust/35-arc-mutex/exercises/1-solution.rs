// Arc<Mutex<T>> = many threads own one value (Arc) and take turns changing it (Mutex).
// It is the thread-safe twin of Rc<RefCell<T>>.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        // One extra owner per thread: a cheap count bump, not a copy of the value.
        let counter_handle = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                // .lock() waits until it is this thread's turn, then hands back
                // a guard. The guard drops at the end of each pass, unlocking.
                let mut value = counter_handle.lock().unwrap();
                *value += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Four threads x ten increments, never lost to a race.
    println!("{}", *counter.lock().unwrap()); // 40
}
