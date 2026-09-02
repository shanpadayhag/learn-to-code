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
`candidate_words[0]`'s letter here the same as `candidate_words[1]`'s? And
`candidate_words[1]` vs `candidate_words[2]`? Walk every
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
words is exactly a prefix of `candidate_words[0]` that every other word also starts with. So keep a
single number — "how many letters still agree" — start it at the full length of the first
word, and shrink it against each other word. There's no grid, so the nested-loop `break` trap
simply can't happen.

**Let one iterator do the letter-walk, reading each letter once.** Rust's iterators
([the tools that will get their own phase later](../../from-zero/rust/README.md)) chain
together to say exactly what we mean:

```rust
// the first word's letters, one forward pass
common_prefix_length = first_word.bytes()
    // pair each with this word's letters; ends at the shorter word
    .zip(current_word.bytes())
    // never look past the prefix we still believe in
    .take(common_prefix_length)
    // keep going while letters match; stop at the first that differs
    .take_while(|(first_word_byte, current_word_byte)| first_word_byte == current_word_byte)
    // how many matched = the new agreed length
    .count();
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
`common_prefix_length` starts at `6` (all of `flower`) and only ever shrinks:

| against | letters compared | matched run | `common_prefix_length` |
|---|---|---|---|
| start (`flower`) | — | — | 6 |
| `flow`   | `f=f, l=l, o=o, w=w`, then `flow` ends | 4 | 4 |
| `flight` | `f=f, l=l`, then `o≠i` → stop | 2 | 2 |

Final `common_prefix_length` is `2`, so the answer is the first two letters of
`flower`. On `ba / ca / ba`: against `ca`, `b≠c` immediately → `common_prefix_length`
becomes `0`, and we're done.

### The answer
Take that many letters off the front of the first word:
[`&first_word[..common_prefix_length]`](../../languages/rust.md#slice) → **`fl`**.
It's correct because `common_prefix_length`
is, by construction, the largest count that *every* word agreed on — shrunk down the moment any
word disagreed, never grown back.

## The Code

### Rust
```rust
use std::io::{self, BufRead};

fn main() {
    let standard_input = io::stdin();
    let mut input_lines = standard_input.lock().lines();

    let input_word_count: usize = input_lines.next().unwrap().unwrap().trim().parse().unwrap();
    let owned_input_words: Vec<String> = (0..input_word_count)
        .map(|_| input_lines.next().unwrap().unwrap())
        .collect();
    let candidate_words: Vec<&str> = owned_input_words.iter().map(|word| word.trim()).collect();

    println!("{}", longest_common_prefix(&candidate_words));
}

fn longest_common_prefix<'a>(candidate_words: &[&'a str]) -> &'a str {
    if candidate_words.is_empty() {
        return "";
    }
    let first_word = candidate_words[0];

    let mut common_prefix_length = first_word.len();
    for current_word in &candidate_words[1..] {
        common_prefix_length = first_word
            .bytes()
            .zip(current_word.bytes())
            .take(common_prefix_length)
            .take_while(|(first_word_byte, current_word_byte)| first_word_byte == current_word_byte)
            .count();
        if common_prefix_length == 0 {
            break;
        }
    }

    &first_word[..common_prefix_length]
}
```

**Time:** O(N · L) — `N` words, each scanned once across the `L` letters that still agree. This
is optimal: you can't confirm a shared prefix without reading its letters in every word.
**Space:** O(1) extra — one `common_prefix_length` counter; the answer is a slice
**borrowed out of the first word**, so nothing is allocated or copied.
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
  because the task guarantees lowercase letters (one byte per character), so
  `common_prefix_length` always
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
