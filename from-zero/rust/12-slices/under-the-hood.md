# Concept 12 · Slices — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## What a slice stores
A plain `&` reference ([Concept 10](../10-borrowing-with-ref/under-the-hood.md)) was a
single address. A slice needs one more thing, because it refers to a *range*, not a whole
value. So a slice is **two words**: a **pointer** to where the range starts, and a
**length** — how many elements it covers.

```
world = { ptr → byte 6 of s's buffer,  len 5 }
```

![the slice `world` pointing at byte 6 of s's buffer, spanning 5 bytes](diagrams/slice-into-buffer.svg)

That's the whole slice. It holds **no text of its own** — just where to start and how far
to go, pointing straight into the owner's existing buffer. Taking a slice copies nothing;
it's as cheap as a reference, plus a length. (This two-word "pointer + length" shape is
sometimes called a *fat pointer*.)

Because the length lives *in the slice*, Rust always knows exactly where your window ends
without scanning for a terminator — and it can bounds-check that your range actually fits
inside the buffer.

## Why growing the borrowed String is forbidden
You saw `s.push_str("!!!")` fail while a slice of `s` was alive. This is the
[Concept 11](../11-mut-references-and-borrow-rules/under-the-hood.md) rule doing its job,
and the danger is concrete.

A slice is a shared `&` borrow, so while it's out, no `&mut` to `s` is allowed — and
`push_str` needs a `&mut`. But *why* is that dangerous? Recall
[Concept 07](../07-the-heap-and-string/under-the-hood.md): growing a `String` can
reallocate — move the text to a **bigger heap buffer** and free the old one. Your slice's
pointer still aims at the **old** location. The moment the buffer moved, the slice would be
pointing at freed memory — a dangling window onto garbage. The borrow checker forbids the
mutation *before* it can happen, so a slice always points at live, valid text.

## `&str` and `String`, finally side by side
You now have both halves of Rust's text story:

| | `String` | `&str` (string slice) |
|---|---|---|
| owns its text? | **yes** — owns a heap buffer | **no** — borrows someone else's |
| shape | ptr + len + capacity | ptr + len |
| can grow? | yes (`push_str`, …) | no — it's a read-only view |
| where the text lives | the heap | a `String`'s heap buffer, **or** baked into the program (a literal) |

A literal like `"hello"` is a `&str` pointing at read-only bytes compiled into your
executable — that's why it needs no allocation and why it isn't a `String`. A slice
`&s[0..5]` is a `&str` pointing into a `String`'s heap buffer. Same type, same two-word
shape; they just point at different places. (Quick reference:
[`String`](../../../languages/rust.md#string) · [slices](../../../languages/rust.md#slice).)

This is also why "take `&str` to read, `String` to own" is the everyday rule: a `&str`
parameter accepts a literal *and* a slice of a `String`, so it's the most flexible way to
say "I just need to look at some text."

## Predict the memory
```rust
fn main() {
    let s = String::from("boathouse");
    let boat = &s[0..4];
    let house = &s[4..];
    println!("{boat} {house} {}", s.len());
}
```

1. How many heap buffers hold text here — one, two, or three?
2. What do `boat` and `house` each *contain* (not the letters — the fields)?
3. What does the line print?

<details>
<summary>Show the answer</summary>

1. **One.** `s` owns the single buffer `"boathouse"`; `boat` and `house` are just windows
   into it — no new buffers.
2. Each is a **pointer + a length**: `boat = {ptr → byte 0, len 4}`,
   `house = {ptr → byte 4, len 5}`. No copied text.
3. `boat house 9` — `"boathouse"` is 9 bytes long.
</details>

## Phase 2 complete — where the memory model lands
Twelve concepts in, you can now answer *where does this value live, and who owns it?* for
anything: a number on the stack, text on the heap, a value moved to a new owner, a copy, a
borrow, a mutable borrow, and now a borrowed window into part of a value. That question —
the through-line of every lesson so far — is the whole of Rust's memory model, and it's
built.

## Next
- **Phase 3 — compound data.** We stop asking only "where does one value live" and start
  building our own types out of many: `struct`s (Concept 13), then `enum`s, `Option`, and
  pattern matching. The ownership and borrowing rules you just learned carry straight over
  — now they apply to types *you* design.
