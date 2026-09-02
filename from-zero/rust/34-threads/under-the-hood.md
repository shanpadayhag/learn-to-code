# Concept 34 · Threads (`thread::spawn` and `move`) — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## Two threads means two stacks
A [stack](../04-functions-and-the-call-stack/use-it.md) is the strip of memory where a single line
of execution parks its frames — one per function call, pushed on the way in, popped on the way out.
The rule that made it simple was that there's only ever *one* of them.

A thread breaks that assumption in the gentlest way: it gets **its own stack**. So a program with a
spawned thread has **two** stacks living at once, side by side in memory, each pushing and popping
its own frames as its own code runs.

![Two separate stacks for main and the spawned thread, a value's ownership moving between them, and one shared heap buffer](diagrams/two-stacks-move.svg)

The **heap** is *not* split — there's one heap for the whole program. That's the crux of everything
about threads: stacks are private per thread, but the heap is shared, so the only question that ever
matters is *who is allowed to touch a heap value, and when*.

## Why a borrow across stacks would dangle
Now the `move` rule makes physical sense. Picture the broken version:

```text
   main's stack                         spawned thread's stack
   ┌────────────────────┐               ┌────────────────────┐
   │ data: String ──────┼──► heap "work"│ closure: &data ─────┼─► ??? 
   └────────────────────┘               └────────────────────┘
        (may pop first!)                     (still running)
```

`main`'s frame holds `data`. If the closure only **borrows** it (`&data`), that reference points
*into main's stack frame*. But the two threads run independently — `main`'s function can return, pop
its frame, and free that slot **while the spawned thread is still going**. The borrow would then
point at reclaimed stack memory: a dangling reference. The compiler can't prove the thread finishes
first, so it rejects the borrow with *"closure may outlive the current function."*

`move` removes the cross-stack pointer entirely. Instead of borrowing, the closure **takes
ownership**, so the value now lives inside the *thread's own frame*, on the *thread's own stack*. No
pointer reaches back into `main`. Nothing can dangle — the exact
[ownership guarantee from Concept 08](../08-ownership-and-moves/under-the-hood.md), reused unchanged
across a thread boundary.

## What `move` actually transfers
Same as every move you've seen: `move` copies the **owner** — for a `String` that's the little
`(ptr, len, cap)` triple sitting on the stack — from `main`'s frame into the thread's frame. The
**heap buffer is never copied**; it stays exactly where it is, and the ownership triple that names
it simply now lives on the other stack.

So "move a `String` into a thread" is cheap: three machine words change hands, not the text. And
because ownership is exclusive, `main` genuinely can't use `data` afterward — there's only one owner,
and it's on the other stack now.

## Values come back the same way
`.join()` is the reverse trip. The closure's return value is handed from the spawned thread back to
the waiting thread as an owned value, wrapped in a [`Result`](../23-result/under-the-hood.md):

- `Ok(value)` — the thread finished normally; `value` is moved out to you.
- `Err(_)` — the thread **panicked**; the panic was caught at the boundary and turned into an error,
  so it can't silently corrupt the rest of the program.

That's why a thread's work reaches you *through* `join` and not by reading some shared variable: with
one owner per value, handing the result back is itself a move.

## Predict the memory
```rust
use std::thread;

fn main() {
    let name = String::from("ada");

    let handle = thread::spawn(move || {
        format!("hi {}", name)     // (2)
    });

    // println!("{}", name);        // (1)  — imagine this line were here

    let greeting = handle.join().unwrap();  // (3)
    println!("{}", greeting);
}
```

1. If the commented line were uncommented, would it compile?
2. Where does `name` live while the closure runs — main's stack or the thread's stack?
3. How does `greeting` get from the spawned thread back to `main`?

<details>
<summary>Show the answer</summary>
<ol>
<li><strong>No.</strong> <code>move</code> transferred ownership of <code>name</code> into the thread, so <code>main</code> no longer owns it; using it here is a use-after-move — a compile error, caught before the program ever runs.</li>
<li><strong>On the spawned thread's stack.</strong> <code>move</code> put the <code>String</code>'s owner (<code>ptr/len/cap</code>) into the closure's frame on the new thread; the heap buffer <code>"ada"</code> stays put and is now named from there.</li>
<li><strong>Through <code>join</code>.</strong> The closure <em>returns</em> the <code>String</code>; <code>.join()</code> hands that owned value back to <code>main</code> inside <code>Ok(...)</code>, <code>.unwrap()</code> opens it, and <code>greeting</code> becomes its new owner on main's stack. One owner throughout — the value simply moved home.</li>
</ol>
</details>

## Next
- You can now run code on a second stack and hand owned values in (`move`) and out (`join`). The one
  thing `move` *can't* do is let **two** threads use the **same** value — ownership goes to exactly
  one of them.
- That's the next concept: `Arc<Mutex<T>>`, the thread-safe pair that lets many threads **share**
  one value and take turns **mutating** it — the concurrent echo of the `Rc<RefCell<T>>` you already
  know.
