// .await is SEQUENTIAL: inside one task it means "stop here until this is done".
// Concurrency comes from the EXECUTOR polling several futures in one loop.
// Same two tasks, same one thread — only the driving loop differs.
//
// Run with:  rustc --edition 2024 2-solution.rs && ./2-solution

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

// The same four lines, holding two futures instead of one. That is the entire
// difference between running a task and running tasks concurrently.
fn run_both<A: Future, B: Future>(first: A, second: B) -> (A::Output, B::Output) {
    let mut first = pin!(first);
    let mut second = pin!(second);
    let mut context = Context::from_waker(Waker::noop());

    // A finished future must never be polled again, so the loop has to
    // remember which ones are still legal to touch.
    let mut first_output = None;
    let mut second_output = None;

    while first_output.is_none() || second_output.is_none() {
        if first_output.is_none() {
            if let Poll::Ready(value) = first.as_mut().poll(&mut context) {
                first_output = Some(value);
            }
        }
        if second_output.is_none() {
            if let Poll::Ready(value) = second.as_mut().poll(&mut context) {
                second_output = Some(value);
            }
        }
    }

    (first_output.unwrap(), second_output.unwrap())
}

async fn steps(label: &str, count: u32) -> u32 {
    let mut done = 0;
    while done < count {
        pause(1).await;
        done += 1;
        println!("  {label} step {done}");
    }
    done
}

async fn one_after_the_other() -> (u32, u32) {
    let toast = steps("toast", 3).await;
    let eggs = steps("eggs", 2).await;
    (toast, eggs)
}

fn main() {
    println!("sequential — .await one, then the other:");
    let (toast, eggs) = block_on(one_after_the_other());
    println!("  toast {toast}, eggs {eggs}");

    println!();
    println!("concurrent — poll both in one loop:");
    let (toast, eggs) = run_both(steps("toast", 3), steps("eggs", 2));
    println!("  toast {toast}, eggs {eggs}");

    println!();
    println!("threads spawned: 0");
}

// sequential — .await one, then the other:
//   toast step 1
//   toast step 2
//   toast step 3
//   eggs step 1
//   eggs step 2
//   toast 3, eggs 2
//
// concurrent — poll both in one loop:
//   toast step 1
//   eggs step 1
//   toast step 2
//   eggs step 2
//   toast step 3
//   toast 3, eggs 2
//
// threads spawned: 0
//
// Drop the `second_output.is_none()` guard and the shorter task (eggs) gets
// polled after it already answered Ready:
//   thread 'main' panicked at 2-solution.rs:
//   `async fn` resumed after completion
//
// It must be that guard: eggs finishes on round 3, toast on round 4, so only
// eggs is still in the loop after it is done.
