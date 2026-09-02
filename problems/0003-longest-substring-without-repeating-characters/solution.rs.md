# 3. Longest Substring Without Repeating Characters — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). New features are explained in
the [Rust handbook](../../languages/rust.md); already-known ones are linked there.

## New here
- `input_text: String` — [`String` type](../../languages/rust.md#string): the function
  *owns* the text it's handed, rather than borrowing a view of it. We only read it, so
  the distinction never bites us here. (The LeetCode default name is `s`; we rename it
  to `input_text` to match the repo's descriptive-naming style.)
- `input_text.chars()` — [`.chars()`](../../languages/rust.md#chars): walks the string
  one *character* at a time (not one byte), which is what lets us count substring
  length in characters.
- `for (current_character_index, current_character) in input_text.chars().enumerate()`
  — same [`.enumerate()`](../../languages/rust.md#for-iter-enumerate) as Two Sum, but
  note the pattern is a plain `current_character` with **no `&`**: `.chars()` yields
  owned `char` values, so there's no reference to peel (contrast the
  `&current_number_value` we needed when `.iter()` gave us `&i32`).
- `current_substring_start_index` / `current_character_index` are `usize` —
  [`usize` and underflow](../../languages/rust.md#usize): the subtraction
  `current_character_index - current_substring_start_index + 1` is only safe because
  `current_substring_start_index` can never pass `current_character_index`; a `usize`
  going negative would panic.
- `longest_substring_length.max(...)` — [`.max()`](../../languages/rust.md#ord-max):
  keeps the running maximum window width without an explicit `if`.

## Already covered
- `use std::collections::HashMap;` — [`use`](../../languages/rust.md#use)
- `impl Solution { ... }` — [`impl` block](../../languages/rust.md#impl)
- `pub fn length_of_longest_substring(...) -> i32` — [`pub fn`](../../languages/rust.md#pub-fn)
- `HashMap<char, usize>` / `HashMap::new()` / `.get()` / `.insert()` —
  [`HashMap`](../../languages/rust.md#hashmap), Rust's spelling of the
  [hash map](../../glossary/hash-map.md) concept
- `let mut last_seen_character_index` / `let mut current_substring_start_index` /
  `let mut longest_substring_length` — [`let mut`](../../languages/rust.md#let-mut):
  all three are reassigned as we scan
- `if let Some(&existing_character_index) = ...` — [`if let` with `Option`](../../languages/rust.md#if-let),
  and the `&existing_character_index` is the same [`&`-in-patterns](../../languages/rust.md#ref-pattern)
  peel that lifts the `usize` out of the `&usize` that `.get()` returns
- `longest_substring_length as i32` — [`as` cast](../../languages/rust.md#as-cast):
  the running maximum is a `usize`; LeetCode wants an `i32` back

## Line by line
The interesting parts are the window bookkeeping; the rest mirrors Two Sum.

**The map.**
```rust
let mut last_seen_character_index: HashMap<char, usize> = HashMap::new();
```
The **key** is a character and the value is the **last index we saw it at**. So an
entry `'b' → 1` reads as "the most recent `b` was at position 1." It's `mut` because
we overwrite a character's position every time we meet it again.

**The two counters.**
```rust
let mut current_substring_start_index = 0;
let mut longest_substring_length = 0;
```
`current_substring_start_index` is the left edge of the current repeat-free window;
`longest_substring_length` is the widest window we've measured so far. Both are
`usize` (inferred from the arithmetic below), and both change as we scan, so both are
`mut`.

**The loop header.**
```rust
for (current_character_index, current_character) in input_text.chars().enumerate() {
```
`.chars()` hands us each character *by value* as a `char`, and `.enumerate()` pairs
it with its position, giving `(usize, char)` each turn. Because the character comes
by value, the pattern is a plain `current_character` — there's no `&` to peel, unlike
the `.iter()` loop in Two Sum where each item was a *reference*.

**Slide the left edge.**
```rust
    if let Some(&existing_character_index) =
        last_seen_character_index.get(&current_character)
    {
        if existing_character_index >= current_substring_start_index {
            current_substring_start_index = existing_character_index + 1;
        }
    }
```
`.get(&current_character)` asks the map "have I seen this character, and where?" and
returns an [`Option`](../../languages/rust.md#if-let) — `Some` on a hit, `None`
otherwise. The `&existing_character_index` peels the `&usize` down to a plain
`usize`. The **inner** `if` is the crux of the whole algorithm: we move the left edge
*only* when the previous sighting is at or after `current_substring_start_index` —
i.e. still inside the current window. If it's before it, that old copy was already
dropped and isn't a real repeat, so we leave the edge alone. Moving it there would
drag the edge *backward*, which on `usize` positions would also risk the underflow
panic described in the handbook.

**Record and measure.**
```rust
    last_seen_character_index.insert(current_character, current_character_index);

    let current_substring_length =
        current_character_index - current_substring_start_index + 1;
    longest_substring_length = longest_substring_length.max(current_substring_length);
```
First stamp this character's new latest position into the map (overwriting any older
one). Then `current_character_index - current_substring_start_index + 1` is the
current window's width — the count of characters from the left edge through here —
and `.max` keeps it only if it beats the best so far. This runs *after* the edge may
have moved, so the width is always measured on a clean, repeat-free window.

**The return.**
```rust
longest_substring_length as i32
```
`longest_substring_length` is a `usize`; the judge wants an `i32`, so we
[cast](../../languages/rust.md#as-cast) it. As the last expression with no semicolon,
it *is* the return value.

## Running it
```
rustc solution.rs && ./solution
```

The `impl Solution` block is exactly what goes into LeetCode; the rest of the file is
the half their editor hides plus the harness:

- `struct Solution;` — [unit struct](../../languages/rust.md#unit-struct), and
  `fn main` — [entry point](../../languages/rust.md#main). Same two pieces as
  [Two Sum](../0001-two-sum/solution.rs.md).
- `check(input_text, expected_length)` takes a `&str` and calls `.to_string()` on it,
  because the signature LeetCode fixes wants an owned
  [`String`](../../languages/rust.md#string) while a literal in the harness is a
  borrowed `&str`.
- The cases cover the three LeetCode examples plus the ones that break naive
  solutions: `""` (empty), `" "` (a single space is still a character), `"dvdf"` (the
  repeat is *behind* the window start, so the left edge must not slide backwards), an
  all-distinct string where the answer is the whole length, and `"aabbaa"` (adjacent
  pairs, so the left edge jumps on nearly every step and the answer stays a stubborn
  `2` — the window never gets room to grow).
- `println!("... {:?} ...", input_text)` — [`println!`](../../languages/rust.md#println)
  with `{:?}` on the text on purpose: the debug form quotes the string, so the empty
  and single-space cases are visible in the output instead of vanishing.
