# Concept 02 · Frozen by default, and `mut` — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

You saw that `mut` lets you reassign a variable. The question that actually matters:
when `x = 6` runs, **what happens to the box in memory?**

## Mutation rewrites the same box
Recall Concept 01: `let mut x = 5;` reserves one 4-byte box on the stack, at some
address, and writes `5` into it. When `x = 6;` runs, it does **not** make a new box.
It writes the bits for `6` into **the exact same 4 bytes**, at the exact same address.
The old `5` is simply overwritten.

![The box for x keeps its address while its contents change from 5 to 6](diagrams/mutate-in-place.svg)

So mutation is *in place*: same slot, same address, new contents. That's the whole
mechanical difference between a `mut` variable and a frozen one — a frozen variable is
written **once** and never again.

## "Frozen" isn't a padlock the program carries around
You might picture "frozen" as Rust snapping a little padlock on the box and checking it
every time the program runs. It's nothing like that. Being frozen costs the running
program **nothing** — no padlock, no checking, no slowdown. `let x = 5;` (without `mut`)
simply tells the **compiler** — the helper that reads your code *before* it ever runs —
to refuse any line that tries to change `x`. It catches the mistake while you're still
typing, then steps out of the way. A frozen box and a changeable (`mut`) box look
exactly the same in memory; the only difference is which changes the compiler will let
you write.

## Why this isn't the same as re-declaring
The [Use it](use-it.md) lesson noted that `let x = 6;` (a second `let`) is different
from `x = 6;` (mutation). Now it's physical:

- `x = 6;` — **same box**, contents overwritten. One address, start to finish.
- `let x = 6;` — a **new box** (its own address); the old one still sits there until
  the scope ends, just unreachable by that name.

That's why shadowing is worth its own treatment, in
[Interlude 12b](../12b-trim-returns-str/use-it.md): it's a different thing happening in
memory, not just a different way to write the same thing.

## Predict the memory
```rust
fn main() {
    let mut x = 5;
    x = 6;
}
```

Two questions before you expand the answer:
1. After `x = 6`, is `x` at the **same address** as before, or a new one?
2. Did making `x` mutable (instead of frozen) change how much memory it uses?

<details>
<summary>Show the answer</summary>

1. **Same address.** `x = 6` overwrites the same 4-byte slot in place — mutation never
   relocates the box.
2. **No.** A `mut i32` and a frozen `i32` are identical in memory (both 4 bytes on the
   stack). `mut` only changes what the *compiler* lets you do; it adds nothing at
   runtime.
</details>

## Next
- [Concept 03 — Types have sizes](../README.md): we've been saying "4 bytes" — next is
  *why* the compiler always knows that number, and what breaks if it can't.
