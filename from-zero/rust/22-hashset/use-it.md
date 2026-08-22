# Concept 22 · `HashSet<T>` (membership, no duplicates) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 21](../21-trait-objects/use-it.md)

## The idea
[`HashMap`](../18-hashmap/use-it.md) answered *"what value is attached to this key?"* Often you don't
care about an attached value at all — you only want to answer one yes/no question:

> **"Have I seen this before?"**

Is this username taken? Is this number already in my list? Did this word show up already? You don't
need a value stored against it — you just need to know whether the thing is **in the collection or
not**. That's a **`HashSet<T>`**: a bag of values where each value is either present or absent, and
**no duplicates are ever kept**.

Think of it as a `HashMap` with the values thrown away — just the keys. In fact that's almost exactly
what it is under the hood (next lesson), so it inherits the map's superpower: checking membership is
**~O(1)** ([Big-O](../../../glossary/big-o-notation.md)), near-instant, no matter how many values are
in the set.

## Making one and putting things in
`HashSet` lives in the same module as `HashMap`, so you `use` it, then `insert`:

```rust
use std::collections::HashSet;

let mut fruits: HashSet<String> = HashSet::new();
fruits.insert(String::from("apple"));
fruits.insert(String::from("pear"));
fruits.insert(String::from("apple"));   // ignored — "apple" is already in

println!("{}", fruits.len());           // 2, not 3
```

`insert` needs [`mut`](../../languages/rust.md#let-mut) because it changes the set. The third insert
does **nothing**: `"apple"` is already a member, and a set never stores a second copy. That automatic
**de-duplication** is the first thing sets give you for free — the count is 2.

## `insert` hands you back a "was it new?" bool
This is the detail that makes sets so useful. `insert` **returns a `bool`**:

- `true`  → the value was **new** (it got added), and
- `false` → the value was **already there** (nothing changed).

So a single `insert` call is *both* "remember this" *and* "had I already seen it?" rolled into one:

```rust
let mut seen = HashSet::new();
if !seen.insert(5) {
    println!("5 is a repeat!");   // runs only the *second* time you insert 5
}
```

You'll lean on this constantly: walk a stream of items, `insert` each, and the moment `insert` returns
`false` you've caught a duplicate — no separate "check then add" needed.

## Asking "is it in there?"
To just test membership without inserting, use `contains`:

```rust
let primes: HashSet<i32> = HashSet::from([2, 3, 5, 7]);
println!("{}", primes.contains(&7));   // true
println!("{}", primes.contains(&8));   // false
```

`contains` takes a **reference** (`&7`) — it's only *looking*, not taking ownership, the same borrow
you met with [`&`](../10-borrowing-with-ref/use-it.md). And `HashSet::from([...])` builds a set
straight from an array, deduping as it goes.

## Why not just a `Vec` and `.contains()`?
A [`Vec`](../17-vec/use-it.md) has a `.contains()` too — so why a whole new type? **Speed, and at
scale it's night and day.** `vec.contains(&x)` has to **scan** — walk the list comparing every element
until it finds `x` or runs out. That's **O(n)**: a million-item `Vec` may do a million comparisons for
one check. A `HashSet` **hashes** `x` straight to its slot and looks only there — **~O(1)**, a handful
of steps regardless of size. Do that check in a loop (the exercises do) and it's the difference between
O(n²) and O(n).

The trade you accept: a set is **unordered** (values sit wherever their hash sends them, not in insert
order) and holds **no duplicates**. When you specifically need *fast membership* or *automatic dedup*,
that's exactly the trade you want. When you need order or repeats, stay with `Vec`.

| you want… | reach for |
|---|---|
| "is X present?" fast, or drop duplicates | **`HashSet<T>`** |
| a value attached to each key | [`HashMap<K, V>`](../18-hashmap/use-it.md) |
| order, positions, or duplicates kept | [`Vec<T>`](../17-vec/use-it.md) |

## Exercises
1. **Count the unique values** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Insert a list of numbers (with repeats) into a `HashSet<i32>` and print how many *different* values
   there were. (Expect `7`.)
2. **First value that repeats** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Use `insert`'s returned bool to find the first value that shows up twice in a stream, returning an
   [`Option`](../15-option/use-it.md). (Expect `first repeat: 2`.)

Handbook: [`HashSet<T>` — a set of unique values](../../languages/rust.md#hashset).

## Next
- A `HashSet` is *literally* a `HashMap<T, ()>` — the same bucket array and the same hashing you saw in
  [Concept 18](../18-hashmap/under-the-hood.md), but each slot stores **only the key** and a zero-size
  placeholder value. That's why membership is ~O(1) and why there's no order: see the shared machinery
  in [Under the hood](under-the-hood.md).
