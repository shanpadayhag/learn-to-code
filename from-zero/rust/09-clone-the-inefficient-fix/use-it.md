# Concept 09 · `.clone()` (the inefficient fix) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 08](../08-ownership-and-moves/use-it.md)

## The idea
[Concept 08](../08-ownership-and-moves/use-it.md) left you with a problem: `let s2 = s1`
**moves** a `String`, and then `s1` is gone. But sometimes you genuinely want **two
separate Strings** you can both keep using — maybe even change independently.

The blunt tool for that is **`.clone()`**. It makes a *full, independent copy* — a brand
new piece of heap text, owned by the new variable. Now both variables are real owners of
their own data:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();     // s2 gets its OWN copy of the text
println!("{s1} {s2}");   // ✅ hello hello — both usable
```

No move happened, so `s1` is still valid. `.clone()` sidestepped the whole ownership
transfer by simply making more data to go around. (Quick reference:
[`.clone()` in the handbook](../../../languages/rust.md#to-owned-clone).)

## They're truly independent
Because each owns a separate buffer, changing one never touches the other:

```rust
let original = String::from("cat");
let mut copy = original.clone();
copy.push_str("s");
println!("{original} {copy}");   // cat cats
```

`original` is still `"cat"`; only `copy`'s own buffer grew. Contrast this with a move,
where there was only ever *one* buffer.

## The catch: it isn't free
`.clone()` on a `String` copies **every byte** of the text into a new heap buffer. For
`"hello"` that's nothing. For a 10-megabyte string, cloning copies all 10 megabytes —
every single time you call it. Compare the three things you now know:

| operation | what's copied | cost |
|---|---|---|
| move (`let s2 = s1`) | just the handle | tiny, fixed |
| `Copy` (`let b = a` on `i32`) | the whole value (it's small) | tiny |
| **`.clone()`** on a `String` | the handle **and** all the heap text | grows with length |

So `.clone()` is the **inefficient fix**: it always works and it's easy to reach for, but
it can quietly cost a lot. The classic beginner trap is sprinkling `.clone()` everywhere
just to silence "value moved here" errors — when all you actually wanted was to *look at*
the value, not own a second copy of it.

That "I only wanted to look" case is the entire point of the next concept.

## Exercises
1. **Clone so both live** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Clone `s1` into `s2` and print both. (Expect `hello hello`.)
2. **Independent copies** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Clone a String, grow only the copy, print both. (Expect `cat cats`.)

## Next
- What `.clone()` physically does on the heap, why it's O(n), and how `Copy` and `Clone`
  relate: [Under the hood](under-the-hood.md). Then Concept 10 — borrowing — shows how to
  avoid paying for a copy you didn't need.
