// The same pattern with a collection instead of a number: several threads
// writing into one shared Vec<String>.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. Share an empty Vec<String> behind an Arc<Mutex<...>>.
    // let reports = Arc::new(Mutex::new(Vec::new()));

    // JoinHandle<()> because each thread's closure returns nothing.
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for worker_id in 1..=3 {
        // 2. One Arc::clone per thread.
        // let reports_handle = Arc::clone(&reports);

        // 3. Each thread locks, pushes format!("worker {} reporting", worker_id),
        //    and lets the guard drop right away so the next thread can go in.
        // handles.push(thread::spawn(move || { ... }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 4. Lock, sort the list (thread order is not guaranteed), and print each line.
    // let mut finished = reports.lock().unwrap();
    // finished.sort();
    // for line in finished.iter() { println!("{}", line); }
}
