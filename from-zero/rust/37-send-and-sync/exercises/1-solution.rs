// `Send` = this value may MOVE to another thread. It is an auto trait: the
// compiler grants it to your struct only if every field has it. One `Rc` field
// takes it away from the whole struct.

use std::mem;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Report {
    title: String,
    // Was Rc<Vec<String>>, which is not Send — so `Report` was not Send either,
    // and thread::spawn refused the closure that captured it.
    lines: Arc<Vec<String>>,
}

fn main() {
    let shared_lines = Arc::new(vec![String::from("disk ok"), String::from("network ok")]);

    let mut handles = Vec::new();

    for worker_id in 1..=2 {
        // One handle per thread — the same move as Arc::clone in concept 35.
        let report = Report {
            title: format!("worker {}", worker_id),
            lines: Arc::clone(&shared_lines),
        };

        handles.push(thread::spawn(move || {
            println!("{}: {} lines", report.title, report.lines.len());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Identical size, identical shape. The fix was about traits, not layout.
    println!("Rc<Vec<String>>  is {} bytes", mem::size_of::<Rc<Vec<String>>>());
    println!("Arc<Vec<String>> is {} bytes", mem::size_of::<Arc<Vec<String>>>());
}
