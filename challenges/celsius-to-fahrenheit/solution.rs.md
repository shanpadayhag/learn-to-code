# Celsius → Fahrenheit — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). Everything here is in the
[Rust handbook](../../languages/rust.md) or the linked From-Zero lessons.

## The pieces
- `io::stdin().read_line(&mut input)` — read one line of input into the `String` `input`. It
  needs `&mut input` because it *writes into* the string ([`&mut`](../../languages/rust.md#mut-ref)).
- `input.trim().parse().unwrap()` — [`.trim()`](../../languages/rust.md#trim) drops the trailing
  newline; `.parse()` turns the text into a number; `.unwrap()` takes the value out (crashing on
  bad input — fine for a practice input, see [`.unwrap()`](../../languages/rust.md#unwrap)).
- `let celsius: f32 = ...` — the `: f32` annotation is doing real work: it's what tells `.parse()`
  to produce a **float**, and it's why the literals below are inferred as floats too. Swap it for
  `i32` and the whole calculation silently truncates — the bug the [README](README.md) dissects.
- `9.0 / 5.0` — float literals. The `.0` is the difference between `1.8` (float division) and `1`
  ([integer division](../../languages/rust.md#int-division)). Because `celsius` is `f32`, these
  are inferred `f32`, so the arithmetic stays in floats with no mismatch.
- `{:.1}` — a [format specifier](../../languages/rust.md#format-spec): print exactly one digit
  after the decimal point, so `77.0` shows its decimal instead of printing as `77`. This changes
  the *display*, not the value. Taught from zero in
  [Interlude 01a — Printing and formatting](../../from-zero/rust/01a-printing-and-formatting/use-it.md).
