// Calling an async fn runs NOTHING. It builds a value — a paused function —
// and hands it back. Something else has to drive that value to completion.
//
// Run with:  rustc --edition 2024 1-starter.rs && ./1-starter
// (a bare `rustc` defaults to the 2015 edition, where `async` isn't a keyword)

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

// The executor. Treat it as a black box for now — concept 39 opens it up and
// you write it yourself. All it does is ask the future to make progress, over
// and over, until it answers Ready.
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

// 1. Write the plain version. Print "  grinding {beans}" then return
//    format!("{beans} coffee").
// fn brew_sync(beans: &str) -> String { ... }

// 2. Write the SAME body again with `async` in front. Change nothing else.
// async fn brew(beans: &str) -> String { ... }

// 3. Write an async fn `breakfast` that awaits brew("arabica"), makes a
//    `toast` String, and returns "{cup} and {toast}".
//    Remember: `.await` is a suffix — brew("arabica").await
// async fn breakfast() -> String { ... }

fn main() {
    // 4. Call brew_sync and watch it grind immediately.

    // 5. Call brew("arabica") into a variable. Print a line AFTER it saying
    //    nothing was ground. Read your own output: the grinding line is missing.

    // 6. Now println!("{}", block_on(future)) and watch the grinding happen
    //    late — at the moment it is driven, not the moment it was called.

    // 7. Do the same with block_on(breakfast()).

    // 8. Finally, add a bare `brew("decaf");` line on its own and compile.
    //    Read the whole warning, including the note. Then delete it.
}
