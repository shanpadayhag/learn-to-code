// A future is a value you poke until it says it's done. Two answers only:
// Poll::Ready(value) = finished, or Poll::Pending = ask me again.
// An executor is the ordinary, non-async loop that does the poking.
//
// Run with:  rustc --edition 2024 1-starter.rs && ./1-starter

use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

// 1. A future you write by hand. State goes in the fields, the decision goes
//    in poll(). This one pauses `polls_left` times, then finishes.
// struct Pause { polls_left: u32 }
// fn pause(times: u32) -> Pause { Pause { polls_left: times } }

// 2. Implement the trait. The signature is fixed — copy it exactly:
//
// impl Future for Pause {
//     type Output = ();
//     fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<()> {
//         // if the counter is 0 -> Poll::Ready(())
//         // otherwise -> decrement it and return Poll::Pending
//     }
// }
//
//    `mut self: Pin<&mut Self>` is a &mut self carrying one extra promise:
//    this value will not move again. That's why you can write to the field.

// 3. Write the executor, and have it COUNT its polls so you can see the loop.
//
// fn block_on<F: Future>(future: F) -> F::Output {
//     let mut future = pin!(future);                            // park it
//     let mut context = Context::from_waker(Waker::noop());     // a waker that does nothing
//     let mut polls = 0;
//     loop {
//         polls += 1;
//         match future.as_mut().poll(&mut context) {            // .as_mut() re-borrows the pin
//             Poll::Ready(value) => { println!(...); return value; }
//             Poll::Pending => {}
//         }
//     }
// }

// 4. An ordinary async fn that awaits your hand-written future.
// async fn work() -> u32 {
//     pause(3).await;
//     42
// }

fn main() {
    // 5. PREDICT the poll count for pause(3) before you run it. It is not 3.
    //    Work out why a future has to be asked one more time than it pauses.

    // 6. block_on(work()) and print the answer.

    // 7. Try pause(0) and pause(10). Confirm the rule you worked out in step 5.
}
