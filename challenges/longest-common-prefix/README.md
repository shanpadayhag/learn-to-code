# Longest Common Prefix

| | |
|---|---|
| Date       | 2026-08-24 |
| Language   | Rust |
| Source     | [LeetCode 14 — Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/) (practice variant: read `N`, then `N` lowercase words from input) |
| Lessons    | [`break` only leaves the nearest loop + labels](../../languages/rust.md#loop-control) · [`.unwrap()` is a crash-on-`None` bet](../../languages/rust.md#unwrap) · [`.zip()` / `.take_while()` / `.count()`](../../languages/rust.md#iterator-adapters) · [`let … else`](../../languages/rust.md#let-else) |

## The Task
The first line of input is a number `N`. The next `N` lines are each one word (lowercase
letters). Print the **longest prefix shared by all `N` words** — the letters they all start
with. If they share no starting letter, print an empty line.

```
3
flower
flow
flight        ->  fl        ("fl" is the longest start common to all three)
```

## My First Attempt — the way I already knew

Walk letter positions in an outer loop; for each position, compare every neighbouring pair of
words in an inner loop; if a pair's letters match all the way to the last pair, record that
letter ([initial.rs](initial.rs)):

```rust
for character_index in 0..=9999 {          // outer: each letter position
    for index in 0..=(n - 1) {             // inner: each word paired with the next
        if index == n - 1 { break; }
        let character1 = words[index].chars().nth(character_index);
        let character2 = words[index + 1].chars().nth(character_index);
        if let Some(character1) = character1 {
            if let Some(character2) = character2 {
                if character1 == character2 {
                    if index == n - 2 { common_letters.push(character1); }
                } else {
                    break;                  // ← meant to "stop searching"
                }
            }
        }
    }
}
```

It printed `fl` for the sample. It looks like it works. It doesn't.

### Why it's not the answer

**1. It's wrong — a real bug (the one that taught the most).** That `break` was meant to say
"a mismatch means the common prefix is over — stop everything." But
[`break` only leaves the **nearest** loop](../../languages/rust.md#loop-control) — here, the
inner one. The outer loop marched on to the next letter position and kept recording matches it
should never have examined. On `ba / ca / ba`:

| position | inner loop | should have… |
|---|---|---|
| 0 | `b` vs `c` differ → `break` inner | stopped **everything** — answer is `""` |
| 1 | outer ran anyway; `a`==`a`==`a` → records `a` ❌ | never run |

So it prints `a` when the correct answer is empty. The sample words happened to hide it — the
most dangerous kind of bug. The direct fix is a **labeled loop** (`break 'search`), written up
in [From-Zero Interlude 05b](../../from-zero/rust/05b-break-continue-and-labels/use-it.md).

**2. It's slow.** `word.chars().nth(k)` doesn't jump to letter `k` — a string is walked one
character at a time, so `.nth(k)` re-walks from the start every call, costing `O(k)`. Buried in
two nested loops, the whole thing is roughly **`O(N · L²)`** for `L`-length words.

**3. The plumbing fought me.** Reading two `Option<char>`s forced a nested `if let` staircase,
and my instinct — one `if let A && let B` — didn't compile
([let-chains need the 2024 edition](../../from-zero/rust/15a-opening-options-safely/use-it.md)).

## The Trick — compare against the first word, and let iterators do the walking

Two shifts turn all three problems into non-problems.

**Compare every word to the first one, not neighbour-to-neighbour.** A prefix common to *all*
words is exactly a prefix of `words[0]` that every other word also starts with. So carry a
"how many letters still agree" length, start it at the full first word, and shrink it against
each other word. No grid, so no nested-loop `break` trap at all — the better fix here isn't the
label, it's *not nesting*.

**Let one iterator chain do the character walk**, reading each letter exactly once:

```rust
length = first.bytes()                    // the first word's letters, one pass
    .zip(word.bytes())                    // pair them with this word's; stops at the shorter
    .take(length)                         // don't look past the prefix we still believe in
    .take_while(|(a, b)| a == b)          // keep going while letters match; stop at the first that doesn't
    .count();                             // how many matched = the new agreed length
```

- [`.zip()`](../../languages/rust.md#iterator-adapters) walks two sequences together and ends
  the moment either runs out — so a shorter word caps the prefix for free (no `Option`
  juggling, no bounds check).
- [`.take_while()`](../../languages/rust.md#iterator-adapters) stops at the first mismatch —
  the "stop everything" my broken `break` was reaching for, expressed directly.
- `.count()` is the surviving length.

Comparing `bytes()` is safe here because the input is lowercase letters (one byte each). It's
also faster than `chars()`, which decodes UTF-8 as it goes.

## Watch it run — `flower / flow / flight`

`length` starts at `6` (all of `flower`) and only ever shrinks:

| against | zipped letters compared | matched run | `length` |
|---|---|---|---|
| start (`flower`) | — | — | 6 |
| `flow`   | `f=f, l=l, o=o, w=w`, then `flow` ends | 4 | 4 |
| `flight` | `f=f, l=l`, then `o≠i` → stop | 2 | 2 |

Final `length` is `2`, so the answer is the first two letters of `flower` →
[`&first[..2]`](../../languages/rust.md#slice) → **`fl`**. And on `ba / ca / ba`: against `ca`,
`b≠c` immediately → `length` becomes `0`, we `break`, and `&first[..0]` is `""` — correct.

## The Answer

[solution.rs](solution.rs) — no nested loops, each letter read once, zero new allocations:

```rust
fn longest_common_prefix<'a>(words: &[&'a str]) -> &'a str {
    let Some(first) = words.first() else {
        return "";
    };

    let mut length = first.len();
    for word in &words[1..] {
        length = first.bytes()
            .zip(word.bytes())
            .take(length)
            .take_while(|(a, b)| a == b)
            .count();
        if length == 0 {
            break;
        }
    }

    &first[..length]
}
```

[`let … else`](../../languages/rust.md#let-else) handles the "no words at all" case up front and
returns, so the rest reads as one flat happy path. The return type `&'a str` means the answer is
a **slice borrowed out of the first word** — we build no new `String`, so there's nothing to
allocate or copy.

**Time:** `O(N · L)` where `L` is the shortest word — each word is scanned once, only across the
letters that still agree. That's optimal: you can't know the common prefix without looking at
its letters in every word. **Space:** `O(1)` extra — a single `length` counter; the result
borrows the first word rather than copying it. The first attempt was `O(N · L²)` time and built
a `String`; this is faster *and* lighter.

## Takeaway

- **`break` leaves only the nearest loop.** My whole bug. When you truly need to bail out of an
  outer loop, [label it and `break 'label`](../../languages/rust.md#loop-control) — or, better,
  restructure so you aren't nesting in the first place.
- **`.chars().nth(k)` is `O(k)`, not `O(1)`.** A string isn't an array of characters you can
  index into; walking positions with `.nth()` inside a loop is quietly quadratic. Reach for an
  iterator that makes a single pass.
- **`.zip()` + `.take_while()` + `.count()` is the "compare two sequences until they diverge"
  combo.** It replaced a hand-written double loop, an `Option` staircase, *and* the buggy
  `break` in one readable line.
- **Compare-to-the-first beats compare-each-neighbour** for "common to all," and returning a
  **borrowed slice** (`&str`) keeps it allocation-free.
- **"It passes" ≠ "it's correct."** `flower/flow/flight` passed while `ba/ca/ba` was silently
  wrong. Test the case that *should* fail, not just the one you were given.
- **Comparing `bytes()` is a deliberate bet on the input, not a free lunch.** It's the right,
  fast call *here* because the task guarantees lowercase letters — one byte per character — so
  `length` always lands on a real character boundary and [`&first[..length]`](../../languages/rust.md#slice)
  can never split one. On **arbitrary Unicode** (accents, emoji — where one character is several
  bytes) that same slice could cut a character in half and **panic**, and matching a shared byte
  prefix wouldn't even mean a shared *character* prefix. The general-purpose version compares
  [`.chars()`](../../languages/rust.md#string-indexing) instead and tracks the boundary in bytes
  (e.g. sum each matched `char::len_utf8()`), trading a little speed — UTF-8 has to be decoded —
  for correctness on any text. Match the tool to the guarantees you actually have.

Related From-Zero lessons this surfaced:
[05b — break/continue/labels](../../from-zero/rust/05b-break-continue-and-labels/use-it.md) ·
[15a — opening an Option safely](../../from-zero/rust/15a-opening-options-safely/use-it.md).
