# Concept 35 · `Arc<Mutex<T>>` (shared, mutable state across threads) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 34](../34-threads/use-it.md)

## The idea
[Concept 34](../34-threads/use-it.md) left us with one value and one thread. `move` hands a value
*into* a thread, and because ownership is exclusive, that's the end of the story — the value belongs
to that thread and nobody else can touch it.

But plenty of real work needs the opposite: **four threads all adding to the same counter**, all
pushing into the same list, all updating the same total. One value, several threads.

You have already solved this exact problem — just not across threads. Back in
[Concept 32](../32-rc-refcell/use-it.md) the answer was `Rc<RefCell<T>>`:

- [`Rc<T>`](../30-rc/use-it.md) → **many owners** of one value.
- [`RefCell<T>`](../31-refcell/use-it.md) → **mutate** it through any of those shared handles.

Neither of those is allowed across threads (the next lesson shows the physical reason). Each has a
**thread-safe twin**, and you swap them one for one:

| Single thread | Across threads | Job |
|---|---|---|
| `Rc<T>` | **`Arc<T>`** | many owners of one value |
| `RefCell<T>` | **`Mutex<T>`** | mutate through a shared handle |
| `Rc<RefCell<T>>` | **`Arc<Mutex<T>>`** | **shared, mutable state** |

> **`Arc<Mutex<T>>` = many threads own one value, and take turns changing it.**

`Arc` is "**a**tomically **r**eference **c**ounted" — the same owner-count as `Rc`, but counted in a
way two threads can't corrupt. `Mutex` is short for "**mut**ual **ex**clusion" — a lock that lets
exactly **one** thread inside at a time, so their edits can't collide.

![Three thread stacks each holding an Arc handle to one heap box containing an atomic owner count, a lock flag, and the shared value](diagrams/arc-mutex.svg)

## The shape, line by line
Here's the whole pattern — four threads each bumping one shared counter ten times:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter_handle = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                let mut value = counter_handle.lock().unwrap();
                *value += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());   // always 40
}
```

Four beats, and each one is a tool you already know:

1. **`Arc::new(Mutex::new(0))`** — read it outside-in, exactly like `Rc<RefCell<T>>`: the outer `Arc`
   shares ownership, the inner `Mutex` guards the value so it can be changed.
2. **`Arc::clone(&counter)`** — make one extra owner *per thread*. It is a cheap count bump, not a
   copy of the value; every clone points at the **same** `Mutex`.
3. **`move ||`** — the clone (not the original) is moved into the thread, so the thread owns its own
   handle to the shared value. This is the [`move`](../34-threads/use-it.md) rule from last lesson,
   unchanged.
4. **`.lock().unwrap()`** — "wait until it's my turn, then let me in." What comes back behaves like a
   `&mut` to the value, so `*value += 1` edits the one shared number.

Always `Arc::clone` **before** the closure. `move` would otherwise consume the original `counter`
on the first loop pass, and there would be nothing left to clone on the second.

## `.lock()` — taking your turn
`.lock()` is the `Mutex` version of [`.borrow_mut()`](../31-refcell/use-it.md), and it differs in one
important way:

- `RefCell::borrow_mut()` **panics** if someone else is already borrowing — in one thread, a second
  borrow is always a bug.
- `Mutex::lock()` **waits**. If another thread holds the lock, this thread simply sleeps until that
  thread is done, then goes in. Waiting is normal here, not a bug — it's how the turns are taken.

`.lock()` returns a [`Result`](../23-result/use-it.md), which is why you see `.unwrap()`. The `Err`
case has one specific meaning: a thread **panicked while holding the lock**, so the value might be
half-updated. Rust calls that a *poisoned* lock and reports it rather than handing you data that may
be inconsistent.

Inside the `Ok` is a **`MutexGuard`** — think of it as the ticket that proves it's your turn. You use
it like a reference to the value (`*value += 1`), and this is the key rule:

> **The lock is released when the guard is dropped** — automatically, at the end of its scope.

There is no `.unlock()` to forget. It's the same
[scope-ends-so-it's-cleaned-up](../08-ownership-and-moves/use-it.md) rule that frees a `String`,
doing lock management for free.

## Hold the lock for as short a time as possible
Because the guard holds the lock until it drops, *where you put it* decides how long other threads
wait:

```rust
// 🚫 holds the lock across a slow operation — the other threads queue behind it
let mut value = counter_handle.lock().unwrap();
*value += 1;
expensive_unrelated_work();
// still locked until the end of the block

// ✅ take the lock, edit, let it go, then do the slow part
{
    let mut value = counter_handle.lock().unwrap();
    *value += 1;
}                       // guard dropped here — lock free again
expensive_unrelated_work();
```

Same idea if you only need to *read* the value: `let total = *counter.lock().unwrap();` takes the
lock, copies the number out, and drops the guard on the same line.

And one trap worth knowing by name: if a thread tries to `.lock()` the **same** mutex it is already
holding, it waits for itself forever. That freeze is called a **deadlock** — no panic, no error, the
program simply stops. Locking once, briefly, and letting the guard drop avoids it.

## When to reach for it
- **Several threads updating one thing** — a shared counter, a shared results list, a cache many
  workers write into.
- **Not on one thread.** If there's only one thread, use [`Rc<RefCell<T>>`](../32-rc-refcell/use-it.md):
  atomics and locks cost real time, and you'd be paying for coordination nobody needs.
- **Not when you can just hand values back.** If threads only need to *report* results, `.join()`'s
  return value — or a channel, the next concept — is simpler than sharing memory at all.
- **`Arc` alone (no `Mutex`) is fine for read-only data** shared by many threads, e.g.
  `Arc<Vec<String>>` that everyone reads and nobody writes.

> Quick reference: [`Arc<T>`](../../../languages/rust.md#arc) and
> [`Mutex<T>`](../../../languages/rust.md#mutex) in the handbook.

## Exercises
1. **The shared counter** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Build the counter above from scratch: `Arc::new(Mutex::new(0))`, four threads that each add 10,
   join them all, print `40`. Try deleting one `Arc::clone` and read the compiler error — it tells
   you exactly why the clone is needed.
2. **A shared list several threads write into** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Share an `Arc<Mutex<Vec<String>>>` across three threads; each one locks it, pushes
   `"worker N reporting"`, and drops the guard. Join them, then sort and print the finished list.
   (Sorting is what makes the output stable — thread order isn't.)

## Next
- What this looks like **in memory** — one heap box holding an atomic count, a lock flag, and the
  value; why `Rc`'s ordinary count and `RefCell`'s ordinary flag physically break when two threads
  race on them: [Under the hood](under-the-hood.md).
- Then the other half of Rust concurrency: instead of *sharing* one value behind a lock, **send**
  values from thread to thread down a **channel** — no lock at all, because ownership moves.
