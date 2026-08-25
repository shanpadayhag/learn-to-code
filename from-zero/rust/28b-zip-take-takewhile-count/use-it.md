# Interlude 28b · Scanning two streams together — `.zip` · `.take` · `.take_while` · `.count`

> Interlude: a **single lesson**. It reuses the cursor picture from
> [Interlude 28a](../28a-how-next-works/use-it.md) and the laziness from
> [Concept 27](../27-iterator-adapters/under-the-hood.md), so it adds no new memory page.
> Track: [From-Zero: Rust](../README.md) · Previous: [Interlude 28a](../28a-how-next-works/use-it.md)

[Concept 27](../27-iterator-adapters/use-it.md) gave you `.map`, `.filter`, and `.collect`.
This interlude adds the four adapters that show up whenever you **compare two sequences
letter by letter** — reading real code like this and wanting every piece to click:

```rust
let matching_length = first
    .bytes()
    .zip(word.bytes())
    .take(length)
    .take_while(|(a, b)| a == b)
    .count();
```

That chain finds how long a **common prefix** two words share. We'll build it up one adapter
at a time, then read the whole thing. Every adapter here is just another
[cursor wrapping a cursor](../28a-how-next-works/use-it.md); nothing runs until `.count()` at
the end pulls on it.

## `.zip` — walk two cursors in lockstep

`.zip(other)` takes a second iterator and pairs items up: first-with-first,
second-with-second, and so on. It hands out **tuples**.

```rust
let first  = "flower".bytes();
let second = "flight".bytes();

for pair in first.zip(second) {
    println!("{pair:?}");   // (102, 102), (108, 108), (111, 105), ...
}
```

Picture the two byte-cursors stepping forward together:

```text
"flower":  f  l  o  w  e  r
"flight":  f  l  i  g  h  t
zip:      (f,f)(l,l)(o,i)(w,g)(e,h)(r,t)
```

**The rule that matters: `.zip` stops the moment *either* cursor runs out.** The clearest way
to see that is with two words of *different* lengths — `"flower"` and `"flow"`:

```text
"flower":  f  l  o  w  e  r
"flow":    f  l  o  w
zip:      (f,f)(l,l)(o,o)(w,w)   ← then "flow" has no 'e' to pair with, so zip ends
```

There is no `(e, ?)` pair, because there is no fifth byte in `"flow"` to pair the `e` with.
`.zip` can only make a pair when *both* sides can hand it a next item.

![Two byte-cursors for flower and flight step forward together; zip pairs them (f,f) (l,l) (o,i); take_while keeps the matching pairs and stops for good at (o,i); count returns 2. Below, flower zipped with flow stops after (w,w) because flow has no fifth byte to pair.](diagrams/zip-scan.svg)

## `.take(n)` — at most `n`, then stop

`.take(n)` is a cursor that passes along **up to** `n` items and then reports itself empty:

```rust
let first_three: Vec<u8> = "flower".bytes().take(3).collect();
println!("{first_three:?}");   // [102, 108, 111]  ← just f, l, o
```

Two things beginners double-check, both true:

- **`n` counts *items*, not anything else.** `.take(3)` means "let three pairs through," full
  stop. (In the prefix code, the number is called `length` — the prefix length found so far —
  but it's still just "how many pairs to allow.")
- **It's *up to* `n`, not *exactly* `n`.** If the stream only has 4 pairs and you write
  `.take(6)`, you get the 4 that exist and it stops there — asking for 6 never invents more.

## `.take_while(test)` — keep going *while* it passes, then stop for good

`.take_while` takes a [closure](../26-closures/use-it.md) that returns `true`/`false`, and
hands items along **only up to the first `false`** — then it's done, permanently.

```rust
let leading_small: Vec<i32> = [1, 2, 9, 3, 4]
    .iter()
    .copied()
    .take_while(|n| *n < 5)     // 1 ✓  2 ✓  9 ✗ → stop
    .collect();
println!("{leading_small:?}");  // [1, 2]
```

This is the one to hold next to `.filter` from Concept 27, because they look similar and do
opposite things at the mismatch:

| | on an item that **fails** the test |
|---|---|
| `.filter(test)` | **skips** it and keeps walking to the very end |
| `.take_while(test)` | **stops** — that item and everything after it are gone |

For the same `[1, 2, 9, 3, 4]`: `.filter(|n| *n < 5)` gives `[1, 2, 3, 4]` (it skips the 9),
but `.take_while(|n| *n < 5)` gives `[1, 2]` (it halts at the 9). When you're finding a
*prefix* — a run at the **front** — stopping is exactly what you want.

The `|(a, b)|` in the prefix code is that same closure, destructuring the tuple `.zip` made:
it names the two bytes `a` and `b` and keeps the pair while `a == b`.

## `.count()` — the consumer that drives it all

Everything above is lazy — pure setup, no work done yet. `.count()` is a **consumer**
([Concept 27](../27-iterator-adapters/under-the-hood.md)): it pulls `.next()` over and over
until the stream ends, and returns **how many items came through**.

```rust
let vowels = "flower".bytes().filter(|b| b"aeiou".contains(b)).count();
println!("{vowels}");   // 2  ← 'o' and 'e'
```

Because `.count()` is what finally pulls, it's the moment the whole chain actually runs —
one byte-pair at a time, all the way through the cursors stacked above it.

## Reading the whole scan

Now the original chain reads as one plain sentence. For `first = "flower"`, `word = "flight"`,
and `length = 6`:

```rust
first.bytes()                    // f l o w e r
    .zip(word.bytes())           // (f,f) (l,l) (o,i) (w,g) (e,h) (r,t)
    .take(length)                // allow up to 6 pairs (no cap in practice here)
    .take_while(|(a, b)| a == b) // (f,f)✓ (l,l)✓ (o,i)✗ → stop
    .count();                    // 2 pairs made it through
```

Pull it one pair at a time — only ever **one pair in flight**, exactly like the lazy pull you
saw in Concept 27:

| pull | zip yields | take (≤6?) | take_while (`a == b`?) | count so far |
|------|-----------|-----------|------------------------|--------------|
| 1 | `(f, f)` | ok | keep ✓ | 1 |
| 2 | `(l, l)` | ok | keep ✓ | 2 |
| 3 | `(o, i)` | ok | **stop ✗** | 2 |
| — | (rest never pulled) | — | — | **2** |

So `matching_length` is `2` — `"flower"` and `"flight"` share the prefix `"fl"`. The `.take(length)`
looks idle here, but its job shows across *many* words: once an earlier word has shrunk the
shared prefix to, say, `4`, `.take(4)` stops later comparisons from bothering to look past the
best answer still possible.

## Handbook
Terse reference: [`.zip`](../../../languages/rust.md#zip),
[`.take`](../../../languages/rust.md#take), [`.take_while`](../../../languages/rust.md#iterator-adapters),
[`.count`](../../../languages/rust.md#count).

## Exercises
1. **Shared-prefix length** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   For `"flower"` and `"flight"`, build `first.bytes().zip(second.bytes()).take_while(|(a, b)| a == b).count()`
   and print it. (Expect `2`.) This is the heart of the longest-common-prefix code, in one line.
2. **`take_while` vs `filter`** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Given `vec![2, 4, 5, 6, 8]`, collect one `Vec<i32>` with `.filter(even)` and another with
   `.take_while(even)`, and print both. Watch `filter` skip the `5` (→ `[2, 4, 6, 8]`) while
   `take_while` halts at it (→ `[2, 4]`).

## Next
- The reference for these on one page: the [iterator adapters](../../../languages/rust.md#iterator-adapters) handbook entry.
- Back to where the main course is heading after iterators: the [Rust roadmap](../README.md).
