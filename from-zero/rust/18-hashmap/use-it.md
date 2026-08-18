# Concept 18 · `HashMap<K, V>` (look up by key) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 17](../17-vec/use-it.md)

## The idea
A [`Vec`](../17-vec/use-it.md) lets you find things **by position**: "give me the item at
slot 2." But often that's the wrong question. You don't want "the score at slot 2," you want
"the score *for Alice*." You have a **name** and you want the **thing attached to that name**.

That's a **`HashMap<K, V>`** — a table of **key → value** pairs. You look things up by *key*
(`K`) instead of by position. A `HashMap<String, i32>` maps names to scores; a
`HashMap<i32, bool>` maps id numbers to yes/no. And the payoff: finding a key is **near-instant**
no matter how many entries the map holds — it doesn't scan them one by one. (That "why it's
instant" is the whole point of the [hash-map concept](../../../glossary/hash-map.md), and the
[Under the hood](under-the-hood.md) lesson shows the mechanism.)

Picture a coat check: you hand over a **ticket** (the key) and get back *your* **coat** (the
value) — the attendant walks straight to it, they don't check every hook.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 88);   // key "Alice" → value 88
scores.insert("Bob",   92);
```

## Making one
`HashMap` lives in the standard library's collections, so you bring it into scope with a `use`
line first, then `HashMap::new()`:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();   // types filled in by the first insert
scores.insert("Alice", 88);
```

Note the `mut` ([Concept 02](../02-frozen-by-default-and-mut/use-it.md)) — inserting *changes*
the map, so it must be mutable. You never wrote the types: Rust sees `insert("Alice", 88)` and
infers `HashMap<&str, i32>` on its own.

## Reading a value
Ask for a key with `.get(&key)`. Just like [`Vec::get`](../17-vec/use-it.md#reading-items), it
hands back an [`Option`](../15-option/use-it.md) — `Some(value)` if the key is there, `None` if
it isn't — because the key **might not exist**, and Rust makes you handle that instead of
crashing:

```rust
match scores.get("Alice") {
    Some(score) => println!("Alice has {score}"),
    None => println!("no score for Alice"),
}
```

This is [`Option`](../15-option/use-it.md) doing its job again: "no such key" is a value you
handle, not a surprise crash. (`.get` takes `&key` — a [borrow](../10-borrowing-with-ref/use-it.md)
of the key — because it only needs to *read* the key to look it up, not take ownership of it.)

To just check presence, `.contains_key(&key)` returns a plain `bool`.

## Insert overwrites
Inserting a key that's already there **replaces** the old value:

```rust
scores.insert("Alice", 88);
scores.insert("Alice", 100);   // Alice is now 100, not 88 — one value per key
```

A key maps to exactly one value. Insert again and the new value wins.

## The `entry` shortcut: "get me the slot, make it if missing"
A super common need: "add one to Alice's count — but if Alice isn't in the map yet, start her at
zero." Done the long way that's an annoying *check-then-insert* dance. The `entry` API does it in
one line:

```rust
let mut counts = HashMap::new();
for word in ["sun", "sea", "sun"] {
    *counts.entry(word).or_insert(0) += 1;
}
// counts: {"sun": 2, "sea": 1}
```

Read `entry(word).or_insert(0)` as: "hand me the value-slot for `word`; if there's no entry yet,
put a `0` there first." It hands back a *reference* to that slot, so `*... += 1` bumps it. This
counting move — `entry(k).or_insert(0)` then `+= 1` — is the single most common thing people do
with a `HashMap`.

## Walking every pair
A `for` loop over `&the_map` visits each **(key, value)** pair (the `&` borrows the map so the
loop just reads it — [Concept 10](../10-borrowing-with-ref/use-it.md)):

```rust
for (name, score) in &scores {
    println!("{name}: {score}");
}
```

One catch worth knowing now: the order is **not** predictable — a `HashMap` doesn't keep pairs in
insertion order or sorted order (the [Under the hood](under-the-hood.md) lesson explains why).
If you need sorted keys, that's what [`BTreeMap`](../../../languages/rust.md#btreemap) is for.

## A few more everyday moves
- `map.len()` — how many pairs.
- `map.contains_key(&key)` — `true`/`false` without pulling the value out.
- `map.remove(&key)` — take a key out, returning its old value as an `Option`.

## Exercises
1. **Count the words** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Given a sentence, count how many times each word appears using
   `entry(word).or_insert(0)`. For `"the cat sat on the mat"`, print each word with its count
   (order will vary). (Expect `the` → `2` and every other word → `1`.)
2. **Safe lookup** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Build a `HashMap` of three name → score pairs, then use `.get(name)` and `match` to look up a
   name that exists and one that doesn't. (Expect `Alice: 88`, then `no score for Carol`.)

## Next
- What "look it up instantly" actually *does* in memory — how a **hash function** turns a key
  into a *slot number* so the map jumps straight to the value instead of scanning, why that makes
  lookups ~O(1), why the order comes out scrambled, and how the map owns its keys and values out
  on the heap: [Under the hood](under-the-hood.md).
