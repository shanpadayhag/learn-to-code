# 1. Two Sum — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). New features are explained in
the [Rust handbook](../../languages/rust.md); already-known ones are linked there.

## New here
This is the first Rust solution in the repo, so every construct below is new and
now lives in the handbook. Each line is a one-sentence "what and why"; the full
treatment is one click away in the handbook.
- `use std::collections::HashMap;` — [`use` declaration](../../languages/rust.md#use):
  pulls `HashMap` into scope so we write the short name, not the full path, all loop.
- `impl Solution { ... }` — [`impl` block](../../languages/rust.md#impl): the wrapper
  LeetCode's judge expects; it's where a method attaches to a type.
- `pub fn two_sum(...) -> Vec<i32>` — [`pub fn`](../../languages/rust.md#pub-fn):
  a public function; parameter types come after the name, the return type after `->`.
- `Vec<i32>` — [vector type](../../languages/rust.md#vec-type): a growable list of
  integers, used here for both the input and the answer.
- `let mut seen_number_index` — [`let mut`](../../languages/rust.md#let-mut): a
  *changeable* binding — needed because we insert into the map on every pass; a plain
  immutable `let` would reject the next `.insert`.
- `HashMap<i32, i32>` / `HashMap::new()` —
  [`HashMap`](../../languages/rust.md#hashmap): Rust's spelling of the
  [hash map](../../glossary/hash-map.md) concept, the structure the whole trick rests on.
- `numbers.iter().enumerate()` —
  [`for` + `.iter()` + `.enumerate()`](../../languages/rust.md#for-iter-enumerate):
  one loop that hands us each value *and* its position.
- `&current_number_value` and `Some(&existing_number_index)` —
  [`&` in patterns](../../languages/rust.md#ref-pattern): peels the reference so we
  hold a plain `i32` instead of a pointer to one.
- `if let Some(...) = ...get(&needed_number_value)` —
  [`if let` with `Option`](../../languages/rust.md#if-let): runs the block only when
  the lookup actually found something — Rust's no-`null` answer to "was it there?".
- `current_number_index as i32` — [`as` cast](../../languages/rust.md#as-cast): converts the
  `usize` position into the `i32` the answer vector holds, because Rust won't mix
  number types silently.
- `vec![existing_number_index, current_number_index as i32]` / `vec![]` —
  [`vec![]` macro](../../languages/rust.md#vec-macro): builds the result vector.

## Line by line
The first Rust solution in the repo, so this walks the whole thing top to bottom. The
goal is that each line *clicks*, not just that you can read past it.

**The signature.**
```rust
pub fn two_sum(input_numbers: Vec<i32>, target_sum: i32) -> Vec<i32> {
```
Read it left to right: a public function `two_sum` taking a list of integers
(`input_numbers: Vec<i32>`) and one integer (`target_sum: i32`), handing back a list
of integers (`-> Vec<i32>`). The function *owns* `input_numbers` — it's passed by
value, not borrowed — which is fine here because we only read it. We return a `Vec` rather than
a tidy pair because that's the exact shape LeetCode's judge checks for.

**The map.**
```rust
let mut seen_number_index: HashMap<i32, i32> = HashMap::new();
```
The type spelled out is `HashMap<i32, i32>`: the **key** is a value we've already
walked past, and the data stored under it is the **index** where we saw that value.
So a key of `7` pointing to `0` reads as "the number 7 lives at position 0." It's
`mut` because we add an entry every loop — delete the `mut` and the first `.insert`
below stops compiling, since Rust forbids changing an immutable binding. The
`: HashMap<i32, i32>` annotation is there because `HashMap::new()` on its own doesn't
reveal what it holds; this states the key and value types up front.

**The loop header.**
```rust
for (current_number_index, &current_number_value) in input_numbers.iter().enumerate() {
```
Trace what the chain produces, one step at a time:
- `input_numbers` is a `Vec<i32>`.
- `.iter()` walks it *by borrow*, so each item is a `&i32` — a pointer to the number,
  not the number itself. (Borrowing = looking without taking ownership, so
  `input_numbers` survives the loop intact.)
- `.enumerate()` pairs each item with its position, yielding `(usize, &i32)` — a
  `usize` is the integer type Rust uses for positions and lengths.

So every turn produces a `(usize, &i32)`, and the pattern `(current_number_index, &current_number_value)`
unpacks it. `current_number_index` catches the `usize`. The `&` on `&current_number_value` *peels
the reference*: because the incoming value is a `&i32`, matching it against
`&current_number_value` binds `current_number_value` to the plain `i32` underneath. Skip that `&`
and `current_number_value` would stay a `&i32`, forcing the very next line to read
`target_sum - *current_number_value` with a manual dereference. Peeling once here keeps the rest
of the body in plain numbers.

**The partner.**
```rust
    let needed_number_value = target_sum - current_number_value;
```
The single number that would complete the pair. Plain `let`, no `mut`: it's computed
fresh each loop and never reassigned, so it stays immutable by default.

**The lookup.**
```rust
    if let Some(&existing_number_index) = seen_number_index.get(&needed_number_value) {
        return vec![existing_number_index, current_number_index as i32];
    }
```
`.get(&needed_number_value)` asks the map "is this value one of your keys?" It takes the key
*by reference* and answers with an `Option<&i32>`. Rust has no `null`, so a
maybe-missing result comes wrapped: `Some(x)` means found (with `x` inside) and `None`
means not found. The `if let Some(&existing_number_index) = ...` does two jobs in one line —
it tests for the "found" case *and*, on a hit, lifts the inner value out and binds it.
That inner value is itself a `&i32`, so the `&existing_number_index` peels it down to a plain
`i32`, the same move as in the loop header. On a miss the block is simply skipped.
Inside, `current_number_index as i32` converts the `usize` position to the `i32` the answer
holds (Rust won't do that conversion silently), and `vec![...]` packs both positions
into the result we return immediately — the first match is the answer.

**The store.**
```rust
    seen_number_index.insert(current_number_value, current_number_index as i32);
```
Reached only when the partner *wasn't* already in the map. We file the current value
→ its index so a *later* number can find it. This runs after the lookup on purpose:
checking before storing is what stops a number from matching itself.

**The fallback.**
```rust
vec![]
```
The last expression in the body, with no trailing semicolon, *is* the function's
return value — so this hands back an empty `Vec<i32>`. The problem guarantees an
answer exists, so we never actually arrive here, but every path through a Rust
function must produce the declared return type, and the compiler rejects the function
without it.

## Running it
`solution.rs` is a whole program, not the half-file LeetCode shows you. Run it with:

```
rustc solution.rs && ./solution
```

The algorithm above is untouched — it's still exactly what you paste into LeetCode.
Around it sit the two pieces their editor supplies off-screen, plus the harness that
proves the thing works:

- `struct Solution;` — [unit struct](../../languages/rust.md#unit-struct): a type with
  no fields, there only so the method can be called as `Solution::two_sum(...)`.
  LeetCode declares it invisibly; without it this file wouldn't compile.
- `fn main() { ... }` — [`fn main`](../../languages/rust.md#main): where the program
  starts. LeetCode's judge owns this half, which is why a copied solution alone errors
  with `` `main` function not found ``.
- `assert_eq!(matching_number_indexes, expected_indexes)` —
  [`assert_eq!`](../../languages/rust.md#assert-eq):
  states the expected answer as code. A wrong result stops the run at the failing case
  instead of scrolling past.
- `println!("two_sum({:?}, {}) = {:?}", ...)` —
  [`println!`](../../languages/rust.md#println): shows each case's input and answer, so
  a passing run reads like a worked example. `{:?}` is used for the vectors because a
  `Vec` prints only in the debug form.
- `check(...)` — one named function per case keeps `main` a readable list of examples:
  the LeetCode examples first, then negatives and a repeated `0` for the edge cases.
- `input_numbers.clone()` — [`.clone()`](../../languages/rust.md#to-owned-clone):
  `two_sum` *consumes* its `Vec`, so the harness hands it a copy and keeps the
  original to print.
