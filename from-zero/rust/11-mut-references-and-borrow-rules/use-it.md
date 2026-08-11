# Concept 11 · `&mut` and the borrow rules — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 10](../10-borrowing-with-ref/use-it.md)

## The idea
[Concept 10](../10-borrowing-with-ref/use-it.md) gave you a way to *read* a value without
owning it — a `&` borrow. But it was read-only: `s.push_str("!")` through a `&String`
refused to compile. Sometimes you want a function to **change** your value in place,
without taking it away and without cloning it.

That's a **mutable reference**, written `&mut`:

```rust
fn add_bang(s: &mut String) {
    s.push('!');            // allowed — this is a mutable borrow
}

fn main() {
    let mut text = String::from("hi");
    add_bang(&mut text);    // lend it out mutably
    println!("{text}");     // hi!  — the original changed
}
```

Three things have to line up:
- **`let mut text`** — you can only lend out write-access to a value that's itself
  changeable ([Concept 02](../02-frozen-by-default-and-mut/use-it.md)).
- **`s: &mut String`** in the signature — "I take a mutable reference."
- **`&mut text`** at the call — "borrow mine, and you may change it."

(Quick reference: [`&mut` in the handbook](../../../languages/rust.md#mut-ref).)

Because the function edits the *owner's* value (not a copy), the change is still there
after the call returns. Contrast [Concept 06](../06-copy-types/use-it.md), where changing a
`Copy` parameter only touched the function's private copy — that was a copy; this is a
reference to the very same value.

```rust
fn append_domain(email: &mut String) {
    email.push_str("@example.com");
}

fn main() {
    let mut user = String::from("sam");
    append_domain(&mut user);
    println!("{user}");   // sam@example.com
}
```

## The one rule that governs all borrowing
`&mut` is powerful — it can change things — so Rust guards it with a single rule. At any
given moment, a value may have **either**:

- **any number of shared `&` readers**, *or*
- **exactly one `&mut` writer**,

**but never both at once, and never two writers.** One writer *xor* many readers.

Break it and the compiler stops you — these don't build:

```rust
let mut s = String::from("hi");
let r1 = &mut s;
let r2 = &mut s;        // ❌ error[E0499]: cannot borrow `s` as mutable more than once
println!("{r1} {r2}");
```

```rust
let mut s = String::from("hi");
let reader = &s;
let writer = &mut s;   // ❌ error[E0502]: ...as mutable because it is also borrowed as immutable
println!("{reader} {writer}");
```

This feels strict at first, but it's the guarantee that a value can never be changed out
from under someone who's reading it, and never scribbled on by two writers at once. Why
that matters — and why it makes whole categories of bugs impossible — is
[Under the hood](under-the-hood.md).

## Exercises
1. **Change it in place** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Lend a String mutably to a function that appends `'!'`. (Expect `hi!`.)
2. **The owner sees the change** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Mutably borrow a String, append a suffix, then print the original. (Expect
   `sam@example.com`.)

## Next
- Why the one-writer rule exists, the exact bug it prevents, and how it ties back to the
  growing `String` from Concept 07: [Under the hood](under-the-hood.md).
