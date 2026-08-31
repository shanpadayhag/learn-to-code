// A future is a value you poke until it says it's done. Two answers only:
// Poll::Ready(value) = finished, or Poll::Pending = ask me again.
// An executor is the ordinary, non-async loop that does the poking.
//
// Run with:  rustc --edition 2024 1-solution.rs && ./1-solution

use std::future::Future;
use std::pin::{pin, Pin};
use std::task::{Context, Poll, Waker};

// The hand-written version of what `async` generates for you:
// state in the fields, the decision in poll().
struct Pause {
    polls_left: u32,
}

fn pause(times: u32) -> Pause {
    Pause { polls_left: times }
}

impl Future for Pause {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<()> {
        if self.polls_left == 0 {
            Poll::Ready(())
        } else {
            self.polls_left -= 1;
            Poll::Pending
        }
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    // Park the future at one address. It may never move again — its fields
    // can point at each other, so moving it would leave those dangling.
    let mut future = pin!(future);

    // A waker whose wake() does nothing. This executor has nowhere to sleep,
    // so on Pending its only option is to ask again.
    let mut context = Context::from_waker(Waker::noop());

    let mut polls = 0;
    loop {
        polls += 1;
        // .as_mut() re-borrows the pin, so the loop can poll more than once.
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => {
                println!("  finished after {polls} polls");
                return value;
            }
            Poll::Pending => {}
        }
    }
}

async fn work(pauses: u32) -> u32 {
    pause(pauses).await;
    42
}

fn main() {
    for pauses in [0, 3, 10] {
        println!("pause({pauses}):");
        let answer = block_on(work(pauses));
        println!("  answer {answer}");
    }
}

// pause(0):
//   finished after 1 polls
//   answer 42
// pause(3):
//   finished after 4 polls
//   answer 42
// pause(10):
//   finished after 11 polls
//   answer 42
//
// The count is always pauses + 1: a future has to be asked ONE more time to
// report that it is finished. Every poll loop has this off-by-one.
