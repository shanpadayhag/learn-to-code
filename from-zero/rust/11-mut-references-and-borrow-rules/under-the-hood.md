# Concept 11 · `&mut` and the borrow rules — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## What `&mut` does in memory
A `&mut` is the same shape as the shared `&` from
[Concept 10](../10-borrowing-with-ref/under-the-hood.md): a small address pointing at the
owner's value. The difference is permission — through a `&mut`, the borrower is allowed to
*write*. It reaches down the pointer to the owner's real handle and heap buffer and edits
them in place.

![a &mut reference writing ! into the owner's heap buffer, changing hi to hi!](diagrams/mut-borrow.svg)

That's why the change survives the call: there was only ever **one** buffer — the owner's
— and the function wrote directly into it. Nothing was copied, nothing was moved.

## The rule, and the bug it prevents
Here's the rule again, stated as memory permissions:

> A value may have **many `&` readers** at once, **or one `&mut` writer**, but never both,
> and never two writers.

![three panels: many readers ok, one writer ok, writer-plus-anything forbidden](diagrams/borrow-rules.svg)

Why forbid a writer while readers exist? Reach back to
[Concept 07](../07-the-heap-and-string/under-the-hood.md): pushing onto a `String` can run
out of capacity, so Rust reserves a **bigger heap buffer, copies the bytes over, and
updates `ptr`** to the new location. Now imagine someone was holding a plain `&` into the
*old* buffer while that happened. Their reference would still point at the old, freed
location — a **dangling pointer** reading garbage.

The one-writer rule makes that impossible *by construction*: while a `&mut` is out and
possibly reallocating, the compiler guarantees **no other reference to that value exists**
to be left dangling. And forbidding two `&mut` writers stops two pieces of code from
scribbling over each other's changes. This is the same guarantee that makes data races
impossible when you get to threads much later — one rule, enforced at compile time, no
runtime cost.

## `mut` in two places — don't mix them up
You've now seen `mut` in two different roles, and it's worth separating them cleanly:

- **`let mut x`** — the *binding* is changeable; you can reassign `x` or call methods that
  mutate it ([Concept 02](../02-frozen-by-default-and-mut/use-it.md)).
- **`&mut x`** — a *reference through which you may mutate* the value it points at.

They cooperate: to hand out `&mut x`, the value `x` must itself be `let mut` — you can't
lend write-access to something frozen. But they're distinct ideas: one is about a
variable's own changeability, the other about the permission a borrow carries.

## Phase 2, complete
Step back and look at what these six concepts built, each one answering the last:

| you learned | it raised the question | answered by |
|---|---|---|
| `Copy` types (06) | what about values too big to copy cheaply? | the heap & `String` (07) |
| heap & `String` (07) | who frees the heap buffer, and when? | ownership & moves (08) |
| moves (08) | how do I keep a value I gave away? | `.clone()` (09) |
| `.clone()` (09) | isn't a full copy wasteful just to read? | borrowing `&` (10) |
| borrowing `&` (10) | how do I *change* a value I only borrowed? | `&mut` (11) |

That chain — from "a number in a box" all the way to safe, zero-cost mutable borrowing —
*is* Rust's memory model. Everything after this (structs, collections, traits, lifetimes,
concurrency) is built on top of it.

## Predict the memory
```rust
fn twice(s: &mut String) {
    let copy = s.clone();
    s.push_str(&copy);
}

fn main() {
    let mut word = String::from("ab");
    twice(&mut word);
    println!("{word}");
}
```

1. Does `twice` own `word`, or borrow it mutably?
2. Is `word` still usable in `main` after the call?
3. What does it print?

<details>
<summary>Show the answer</summary>

1. It **borrows it mutably** — the parameter is `&mut String`. Ownership stays in `main`.
2. **Yes.** A borrow (even a mutable one) never takes ownership, so `word` is `main`'s the
   whole time.
3. `abab` — `twice` clones `"ab"`, then appends that copy onto the owner's buffer in place,
   giving `"ab" + "ab"`.
</details>

## Next
- [Concept 12 — Slices](../README.md): a reference to just *part* of a value — the last
  borrowing idea before Phase 2 wraps and we start building compound types.
