# Concept 38 · `async` and `.await` (a function that can pause) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 37](../37-send-and-sync/use-it.md)

## The idea
Every function you have written so far shares one property you have never had a reason to notice:
**once it starts, it runs to the end.** It can't stop halfway, hand control back to whoever called
it, and be resumed later. Its locals live in a [stack frame](../04-functions-and-the-call-stack/use-it.md),
and that frame exists only while the function is running.

That's fine until a function has to *wait* — for a file, a network reply, a lock. In
[Phase 9](../34-threads/use-it.md) the answer was a thread: if a function is going to sit there
blocked, put it on its own stack so the rest of the program keeps moving. It works, and it's
expensive. Rust gives each spawned thread a **2 MiB stack**, and a thread waiting on a reply is
holding all of it while doing nothing at all.

`async` is the other answer. An **`async fn` doesn't run when you call it.** Calling it builds a
**value** — a struct holding the function's locals and a marker for how far it got — and hands it to
you. Nothing has happened yet. That value is a *paused function*, and because it's a value you can
put it in a variable, move it, store a thousand of them in a `Vec`, and resume any one of them
whenever you like.

That is the whole concept. A frame that must run to completion has become a struct you can hold.

![Three panels: a plain function's stack frame born and destroyed; an async function call producing a struct holding a state tag and saved locals, polled twice to Ready; and three nested futures flattened into one 4-byte struct](diagrams/paused-function.svg)

## Calling it runs nothing
Take a function with a visible side effect and write it both ways:

```rust
fn brew_sync(beans: &str) -> String {
    println!("  grinding {beans}");
    format!("{beans} coffee")
}

async fn brew(beans: &str) -> String {
    println!("  grinding {beans}");
    format!("{beans} coffee")
}
```

The bodies are identical. One word differs. Now call each:

```rust
let drink = brew_sync("robusta");   // prints "grinding robusta"
let future = brew("arabica");       // prints NOTHING
```

The second line ran none of the body. It built a value and stopped. `future` isn't a `String` — it's
a paused `brew`, sitting in `main`'s frame with `beans` saved inside it, waiting for someone to ask
it to make progress.

The compiler will even warn you if you forget to ask:

```
warning: unused implementer of `Future` that must be used
  = note: futures do nothing unless you `.await` or poll them
```

**"futures do nothing"** is not a figure of speech. Nothing runs until something drives the value.

## `.await` — resume that value, right here
Inside another `async fn`, `.await` means: *drive this paused function until it produces its answer,
and while it can't make progress, pause me too.*

```rust
async fn breakfast() -> String {
    let cup = brew("arabica").await;      // drive brew to completion, take the String
    let toast = String::from("toast");
    format!("{cup} and {toast}")
}
```

Three things to notice, because each one surprises people:

1. **`.await` is a suffix**, written after the value like a field — `brew(..).await`, not
   `await brew(..)`. It chains cleanly with `?` and method calls, which is exactly why it was
   designed that way.
2. **It doesn't spawn anything.** `breakfast` waits for `brew`, in order, on the same thread. `async`
   by itself buys you *pausing*, not parallelism — doing two things at once needs one more piece,
   and that's [Concept 39](../39-future-poll-and-the-executor/use-it.md).
3. **`.await` only exists inside `async`.** Write it in `main` and you get:

```
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:3:20
  |
2 | fn main() {
  | --------- this is not `async`
```

Which raises the obvious question.

## Someone has to drive it
If an async function only pauses, and `.await` only pauses the *caller*, then at the very bottom
something ordinary and non-async has to sit in a loop and push the whole stack of paused functions
forward. That thing is called an **executor**, and every async program has exactly one at its root.

For this lesson, take this four-line executor as a black box — [Concept 39](../39-future-poll-and-the-executor/use-it.md)
opens it up and you write it yourself:

```rust
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

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
```

With it, the whole thing runs:

```rust
fn main() {
    let future = brew("arabica");             // nothing ground
    println!("{}", block_on(future));         // "grinding arabica", then "arabica coffee"
}
```

Real programs don't hand-roll this — they reach for a runtime crate (`tokio` is the common one) and
write `#[tokio::main] async fn main()`, which is the same idea with a great deal more machinery
behind it. The shape never changes: **a plain function at the bottom, driving a tree of paused ones.**

## The locals become fields, and you can measure it
Here is the claim made checkable. The future *is* a struct, so `size_of_val` will tell you how big
your paused function is:

```rust
async fn nothing() {}
async fn level_a() -> u64 { nothing().await; 1 }
async fn level_b() -> u64 { level_a().await + 1 }
async fn level_c() -> u64 { level_b().await + 1 }
```

```text
nothing()     1 byte
level_a()     2 bytes
level_b()     3 bytes
level_c()     4 bytes
```

Each `.await` nests the inner future **inside** the outer one, plus one byte to record which state
it's in. Three levels of calls collapse into one flat 4-byte struct — no stack, no allocation, size
known at compile time. That is why a program can hold a million paused tasks and a thread-per-task
program cannot.

And only what is *still alive at the pause* costs you anything:

```rust
async fn scoped_then_await() -> u64 {
    let total = { let big = [0u8; 512]; big.len() as u64 };   // big dies here
    nothing().await;
    total
}

async fn held_across_await() -> u64 {
    let big = [0u8; 512];
    nothing().await;                                          // big must survive this
    big.len() as u64
}
```

```text
scoped_then_await()    16 bytes
held_across_await()   514 bytes
```

Same array, same work. The second one has to carry 512 bytes across the pause, so those bytes are a
permanent field of the future. This is the single most practical fact in the concept: **what you
hold across an `.await` is what your task costs.**

> Quick reference: [`async` / `.await`](../../../languages/rust.md#async-await) in the handbook.

## Exercises
Both need edition 2018 or later, which `cargo` gives you by default. Running a bare file with
`rustc` does **not** — it defaults to the 2015 edition, where `async` isn't a keyword at all. Use:

```bash
rustc --edition 2024 1-solution.rs && ./1-solution
```

1. **Nothing runs until you drive it** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Write `brew` twice — plain and `async` — with a `println!` in the body, and prove from the output
   that calling the async one grinds no beans. Then drive it with `block_on`. Also call it once
   without `block_on` and read the compiler's warning in full.
2. **Measure a paused function** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Print `size_of_val` for a chain of nested futures and for the two 512-byte versions above.
   Predict each number *before* you compile. The nesting one is guessable; the 16-vs-514 pair is the
   one that teaches you something.

## Next
- What the compiler actually builds — the state machine, why your locals become an enum's fields, and
  why a paused future is the only value in Rust that can't be allowed to move: [Under the hood](under-the-hood.md).
- Then [Concept 39](../39-future-poll-and-the-executor/use-it.md) opens `block_on`: what `poll`
  returns, what a `Waker` is for, and how polling two futures in one loop gets you two things
  happening at once on **one thread**.
