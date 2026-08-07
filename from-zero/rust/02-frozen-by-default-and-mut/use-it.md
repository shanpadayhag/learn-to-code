# Concept 02 · Frozen by default, and `mut` — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 01](../01-a-number-in-a-variable/use-it.md)

## The idea
In [Concept 01](../01-a-number-in-a-variable/use-it.md) you put `5` in a box called
`x`. Natural next question: can you *change* what's in the box?

By default, **no**. In Rust a variable is **frozen** the moment you create it. This
won't compile:

```rust
fn main() {
    let x = 5;
    x = 6;          // ❌ error: cannot assign twice to immutable variable `x`
    println!("{x}");
}
```

The compiler stops you on purpose. To say "I actually want to change this one," add
the keyword `mut` (short for *mutable* = changeable):

```rust
fn main() {
    let mut x = 5;  // this box is allowed to change
    x = 6;          // ✅ fine now
    println!("{x}");
}
```

Output:
```
6
```

## Why default to frozen?
Most variables never need to change, and a variable that changes when you didn't mean
it to is a classic bug. Rust flips the usual default: things are locked unless you
*opt in* to mutation with `mut`. The `mut` keyword then doubles as a signal to anyone
reading the code — "watch this one, it moves."

## Not the same as re-declaring
This also compiles, and it is **not** the same thing:

```rust
let x = 5;
let x = 6;   // a brand-new box, also called x
```

Writing `let` again makes a *new* variable that happens to reuse the name (this is
called *shadowing*, and it gets its own lesson later). `mut` is different: it lets you
change the value *inside the existing box*. The [Under the hood](under-the-hood.md)
lesson shows why that distinction is physical, not cosmetic.

> Quick reference: [Rust handbook — `let` / `let mut`](../../../languages/rust.md#let-mut).

## Exercises
1. **A counter** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Start a variable at `0`, add `1` to it twice, then print it. (It should print `2`.)
   You'll need `mut`.
2. **Make it compile** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   The starter fails to compile because it changes a frozen variable. Fix it by
   changing **one word**.

## Next
- The memory side: [Under the hood](under-the-hood.md) — what `x = 6` does to the box,
  and why it costs nothing at runtime.
