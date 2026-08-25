# Interlude 28a · How an iterator actually works — `.next()` and a cursor

> Interlude: a **single lesson**. It leans on one memory picture (an iterator is a
> *cursor* — a reference plus a position — and `.next()` reads-then-advances), so that
> picture lives right here rather than in a separate "Under the hood."
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 28](../28-iter-into-iter-iter-mut/use-it.md)

You met iterators back in [Concept 27](../27-iterator-adapters/use-it.md): a **stream** of
items you pull one at a time, and [laziness](../27-iterator-adapters/under-the-hood.md) — a
chain does nothing until a consumer pulls it. You've been *using* them happily since. But
one honest question keeps coming back when you read real iterator code:

> Understood — but *how does that actually work?*

This interlude answers exactly that, and it turns out to be one small idea. Once you can
picture it, laziness and "one item at a time" stop being magic words and become obvious.

## The whole idea: an iterator is a cursor

Take the byte-stream you've seen on strings:

```rust
let bytes = "flower".bytes();
```

It is tempting to imagine `bytes` as a little list sitting in memory:

```text
['f', 'l', 'o', 'w', 'e', 'r']   ← NOT what happens
```

It isn't. Making that list would mean copying every letter into a *second* place. An
iterator refuses to do that. Instead it holds just two things:

- a **reference** to the string's bytes (the ones already in memory — it borrows them, it
  doesn't copy them), and
- a single number: the **position** it has reached so far.

That's a **cursor** — like the blinking caret in a text editor. It doesn't hold the text;
it just remembers *where it is* in text that already exists.

![The bytes of flower sit in memory once; the iterator is just a reference to them plus a position number pointing at one byte. next() reads the byte at the position, moves the position forward by one, and hands the byte back wrapped in Some; when the position runs off the end it hands back None.](diagrams/cursor.svg)

## `.next()` is the one move it makes

Every iterator can do exactly one thing, and everything else is built on it: **`.next()`**.
One call does three steps:

1. **Read** the item at the current position.
2. **Advance** the position by one.
3. **Hand back** what it read.

Step 3 has a wrinkle worth naming. What does `.next()` return when the cursor has walked
*off the end* — when there's nothing left to read? It can't return a byte; there is none.
So it returns the "maybe there's a value, maybe there isn't" type you already know from
[Concept 15 — `Option`](../15-option/use-it.md):

- while there's more: `Some(byte)` — "here's a value."
- once it's exhausted: `None` — "the stream is empty now."

You can watch that happen by calling `.next()` by hand. Note the `mut` — each call *changes*
the cursor's position, so the variable has to be mutable:

```rust
let mut bytes = "hi".bytes();

println!("{:?}", bytes.next());   // Some(104)   ← 'h'
println!("{:?}", bytes.next());   // Some(105)   ← 'i'
println!("{:?}", bytes.next());   // None        ← ran off the end
```

(104 and 105 are the byte values of `h` and `i`. `.bytes()` streams the raw
[bytes](../../../languages/rust.md#bytes), which is why you get numbers, not letters.)

## What a `for` loop really is

Here's the payoff. This friendly loop:

```rust
for byte in "hi".bytes() {
    println!("{byte}");
}
```

is just `.next()` in a costume. Rust rewrites it into roughly:

```rust
let mut cursor = "hi".bytes();
loop {
    match cursor.next() {
        Some(byte) => println!("{byte}"),
        None => break,          // None means "stop"
    }
}
```

A `for` loop is *call `.next()` until you get `None`*. That's the whole contract. Everything
that can be looped over — a `Vec`, a range like `0..5`, a `HashMap` — is something that can
hand you a cursor with a `.next()`.

## Build one yourself (the idea made concrete)

The best way to *feel* read-then-advance is to make a cursor of your own. In Rust you do that
by writing a [struct](../13-structs/use-it.md) that holds the position, and giving it a
`.next()` by [implementing](../20-traits/use-it.md) the `Iterator` trait — the shared
"I can be walked one item at a time" behaviour that all iterators wear:

```rust
struct CountUp {
    current: u32,
    limit: u32,
}

impl Iterator for CountUp {
    type Item = u32;                  // each item this cursor hands out is a u32

    fn next(&mut self) -> Option<u32> {
        if self.current == self.limit {
            return None;              // nothing left → the stop signal
        }

        let value_to_hand_back = self.current;   // 1. read HERE
        self.current += 1;                       // 2. advance the position
        Some(value_to_hand_back)                 // 3. hand it back
    }
}

fn main() {
    let counter = CountUp { current: 0, limit: 3 };
    for number in counter {
        println!("{number}");        // 0, then 1, then 2
    }
}
```

Those three lines inside `next` are the exact same three steps the built-in `.bytes()` cursor
takes — read, advance, hand back — just over counting numbers instead of a string's bytes.
`type Item = u32` is the cursor announcing *what kind of thing* it produces; `&mut self` is
there because moving the position is a change to the cursor. Write this once and the whole
language opens up to it: your `CountUp` now works in a `for` loop, and you could chain
`.map`, `.filter`, `.take` onto it too — because all any of those need is something with a
`.next()`.

## Why this is the efficient design

Now the laziness from Concept 27 reads as plain mechanics, not a slogan:

- **An adapter is a cursor wrapping a cursor.** `.map(f)` doesn't transform a list — it
  builds a tiny new cursor whose `.next()` calls the *inner* cursor's `.next()`, applies
  `f` to that one item, and hands the result on. No work until someone pulls.
- **A consumer is what pulls.** `.count()`, `.collect()`, a `for` loop — they sit at the end
  calling `.next()` over and over until `None`. That single pull travels back through every
  wrapped cursor, one item at a time. *That's* why the whole chain is one pass and allocates
  nothing in between.
- **A cursor is cheap.** It's a reference and a number — no second copy of your data, no
  heap allocation. Reaching for `.next()` is just "read at the position, bump the position."

So the answer to *"how does that work?"* is: **every iterator is a small struct holding a
position, and `.next()` reads-there-then-steps-forward.** Adapters stack these structs;
consumers drive them. Nothing more.

## Handbook
Terse reference: [the `Iterator` trait and `.next()`](../../../languages/rust.md#iterator-trait)
and [`.bytes()`](../../../languages/rust.md#bytes).

## Exercises
1. **Build a cursor by hand** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Finish `CountUp`'s `next`: read the current value, advance the position by one, and hand
   the value back wrapped in `Some(...)`. Looping over `CountUp { current: 0, limit: 3 }`
   should print `0`, `1`, `2` (each on its own line).
2. **Watch a real cursor run dry** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Call `.next()` three times on `"hi".bytes()` and print each with `{:?}`. You should see
   `Some(104)`, `Some(105)`, then `None` — the two bytes, then the "empty now" signal.

## Next
- The adapters that put this cursor to work reading two words at once — the exact scan from
  the longest-common-prefix code: [Interlude 28b — `.zip` · `.take` · `.take_while` · `.count`](../28b-zip-take-takewhile-count/use-it.md).
- Where you first met streams and laziness: [Concept 27 — Iterator adapters](../27-iterator-adapters/use-it.md).
