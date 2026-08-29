// A channel is a one-way pipe between threads. send() moves a value in,
// recv() moves it out — ownership travels, so nothing is shared and no lock
// is needed.

use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // `move` hands the only sender to the worker thread.
    thread::spawn(move || {
        for n in 1..=5 {
            sender.send(n).unwrap();
        }
    });
    // The closure ended, so its sender dropped and the channel is now closed.

    let mut total = 0;

    // Blocks between values, then stops once no sender is left alive.
    for received in receiver {
        total += received;
    }

    println!("{}", total); // 15
}
