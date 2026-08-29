// The same pattern with a collection instead of a number: several threads
// writing into one shared Vec<String>.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let reports = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for worker_id in 1..=3 {
        let reports_handle = Arc::clone(&reports);

        handles.push(thread::spawn(move || {
            let line = format!("worker {} reporting", worker_id);

            // Braces keep the lock held only for the push itself.
            {
                let mut reports = reports_handle.lock().unwrap();
                reports.push(line);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Threads finish in any order, so sort to make the output stable.
    let mut finished = reports.lock().unwrap();
    finished.sort();

    for line in finished.iter() {
        println!("{}", line);
    }
}
