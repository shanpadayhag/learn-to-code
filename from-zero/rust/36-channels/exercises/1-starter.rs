// A channel is a one-way pipe between threads. send() moves a value in,
// recv() moves it out — ownership travels, so nothing is shared and no lock
// is needed.

use std::sync::mpsc;
use std::thread;

fn main() {
    // 1. Make the channel. It hands back a pair: the sending end and the
    //    receiving end.
    // let (sender, receiver) = mpsc::channel();

    // 2. Spawn a thread with a `move` closure that takes the sender and sends
    //    the numbers 1 through 5 into it. .send() returns a Result.
    // thread::spawn(move || { ... });

    // 3. Loop over the receiver, adding up what arrives, and print the total (15).
    //    The loop ends by itself when the last sender drops — which is why no
    //    .join() is needed here.
    // let mut total = 0;
    // for received in receiver { ... }
    // println!("{}", total);
}
