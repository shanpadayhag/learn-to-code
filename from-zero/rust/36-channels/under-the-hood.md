# Concept 36 · Channels (`mpsc::channel`) — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## A channel is a queue on the heap with a handle at each end
`mpsc::channel()` allocates **one queue on the heap** and hands you two small handles for it:

```text
   worker's stack                 heap                       main's stack
   ┌──────────────┐        ┌──────────────────┐        ┌──────────────────┐
   │ sender ──────┼───────►│ [ "one" ]        │◄───────┼── receiver       │
   └──────────────┘        │ [ "two" ]        │        │ message: String  │
                           │ [  empty  ]      │        └──────────────────┘
                           │ senders alive: 1 │
                           └──────────────────┘
```

The `Sender` and `Receiver` are ordinary values living on ordinary stacks — each is roughly a pointer
to that shared queue, plus bookkeeping. The queue is **first in, first out**: values come out in the
order they went in.

![Two producer thread stacks holding Sender handles feeding one heap queue, with the main thread's Receiver taking values out in order](diagrams/channel.svg)

Yes — the queue itself *is* shared memory, and it *is* internally synchronized. The difference is
that the synchronization is inside the channel, done once, correctly, and you never write a lock. All
you do is give values away and take values out.

## Sending is a move, and that's the whole safety story
`sender.send(value)` is not a copy and not a borrow. It **moves** the value out of the sending
thread's frame and into the queue, exactly like passing a value to a function that keeps it:

- For an `i32` (a [`Copy`](../06-copy-types/under-the-hood.md) type) the four bytes are written into
  the queue slot.
- For a `String`, the **owner triple** (`ptr / len / cap`) is written into the slot. The text buffer
  on the heap is **not moved, not copied** — it stays exactly where it is, and the thing naming it now
  sits in the queue.

Then `receiver.recv()` does the same thing again in the other direction: it takes the value **out** of
the slot and moves it onto the receiving thread's stack.

Follow one `String` all the way through and count its owners at each instant:

| moment | who owns it | where the owner triple lives |
|---|---|---|
| before `send` | the worker | worker's stack frame |
| after `send`, before `recv` | the channel | a queue slot on the heap |
| after `recv` | main | main's stack frame |

**Exactly one owner, always.** That's why no lock is needed in *your* code: two threads never hold
the same value at the same time, so there is nothing to guard. The compiler enforces it the ordinary
way — try to use the `String` after `send` and it's a use-after-move error, caught before the program
runs.

Compare that with [`Arc<Mutex<T>>`](../35-arc-mutex/under-the-hood.md), where the value stays in one
heap box and *many* threads hold handles to it. There, ownership is genuinely shared, so a lock is the
only way to keep two writers apart. Channels dodge the whole problem by never sharing in the first
place.

## How the pipe knows it's closed
The channel keeps a count of **how many senders still exist** — cloning a `Sender` adds one, dropping
a `Sender` subtracts one, the same shape as an [`Rc`](../30-rc/under-the-hood.md) count.

That count is what makes `for received in receiver` terminate:

- **A value is waiting** → the loop hands it over immediately.
- **Nothing waiting, senders alive > 0** → the receiving thread **sleeps**. The OS parks it; it burns
  no CPU, and a later `send` wakes it.
- **Nothing waiting, senders alive == 0** → nobody can ever send again, so `recv` returns `Err` and
  the loop ends.

Now the classic hang explains itself. In `main` you make a channel, clone the sender for each worker,
and loop over the receiver — but `main`'s **original** sender is still sitting in its frame, alive
until the end of `main`. Senders alive never reaches 0, so after the last message the receiver goes to
sleep and never wakes. Nothing crashes; the program simply stops.

```rust
let (sender, receiver) = mpsc::channel();
for id in 1..=3 {
    let worker_sender = sender.clone();          // count: 2, 3, 4
    thread::spawn(move || { worker_sender.send(id).unwrap(); });
}                                                 // workers end → count falls to 1
drop(sender);                                     // count: 0 → the pipe is closed
for value in receiver { println!("{}", value); }  // ends after the third value
```

`drop(sender)` isn't magic — it's the ordinary end-of-life for a value, called early on purpose. The
same fix is often written by putting the original sender inside a scope so it falls out of it before
the loop.

`.send()` mirrors this from the other side: if the `Receiver` has been dropped, the value has nowhere
to go, so `send` hands it **back** to you inside the `Err` instead of silently dropping it. Even a
failed send doesn't lose ownership of anything.

## Predict the memory
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let report = String::from("done");     // (1)
        sender.send(report).unwrap();          // (2)
        // println!("{}", report);              // (3) — imagine this line were here
    });

    for message in receiver {                   // (4)
        println!("{}", message);
    }
}
```

1. Where does the text `"done"` live, and what is on the worker's stack?
2. What physically travels into the channel when `send` runs?
3. Would the commented line compile?
4. What makes this loop stop, given there is no `.join()`?

<details>
<summary>Show the answer</summary>

1. The bytes `"done"` are in a **heap** buffer. The worker's stack frame holds only the `String`'s
   owner triple — `ptr` to that buffer, `len 4`, `cap 4`.
2. **The owner triple, into a queue slot on the heap.** The text buffer doesn't move an inch; it's
   simply named from the queue now instead of from the worker's frame. Sending a huge `String` costs
   the same as sending a tiny one.
3. **No.** `send` *moved* the `String`, so the worker no longer owns it — using `report` afterwards is
   a use-after-move, rejected at compile time. Ownership travelled down the pipe.
4. **The sender dropped.** `move` gave the worker the only `Sender`; when the closure ends, that
   sender dies and the count of live senders hits 0. The channel closes, `recv` returns `Err`, and the
   `for` loop finishes — which also proves the worker is done, so no `.join()` is needed.
</details>

## Next
- You now have both halves of Rust concurrency: **share a value** behind `Arc<Mutex<T>>`, or **send
  values** down a channel — and both are ownership rules you already knew, applied across stacks.
- Both rested on the same permission slip, checked at every boundary and never yet named:
  [`Send` and `Sync`](../37-send-and-sync/use-it.md) — may this value *move* to another thread, and
  may it be *shared* with one? That's what closes Phase 9.
