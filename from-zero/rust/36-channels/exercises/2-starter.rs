// mpsc = multiple producer, single consumer. Clone the sender to give every
// producer thread its own sending end.

use std::sync::mpsc;
use std::thread;

fn main() {
    // 1. Make the channel.
    // let (sender, receiver) = mpsc::channel();

    for worker_id in 1..=3 {
        // 2. Clone the sender for this thread, then `move` the clone in and
        //    send format!("worker {} done", worker_id).
        // let worker_sender = sender.clone();
        // thread::spawn(move || { ... });
    }

    // 3. Drop main's own sender. Without this the loop below never ends:
    //    a live sender means "more messages may still come".
    // drop(sender);

    // 4. Collect every message into a Vec, sort it (thread order is not
    //    guaranteed), and print each line.
    // let mut messages: Vec<String> = receiver.iter().collect();
    // messages.sort();
    // for message in messages { println!("{}", message); }
}
