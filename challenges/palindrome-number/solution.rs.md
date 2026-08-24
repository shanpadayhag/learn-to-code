# Palindrome Number — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). Everything here is already in the
[Rust handbook](../../languages/rust.md) — this is a walk through how the pieces fit in *this*
solution.

## Already in the handbook
- [`while` loops](../../languages/rust.md#while) — we loop on a *condition*
  (`remaining_number > reversed_half`), not a fixed count, so `while` fits where a `for` range
  wouldn't.
- [`%` — remainder](../../languages/rust.md#remainder) — `number % 10` is the last digit.
- [integer division `/`](../../languages/rust.md#int-division) — `remaining_number /= 10` drops
  the last digit; `reversed_half / 10` drops the leftover middle digit. Truncation is a *feature*
  here, not the bug it was in [Celsius → Fahrenheit](../celsius-to-fahrenheit/README.md).
- [`.unwrap()`](../../languages/rust.md#unwrap) — on `parse()`; fine for a challenge with clean
  input, a crash-on-bad-input bet in general (see [Interlude 15a](../../from-zero/rust/15a-opening-options-safely/use-it.md)).

## Line by line
- `let number: u64 = input.trim().parse().unwrap()` — `.trim()` drops the trailing newline,
  `.parse()` reads the text as a number, and the `: u64` annotation is what tells `.parse()`
  *which* number type to produce. `u64` (unsigned) suits a non-negative input.
- `if is_palindrome(number) { "Yes" } else { "No" }` — an [`if` used as an
  expression](../../from-zero/rust/05-expressions-statements-and-return/use-it.md): the whole
  thing *evaluates to* one of the two string literals, which `println!` then prints. No
  intermediate variable needed.
- `let mut remaining_number = number;` / `let mut reversed_half = 0;` — both are `mut` because
  the loop reassigns them every turn; a plain `let` would reject the reassignment.
- `reversed_half = reversed_half * 10 + remaining_number % 10;` — "make room, then drop in the
  last digit": `* 10` shifts the existing reversed digits left one place, `+ … % 10` puts the new
  digit in the ones place.
- The final `a == b || a == b / 10` — one expression covering both even-length (`a == b`) and
  odd-length (`a == b / 10`, ignoring the middle digit) palindromes; it's the function's return
  value because it's the last expression with no semicolon.
