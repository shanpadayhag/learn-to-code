# Concept 32 · `Rc<RefCell<T>>` (shared, mutable state) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 31](../31-refcell/use-it.md)

## The idea
You now hold both halves of a puzzle, each with a gap the other fills:

- [`Rc<T>`](../30-rc/use-it.md) — **many owners** of one value, but **read-only**.
- [`RefCell<T>`](../31-refcell/use-it.md) — **mutate** a value through a shared `&`, but by itself
  it has **one owner** like any normal value.

Put them together and the gaps cancel out. Nest the `RefCell` *inside* the `Rc`:

> **`Rc<RefCell<T>>` = many owners who can all *change* one shared value.** The `Rc` shares the
> ownership; the `RefCell` inside supplies the mutation the `Rc` alone couldn't.

This stack is Rust's standard shape for **shared mutable state** — a value several parts of a
program both own and update: a node several places in a graph edit, a counter many handlers bump, a
config multiple modules tweak. It looks intimidating, but it's just the two tools you already know,
one wrapped in the other.

![An Rc with owner-count shares one RefCell, whose borrow flag guards the mutable value inside](diagrams/rc-refcell-layers.svg)

## Read the type from the outside in
`Rc<RefCell<i32>>` is two layers, and each layer is a job you already do:

- **Outer `Rc<…>`** → "shared ownership." To make another owner, [`Rc::clone`](../30-rc/use-it.md)
  it — a cheap count bump, not a copy.
- **Inner `RefCell<i32>`** → "mutate through a shared `&`." To change the value, call
  [`.borrow_mut()`](../31-refcell/use-it.md) on it.

So the whole workflow is: **`Rc::clone` to hand out owners; `.borrow_mut()` to edit through any of
them.** Because every clone points at the *same* `RefCell`, an edit through one owner is seen by
all of them.

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(0));   // one RefCell, first owner

let owner_a = Rc::clone(&shared);        // a second owner of the SAME RefCell
let owner_b = Rc::clone(&shared);        // a third

*owner_a.borrow_mut() += 5;              // edit through owner_a
*owner_b.borrow_mut() += 10;             // edit through owner_b

println!("{}", shared.borrow());          // 15 — all three see one shared value
```

Three owners, one value, and changes made through any owner land in the single shared `RefCell`.
Note the auto-dereference: `owner_a.borrow_mut()` reaches *through* the `Rc` to call `borrow_mut`
on the `RefCell` inside — you don't write any `*` to get past the `Rc`.

## Why `Rc` goes on the outside
The order matters. `Rc<RefCell<T>>` (Rc outside) is the useful one: cloning the `Rc` gives more
owners **of the same mutable cell**, so everyone shares one value they can all change. The reverse,
`RefCell<Rc<T>>`, means something different and rarer — a *swappable pointer* to a read-only shared
value, where `.borrow_mut()` lets you repoint it at a different `Rc`, not change the value inside.
When you want shared state you can mutate — nearly always — reach for **`Rc<RefCell<T>>`**.

## The costs carry over, both of them
Stacking the tools stacks their catches too — nothing new, just both at once:

- **The `RefCell` panic risk (Concept 31).** Borrow-rule violations are still checked at runtime.
  Two live `.borrow_mut()` on the same cell — even through *different* `Rc` owners — panic with
  `already borrowed`. Keep each borrow short.
- **The `Rc` single-thread limit (Concept 30).** Single-threaded only. The cross-thread version
  swaps both pieces for their thread-safe siblings: `Arc<Mutex<T>>`.

There's also one trap worth naming now: because an `Rc<RefCell<T>>` can hold an `Rc` pointing
*back* at something that points at it, you can build a **reference cycle** — two nodes that keep
each other's owner count above zero forever, so neither is ever freed (a memory leak). The tool
that breaks a cycle, `Weak<T>`, is the next lesson; for now, just know that mutable shared graphs
are where cycles can sneak in.

## When to reach for it
- **Shared mutable state** — a value that genuinely has several owners *and* must change:
  observers, graph/tree nodes with shared children, a counter or cache many parts update.
- **Not as a default.** If a plain `&mut`, or a single owner, or `Rc` alone (read-only sharing)
  already does the job, use that — each is simpler, cheaper, and can't panic. `Rc<RefCell<T>>` is
  the answer specifically when you need *both* many owners *and* mutation.

> Quick reference: the [`RefCell<T>` handbook entry](../../../languages/rust.md#refcell) covers the
> pairing; [`Rc<T>`](../../../languages/rust.md#rc) is the sharing half.

## Exercises
1. **A counter with two owners** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Make an `Rc<RefCell<i32>>` at `0`. `Rc::clone` it into a second owner. Bump the value through the
   *first* owner (`+1`) and through the *second* owner (`+1`), then print the value via the original
   (`2`) — one shared count changed through two owners.
2. **A shared list two writers append to** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Make an `Rc<RefCell<Vec<i32>>>`. Write `fn push(list: &Rc<RefCell<Vec<i32>>>, n: i32)` that
   `.borrow_mut().push(n)`. Clone the `Rc` so two owners exist, push through each, and print the
   final vector (`[1, 2]`).

## Next
- What `Rc<RefCell<T>>` looks like **in memory** — the owner count and the borrow flag as two
  separate counters doing two different jobs in one heap allocation — and exactly how a reference
  **cycle** leaks: [Under the hood](under-the-hood.md).
- Then the smart-pointer story finishes with **`Weak<T>`**: a non-owning `Rc` handle that *doesn't*
  bump the owner count, so it can point back into a structure without creating the leak above.
