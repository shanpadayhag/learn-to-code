# Concept 12 · Slices — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 11](../11-mut-references-and-borrow-rules/use-it.md)

## The idea
A `&` borrow ([Concept 10](../10-borrowing-with-ref/use-it.md)) lends out a *whole* value.
But often you want to refer to just **part** of one — the first word of a sentence, a
middle chunk of text — without copying that part into a new `String`.

A **slice** is exactly that: a reference to a *contiguous range* of a value. For a
`String` you write `&s[start..end]`, and you get back a **string slice**, whose type is
`&str`:

```rust
let s = String::from("hello world");
let hello = &s[0..5];    // "hello"
let world = &s[6..11];   // "world"
```

The `..` is a **range**, and the end is **exclusive**: `[0..5]` covers bytes 0, 1, 2, 3, 4.
No new text is created — `hello` and `world` are windows pointing into `s`'s own buffer.

## Range shorthands
You can drop either side of the range:

```rust
let s = String::from("Rustacean");
let front = &s[..4];   // "Rust"   — from the start
let back  = &s[4..];   // "acean"  — to the end
let whole = &s[..];    // "Rustacean" — everything
```

- `[..n]` — from the beginning up to `n`.
- `[n..]` — from `n` to the end.
- `[..]` — the whole thing.

## You've been using slices since day one
Here's the loop closing. Way back in [Concept 07](../07-the-heap-and-string/use-it.md), a
bare literal like `"hello"` was said to have type `&str`, *not* `String`. Now you can see
why: **`"hello"` is a slice** — a `&str` pointing at text baked into your program. A string
slice is a string slice whether it points into a `String`'s heap buffer or into the
program itself. That's the same `&str` you get from `&s[0..5]`.

This is why functions that only read text usually take `&str`, not `&String`: a `&str`
accepts *both* a slice of a `String` and a plain literal, so it's the more flexible reader.

## Slices are borrows
A slice references data it doesn't own, so [Concept 11](../11-mut-references-and-borrow-rules/use-it.md)'s
rules apply. While a slice is borrowing a `String`, you can't grow that `String` — the
compiler stops you:

```rust
let mut s = String::from("hello");
let part = &s[0..3];
s.push_str("!!!");     // ❌ error[E0502]: cannot borrow `s` as mutable...
println!("{part}");
```

That's the borrow checker protecting the slice — for a reason you'll see in
[Under the hood](under-the-hood.md).

## Exercises
1. **Two windows** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Slice `"hello world"` into `"hello"` and `"world"`. (Expect `hello world`.)
2. **Shorthand ranges** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Use `[..4]` and `[4..]` to split `"Rustacean"`. (Expect `Rust acean`.)

## Next
- What a slice actually stores, why growing the borrowed `String` is forbidden, and how
  `&str` relates to `String`: [Under the hood](under-the-hood.md).
