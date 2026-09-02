# Longest Common Prefix

| | |
|---|---|
| Date       | 2026-08-24 |
| Difficulty | Easy |
| Languages  | Rust |
| Pattern    | [Horizontal scanning](#the-trick) (compare every word to the first) |
| Time/Space | O(N · L) / O(1) |
| Source     | [LeetCode 14 — Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/) (practice variant: read `N`, then `N` lowercase words) |

## The Problem
You're given `N` words (lowercase letters). Find the **longest run of letters they all
start with** — their longest shared *prefix*. If they don't even share a first letter, the
answer is empty.

What matters:
- The answer is always a prefix of **every** word, so it can't be longer than the shortest word.
- "Share no prefix" is a real case — print an empty line, don't crash.
- Words can be up to a few thousand letters, so re-scanning them repeatedly is what we must avoid.

Tiny example:
```
words = [flower, flow, flight]
answer = fl                     because all three start "fl", and flight breaks the streak at the 3rd letter
```

## Understand It

### In plain words
Picture three books lined up on a shelf, all titled something like *"flower…", "flow…",
"flight…"*. You run your finger along the spines letter by letter, all at once: `f` — yes,
all three. `l` — yes, all three. `o` — wait, the third one says `i`. Stop. The letters
everyone agreed on so far — `fl` — is the answer. The moment **one** book disagrees, the
shared beginning is over.

### The slow, obvious way
My first instinct was to compare **neighbouring pairs** of words, column by column: is
`words[0]`'s letter here the same as `words[1]`'s? And `words[1]` vs `words[2]`? Walk every
letter position in an outer loop, every pair in an inner loop, and record a letter when the
whole column agrees.

It *looked* fine and even printed `fl` for the sample — but it had a real **bug**. When a
column disagreed I called `break` to "stop searching," like this:

```rust
for character_index in 0..=9999 {        // outer: each letter position
    for index in 0..words.len() - 1 {    // inner: each neighbouring pair
        if letters_differ {
            break;                        // ← meant "stop everything"
        }
        // ...else record the matching letter...
    }
}
```

But [`break` only leaves the **nearest** loop](../../languages/rust.md#loop-control) — the
inner one. The outer loop marched on to the next letter position and kept recording matches it
should never have looked at. On `ba / ca / ba`:

| position | inner loop does | should have… |
|---|---|---|
| 0 | `b` vs `c` differ → `break` inner | stopped **everything** — answer is `""` |
| 1 | outer ran anyway; `a`==`a`==`a` → records `a` ❌ | never run |

So it printed `a` when the answer is empty. The sample words hid it — the most dangerous kind
of bug. And it was **slow**: reading letter `k` of a word with `word.chars().nth(k)` re-walks
the word from the start every call (`O(k)`), so nested in two loops the whole thing is roughly
`O(N · L²)`.

### The trick
Two shifts fix the bug *and* the speed at once.

**Compare every word to the first word, not neighbour-to-neighbour.** A prefix shared by *all*
words is exactly a prefix of `words[0]` that every other word also starts with. So keep a
single number — "how many letters still agree" — start it at the full length of the first
word, and shrink it against each other word. There's no grid, so the nested-loop `break` trap
simply can't happen.

**Let one iterator do the letter-walk, reading each letter once.** Rust's iterators
([the tools that will get their own phase later](../../from-zero/rust/README.md)) chain
together to say exactly what we mean:

```rust
length = first.bytes()                 // the first word's letters, one forward pass
    .zip(word.bytes())                 // pair each with this word's letters; ends at the shorter word
    .take(length)                      // never look past the prefix we still believe in
    .take_while(|(a, b)| a == b)       // keep going while letters match; stop at the first that differs
    .count();                          // how many matched = the new agreed length
```

- [`.zip()`](../../languages/rust.md#iterator-adapters) walks two sequences together and stops
  the instant either runs out — so a short word caps the prefix for free.
- [`.take_while()`](../../languages/rust.md#iterator-adapters) stops at the first mismatch —
  that's the honest version of the "stop everything" my broken `break` was reaching for.
- `.count()` is the surviving length.

Why this is genuinely fast, not just tidy: each word is scanned **once**, and only across the
letters that still agree. The `.chars().nth(k)` version re-walked from the start every single
letter — that repeated re-walk is exactly the `O(L²)` we're deleting. Reading each letter once
is what turns `O(N · L²)` into `O(N · L)`.

### Watch it run — `flower / flow / flight`
`length` starts at `6` (all of `flower`) and only ever shrinks:

| against | letters compared | matched run | `length` |
|---|---|---|---|
| start (`flower`) | — | — | 6 |
| `flow`   | `f=f, l=l, o=o, w=w`, then `flow` ends | 4 | 4 |
| `flight` | `f=f, l=l`, then `o≠i` → stop | 2 | 2 |

Final `length` is `2`, so the answer is the first two letters of `flower`. On `ba / ca / ba`:
against `ca`, `b≠c` immediately → `length` becomes `0`, and we're done.

### The answer
Take that many letters off the front of the first word:
[`&first[..length]`](../../languages/rust.md#slice) → **`fl`**. It's correct because `length`
is, by construction, the largest count that *every* word agreed on — shrunk down the moment any
word disagreed, never grown back.

## The Code

### Rust
```rust
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let owned_words: Vec<String> = (0..n).map(|_| lines.next().unwrap().unwrap()).collect();
    let words: Vec<&str> = owned_words.iter().map(|word| word.trim()).collect();

    println!("{}", longest_common_prefix(&words));
}

fn longest_common_prefix<'a>(words: &[&'a str]) -> &'a str {
    if words.is_empty() {
        return "";
    }
    let first = words[0];

    let mut length = first.len();
    for word in &words[1..] {
        length = first
            .bytes()
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

**Time:** O(N · L) — `N` words, each scanned once across the `L` letters that still agree. This
is optimal: you can't confirm a shared prefix without reading its letters in every word.
**Space:** O(1) extra — one `length` counter; the answer is a slice **borrowed out of the first
word**, so nothing is allocated or copied.
**Syntax notes:** [solution.rs.md](solution.rs.md) — including the two constructs new to this
repo: the `&[&'a str]` parameter type and the `|(a, b)| a == b` closure.

## Remember This
- **`break` leaves only the nearest loop.** My whole bug. To bail out of an outer loop, [label it and `break 'label`](../../languages/rust.md#loop-control) — or, better, restructure so
  you're not nesting at all (which is what "compare to the first word" achieved here).
- **`.chars().nth(k)` is `O(k)`, not `O(1)`.** A string isn't an array you can jump into;
  walking positions with `.nth()` inside a loop is quietly quadratic. Reach for one forward pass.
- **`.zip()` + `.take_while()` + `.count()`** is the "compare two sequences until they diverge"
  combo — it replaced a double loop, an `Option` staircase, and the buggy `break` in one line.
- **Compare-to-the-first beats compare-each-neighbour** for "common to all," and returning a
  **borrowed `&str`** keeps it allocation-free.
- **Comparing `bytes()` is a deliberate bet on the input.** It's the fast, correct call *here*
  because the task guarantees lowercase letters (one byte per character), so `length` always
  lands on a real character boundary. On **arbitrary Unicode** (accents, emoji — one character,
  several bytes) that byte-slice could cut a character in half and **panic**, and a shared
  *byte* prefix wouldn't even mean a shared *character* prefix. The general version compares
  [`.chars()`](../../languages/rust.md#string-indexing) and tracks the boundary with
  `char::len_utf8()`, trading a little speed for correctness on any text.
- **"It passes" ≠ "it's correct."** `flower/flow/flight` passed while `ba/ca/ba` was silently
  wrong. Test the case that *should* fail, not just the one you were given.

Related From-Zero lessons this surfaced:
[05b — break/continue/labels](../../from-zero/rust/05b-break-continue-and-labels/use-it.md) ·
[15a — opening an Option safely](../../from-zero/rust/15a-opening-options-safely/use-it.md) ·
lifetimes (`&'a`) and closures are taught next in the [roadmap](../../from-zero/rust/README.md).
