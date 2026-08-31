// .await is SEQUENTIAL: inside one task it means "stop here until this is done".
// Concurrency comes from the EXECUTOR polling several futures in one loop.
// Same two tasks, same one thread — only the driving loop differs.
//
// Run with:  rustc --edition 2024 2-starter.rs && ./2-starter

use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

struct Pause { polls_left: u32 }
fn pause(times: u32) -> Pause { Pause { polls_left: times } }

impl Future for Pause {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<()> {
        if self.polls_left == 0 { Poll::Ready(()) } else { self.polls_left -= 1; Poll::Pending }
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => {}
        }
    }
}

// 1. A task that announces each step, pausing between them.
// async fn steps(label: &str, count: u32) -> u32 {
//     let mut done = 0;
//     while done < count {
//         pause(1).await;
//         done += 1;
//         println!("  {label} step {done}");
//     }
//     done
// }

// 2. The sequential version: one task awaiting both, one after the other.
// async fn one_after_the_other() -> (u32, u32) { ... }

// 3. The concurrent version: ONE loop holding TWO futures. Pin both, then
//    poll each one per round — but only while it hasn't finished yet.
//
// fn run_both<A: Future, B: Future>(first: A, second: B) -> (A::Output, B::Output) {
//     let mut first = pin!(first);
//     let mut second = pin!(second);
//     let mut context = Context::from_waker(Waker::noop());
//     let mut first_output = None;
//     let mut second_output = None;
//     while first_output.is_none() || second_output.is_none() {
//         // poll first if first_output.is_none(), store Some(value) on Ready
//         // poll second the same way
//     }
//     (first_output.unwrap(), second_output.unwrap())
// }
//
//    The Option per task is doing real work: a future must NEVER be polled
//    again after it answered Ready.

fn main() {
    // 4. Run steps("toast", 3) and steps("eggs", 2) both ways and put the two
    //    outputs side by side. Predict each order before you look.

    // 5. Count the threads you spawned. (Zero.)

    // 6. Now delete the `second_output.is_none()` guard around the SECOND
    //    poll and run it again. It has to be that one: eggs is the shorter
    //    task, so it finishes while the loop is still running for toast.
    //    Read the panic message. That message is why the Option is there.
}
