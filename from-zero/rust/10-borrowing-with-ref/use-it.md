# Concept 10 · Borrowing with `&` — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 09](../09-clone-the-inefficient-fix/use-it.md)

## The idea
Here's the knot from the last two lessons. You want a function to *read* your `String`:
- **Move** it in ([Concept 08](../08-ownership-and-moves/use-it.md)) and you lose it — the
  function walks off with your value.
- **Clone** it in ([Concept 09](../09-clone-the-inefficient-fix/use-it.md)) and you copy
  the whole heap buffer just so the function can glance at it — wasteful.

But you didn't want to *give away* the value or *duplicate* it. You wanted to **lend** it.
That's **borrowing**, and the tool is `&`:

```rust
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let text = String::from("hello");
    let n = length(&text);        // lend text out — don't give it away
    println!("{text} is {n}");    // ✅ text is still ours: hello is 5
}
```

`&text` hands the function a **reference** — a small pointer that lets it *look at* `text`
without owning it. No move, no copy. When `length` finishes, the reference just vanishes
and `text` is still yours.

Two spots wear the `&`:
- `s: &String` in the signature — "I take a *reference to* a String, not a String."
- `length(&text)` at the call — "here, *borrow* mine."

(Quick reference: [`&T` shared references in the handbook](../../../languages/rust.md#borrow).)

## You can lend it out as often as you like
A borrow doesn't consume anything, so there's no "used it up" problem like a move had:

```rust
fn size(s: &String) -> usize { s.len() }

fn main() {
    let name = String::from("Sam");
    let a = size(&name);      // borrow once
    let b = size(&name);      // borrow again — totally fine
    println!("{name} {a} {b}"); // Sam 3 3
}
```

With a move, the second call would fail (`name` already gone). With clone, you'd have paid
for two full copies. Borrowing: neither problem.

## A plain `&` is read-only
There's one deliberate limit. A `&` borrow lets you **look but not touch** — you can't
change the value through it:

```rust
fn grow(s: &String) {
    s.push_str("!");   // ❌ error[E0596]: cannot borrow `*s` as mutable
}
```

That's not a bug, it's the design: a shared `&` reference promises read-only access.
Changing a value you've only *borrowed* needs a different, stronger kind of borrow —
`&mut` — which is the whole of [Concept 11](../README.md).

## Exercises
1. **Read without owning** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Borrow a String into a function that returns its length; keep using the original.
   (Expect `hello 5`.)
2. **Lend it twice** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Borrow the same String into two calls, then print it and both results.
   (Expect `Sam 3 3`.)

## Next
- What a reference actually *is* in memory, why it's nearly free, and how Rust still keeps
  it safe: [Under the hood](under-the-hood.md).
