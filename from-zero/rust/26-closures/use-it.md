# Concept 26 · Closures (`|x| ...`) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 25](../25-lifetimes/use-it.md)

## The idea
A **closure** is a function you write *inline*, right where you need it, usually with no name.
You've already seen them — the `|(a, b)| a == b` inside the
[Longest Common Prefix](../../../challenges/longest-common-prefix/solution.rs.md) solution was a
closure. This lesson makes them make sense.

Two things make closures more than "a shorter function":
1. They're small enough to write **inline**, so you can hand a bit of behaviour straight to
   another function (like an iterator's `.filter` or `.map`).
2. They can **capture** variables from the code around them — something a plain `fn` cannot do.
   That capture is the whole point, and it's where the memory story lives.

## The syntax
A closure is `|parameters| body`:

```rust
let add = |a, b| a + b;
println!("{}", add(2, 3));   // 5
```

The bit between the pipes `|...|` is the parameter list; the bit after is the body, and its value
is what the closure returns. You call it like any function: `add(2, 3)`. Types are usually
**inferred** from how you use it, so you rarely write them — though you can:
`|a: i32, b: i32| -> i32 { a + b }` for a multi-line body.

## The superpower: capturing the environment
Here's what a closure can do that a named function can't — **use a variable from the surrounding
scope**:

```rust
let factor = 3;
let scale = |x| x * factor;   // `factor` is captured from outside
println!("{}", scale(10));    // 30
```

The closure reached out and grabbed `factor`. Try the same with a plain `fn` and it won't
compile:

```rust
let factor = 3;
fn scale(x: i32) -> i32 { x * factor }   // ❌
// error[E0434]: can't capture dynamic environment in a fn item
```

A `fn` is a standalone thing with no connection to where it's written, so it can only see its own
parameters. A closure is *tied to the spot it's created* and can borrow the locals there. That's
the difference in one sentence.

![A closure that uses `factor` becomes a struct storing factor plus the code; a plain fn has no such data box](diagrams/closure-is-data-plus-code.svg)

## Where closures shine: handing behaviour to iterators
This is their day job. Iterator adapters like `.filter`, `.map`, and `.take_while` each take a
closure describing *what to do with each item* — and the closure can capture locals to decide:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let min = 3;
let count = numbers.into_iter().filter(|&n| n >= min).count();   // 3
```

`|&n| n >= min` captures `min` and gets applied to every item. This is exactly the shape of your
challenge's `.take_while(|(a, b)| a == b)` — a closure, capturing nothing, handed to an adapter to
run on each pair. (The full iterator toolbox — `.map`, `.filter`, `.collect`, and friends — is the
rest of this phase; closures are the piece that makes them go.)

## `move` — capture by taking ownership
By default a closure borrows what it captures. Sometimes it needs to **own** it instead — most
often when the closure has to *outlive* the scope it was made in, like being returned from a
function:

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n            // `move` = take ownership of `n` into the closure
}

let add5 = make_adder(5);
println!("{}", add5(10));     // 15
```

Without `move`, the closure would try to *borrow* `n` — but `n` dies when `make_adder` returns
(remember [lifetimes](../25-lifetimes/use-it.md): a borrow can't outlive its value), so the
borrow would dangle. `move` copies `n` *into* the closure so it carries its own copy and can
safely live on. `impl Fn(i32) -> i32` in the return type just means "some closure that takes an
`i32` and returns an `i32`" — the exact type is unnameable, so we describe it by what it can do.

## Exercises
1. **A closure that captures** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Make a local `tax_rate = 0.2`, then a closure `with_tax = |price| price + price * tax_rate`
   that captures it. Print `with_tax(100.0)` and `with_tax(50.0)` with one decimal (`{:.1}`).
2. **A closure with an iterator** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Given `vec![1, 2, 3, 4, 5]` and a local `min = 3`, use `.into_iter().filter(...).count()` with
   a closure that captures `min` to count how many numbers are `>= min`. (Expect `3`.)

## Next
- Why a closure is really a little **struct that bundles its captured variables with its code**,
  how "read / mutate / move" decides whether it captures by `&`, `&mut`, or ownership (the
  `Fn` / `FnMut` / `FnOnce` traits), and why all of this is **zero-cost** at runtime:
  [Under the hood](under-the-hood.md).
