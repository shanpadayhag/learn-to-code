// mpsc = multiple producer, single consumer. Clone the sender to give every
// producer thread its own sending end.

use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    for worker_id in 1..=3 {
        // One sending end per thread; the channel counts how many are alive.
        let worker_sender = sender.clone();

        thread::spawn(move || {
            worker_sender.send(format!("worker {} done", worker_id)).unwrap();
        });
    }

    // main still holds the original sender. Drop it, or the count of live
    // senders never reaches zero and the loop below waits forever.
    drop(sender);

    // .iter() on a Receiver yields values until the channel closes.
    let mut messages: Vec<String> = receiver.iter().collect();
    messages.sort();

    for message in messages {
        println!("{}", message);
    }
}
