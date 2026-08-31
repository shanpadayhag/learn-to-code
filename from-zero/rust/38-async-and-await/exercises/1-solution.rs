// Calling an async fn runs NOTHING. It builds a value — a paused function —
// and hands it back. Something else has to drive that value to completion.
//
// Run with:  rustc --edition 2024 1-solution.rs && ./1-solution

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

// The executor: ask the future to make progress until it answers Ready.
// Concept 39 explains every line of this.
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

fn brew_sync(beans: &str) -> String {
    println!("  grinding {beans}");
    format!("{beans} coffee")
}

// Identical body. One keyword. Completely different meaning at the call site.
async fn brew(beans: &str) -> String {
    println!("  grinding {beans}");
    format!("{beans} coffee")
}

async fn breakfast() -> String {
    let cup = brew("arabica").await;
    let toast = String::from("toast");
    format!("{cup} and {toast}")
}

fn main() {
    println!("calling the plain function:");
    let drink = brew_sync("robusta");
    println!("  got: {drink}");

    println!("calling the async function:");
    // This builds a value. The body has not run.
    let future = brew("arabica");
    println!("  nothing was ground");

    println!("now driving it:");
    println!("  got: {}", block_on(future));

    println!("the whole breakfast: {}", block_on(breakfast()));
}

// calling the plain function:
//   grinding robusta
//   got: robusta coffee
// calling the async function:
//   nothing was ground          <-- the body never ran
// now driving it:
//   grinding arabica            <-- it runs HERE, at block_on
//   got: arabica coffee
//   grinding arabica
// the whole breakfast: arabica coffee and toast
//
// An unawaited call earns a warning:
//   warning: unused implementer of `Future` that must be used
//     = note: futures do nothing unless you `.await` or poll them
