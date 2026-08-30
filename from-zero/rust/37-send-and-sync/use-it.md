# Concept 37 · `Send` and `Sync` (what may cross a thread) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 36](../36-channels/use-it.md)

## The idea
You have already been stopped by this rule twice, without being told its name.

In [Concept 34](../34-threads/use-it.md) a thread couldn't borrow a local, so you wrote `move`. In
[Concept 35](../35-arc-mutex/use-it.md) an [`Rc`](../30-rc/use-it.md) was *refused* at the thread
boundary and you swapped it for an `Arc`. Neither was a special case about `Rc`. Both were the same
check, and it runs on **every value that crosses from one thread to another**.

The check is two traits — `Send` and `Sync` — and they answer two different questions:

- **`Send`** — may this value **move** to another thread? The bytes leave this stack and arrive on
  that one; this thread can never touch it again.
- **`Sync`** — may this value be **shared** with another thread? The value stays exactly where it is,
  and a `&` reference to it crosses instead, so **two threads can reach the same memory at once**.

That's the whole distinction, and it's a memory picture before it's a rule. Moving means one thread
has it. Sharing means two threads point at it.

![A thread boundary with two crossings: a value moving across (Send) and a reference crossing while the value stays put (Sync), plus a struct stamped not-Send because one field is an Rc](diagrams/send-sync.svg)

## `Send` — the value may travel
Almost everything you have written so far is `Send`: `i32`, `bool`, `String`, `Vec<T>`,
[structs](../13-structs/use-it.md) and [enums](../14-enums/use-it.md) you define, `Box<T>`, `Arc<T>`.
When you hand a `String` to a thread with `move`, or [`send`](../36-channels/use-it.md) it down a
channel, this is the trait that permitted it.

`Rc<T>` is the famous exception. Its owner count is a plain `+= 1`, which two threads can interleave
and corrupt — the walkthrough is in [Concept 35](../35-arc-mutex/under-the-hood.md#why-rcs-count-breaks-across-threads).
So `Rc` is **not** `Send`, and the compiler stops it at the boundary:

```
error[E0277]: `Rc<Vec<String>>` cannot be sent between threads safely
   = help: within `{closure@...}`, the trait `Send` is not implemented for `Rc<Vec<String>>`
```

Read that first line carefully — **"cannot be sent"**. You'll meet its twin in a moment.

## `Sync` — a `&` to the value may travel
Here is the exact definition, and it's worth memorising because it turns one question into the other:

> **`T` is `Sync` if and only if `&T` is `Send`.**

"Shareable" just means "a reference to it is sendable". Nothing new — `Sync` is `Send`, asked about
the reference instead of the value.

The type that separates the two is one you already know: [`RefCell<T>`](../31-refcell/use-it.md).

- **`RefCell` is `Send`.** Move the whole cell to another thread and only that thread has it. Its
  borrow flag is never touched by two threads, so nothing can go wrong.
- **`RefCell` is not `Sync`.** Share `&RefCell` with a second thread and *both* can call
  `.borrow_mut()`. The flag is a plain counter, so both may read "nobody's borrowing" and both hand
  out a `&mut` to the same value — two mutable references at once, the one thing the
  [borrow rules](../11-mut-references-and-borrow-rules/use-it.md) exist to forbid.

Try to share one and the error changes by exactly one word:

```
error[E0277]: `RefCell<i32>` cannot be shared between threads safely
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: required for `Arc<RefCell<i32>>` to implement `Send`
```

**"cannot be shared"** — that's `Sync`. And read that last note: `Arc<RefCell<i32>>` is not `Send`
*because* `RefCell` is not `Sync`. An `Arc` hands every thread a shared reference to one value, so an
`Arc` can only travel if what's inside it can be shared. The definition, spelled out by the compiler.

`Mutex<T>` is the `RefCell` whose flag is a real lock — so it **is** `Sync`, and `Arc<Mutex<T>>`
travels. That pairing you wrote in Concept 35 was these two traits lining up all along.

## You never write these traits — you only lose them
`Send` and `Sync` are **auto traits**. You don't `impl` them and you don't `derive` them. The compiler
stamps them onto your type automatically, by one rule:

> A struct or enum is `Send` if **every field** is `Send`, and `Sync` if **every field** is `Sync`.

So you never gain these traits by writing code, and you lose them the moment one field isn't. That's
why a single `Rc` field poisons the whole struct, and the compiler says so in as many words:

```rust
struct Report {
    title: String,           // Send + Sync
    lines: Rc<Vec<String>>,  // ❌ neither
}                            // → Report is neither
```

```
note: required because it appears within the type `Report`
```

The fix is never to fight the trait — it's to change the field. `Rc` → `Arc`, `RefCell` → `Mutex`.

## The list worth knowing by heart
| type | `Send` | `Sync` | why |
|---|---|---|---|
| `i32`, `bool`, `String`, `Vec<T>` | ✅ | ✅ | plain data, nothing shared behind your back |
| `&T` | ✅ *(if `T: Sync`)* | ✅ | sending a reference **is** sharing the value |
| `Rc<T>` | ❌ | ❌ | non-atomic owner count — two threads can lose an increment |
| `RefCell<T>` | ✅ *(if `T: Send`)* | ❌ | non-atomic borrow flag — fine alone, unsafe to share |
| `Arc<T>` | ✅ *(if `T: Send + Sync`)* | ✅ *(same)* | atomic count, and it only shares what's shareable |
| `Mutex<T>` | ✅ *(if `T: Send`)* | ✅ *(same)* | the lock makes sharing safe |
| `MutexGuard<'_, T>` | ❌ | ✅ *(if `T: Sync`)* | must be unlocked by the thread that locked it |
| `*const T`, `*mut T` | ❌ | ❌ | a raw pointer carries no rules at all — these arrive with `unsafe` |

Two rows repay a second look. **`&T` is `Send` only when `T: Sync`** — that's the definition read
backwards. And **`MutexGuard` is not `Send`**: you may not lock on one thread and unlock on another,
which is exactly why holding a guard across a thread hand-off is rejected rather than merely
discouraged.

## Proving it without running anything
Because these are ordinary trait bounds, you can ask the compiler about any type in two lines:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<Arc<i32>>();      // fine
    assert_sync::<Mutex<i32>>();    // fine
    // assert_send::<Rc<i32>>();    // ❌ "cannot be sent between threads safely"
    // assert_sync::<RefCell<i32>>();// ❌ "cannot be shared between threads safely"
}
```

The functions have empty bodies and are never really called for their behaviour — the **bound** is
the whole test, and it's checked at compile time. Real crates keep exactly these lines in their test
suites so an accidentally-added `Rc` breaks the build instead of a user's program.

> Quick reference: [`Send` and `Sync`](../../../languages/rust.md#send-sync) in the handbook.

## Exercises
1. **One field takes the trait away** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Build a `Report` struct holding an `Rc<Vec<String>>`, hand it to `thread::spawn`, and read the real
   error — including the line naming *which field* is at fault. Then fix it with `Arc`, and print
   `size_of` for both to see that the layout never changed.
2. **Interrogate the compiler** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Write `assert_send` / `assert_sync` and run them over `i32`, `String`, `Rc`, `RefCell`, `Arc`,
   `Mutex` and `MutexGuard`. Predict each answer *before* you compile — the interesting ones are
   `RefCell` (one ✅, one ❌) and `MutexGuard` (the ❌ is the other way round).

## Next
- Why the compiler can decide this without running your program, what an auto trait costs at runtime
  (nothing), and why `MutexGuard` of all things is the odd one out: [Under the hood](under-the-hood.md).
- That closes Phase 9. Every threading rule you've met — `move`, `Arc`, the lock, the channel — was
  one of these two traits being checked. Next, Phase 10 asks a different question: what if a thread
  shouldn't have to **block** while it waits? That's `async`.
