# Celsius → Fahrenheit

| | |
|---|---|
| Date       | 2026-08-17 |
| Difficulty | Easy |
| Languages  | Rust |
| Pattern    | [Integer vs float arithmetic](../../languages/rust.md#int-division) (+ display formatting) |
| Time/Space | O(1) / O(1) |
| Source     | [W3Schools Practice — VARIABLES01](https://www.w3schools.com/practice/practice.php?problem=VARIABLES01&lang=rust) |

## The Problem
Read a Celsius temperature and convert it to Fahrenheit with `fahrenheit = celsius * 9/5 + 32`,
then print `[celsius] Celsius = [fahrenheit] Fahrenheit`.

What matters:
- The Fahrenheit value must **show a decimal** — `77.0`, not `77`.
- The conversion factor `9/5` is `1.8` — a *fraction*, which turns out to be the whole trap.

Tiny example:
```
25  ->  25 Celsius = 77.0 Fahrenheit
```

## Understand It

### In plain words
Converting a temperature is just a recipe: multiply by `1.8`, then add `32`. The catch is a
tool problem, not a math problem — if your calculator quietly rounds `1.8` down to `1` before
multiplying, every answer comes out wrong, and nothing tells you. That's exactly what a whole
number (integer) type does in Rust.

### The slow, obvious way
My first attempt typed everything as `i32` — plain whole numbers:

```rust
let celsius_temperature: i32 = input_line.trim().parse().unwrap();
let fahrenheit_temperature: i32 = (celsius_temperature * (9 / 5)) + 32;
```

For `100` it printed `100 Celsius = 132 Fahrenheit`. The real answer is `212`, so something was
eating the math. **Why:** `9` and `5` are both integers, so `9 / 5` is
[integer division](../../languages/rust.md#int-division) — Rust throws the fraction away rather
than rounding, so `1.8` becomes `1`. The formula silently collapsed to:

```
celsius_temperature * 1 + 32   →   celsius_temperature + 32          100 + 32 = 132
```

The conversion never ran. Nothing warned me, because as far as Rust was concerned this was a
perfectly valid *integer* calculation — just not the one I meant. The **types decided the
result before the arithmetic could.**

### The trick
Two separate fixes, for two separate problems.

**Value — use floats so the fraction survives.** Make the number and the literals `f32`:

```rust
let celsius_temperature: f32 = input_line.trim().parse().unwrap();
let fahrenheit_temperature = (celsius_temperature * (9.0 / 5.0)) + 32.0;
```

Now `9.0 / 5.0` keeps its `1.8`. Writing the literals as `9.0`/`5.0` matters: because `celsius_temperature`
is `f32`, they're inferred as `f32` too, so there's no type mismatch.

**Display — ask for the decimal to be printed.** Getting the math right isn't enough: plain `{}`
prints an `f32` of `77.0` as just `77`. The value is *already* `77.0` in memory; it just isn't
*shown* with the decimal. That's a [formatting](../../languages/rust.md#format-spec) problem,
separate from the type — solved with `{:.1}` ("one digit after the point"):

```rust
println!(
    "{} Celsius = {:.1} Fahrenheit",
    celsius_temperature, fahrenheit_temperature
);
```

### Watch it run

| input | as `i32` (buggy) | as `f32` + `{:.1}` (fixed) |
|---|---|---|
| 100 | `9/5` = 1 → `100 + 32` = **132** ❌ | `100 * 1.8 + 32` = 212.0 → **212.0** ✅ |
| 25  | `25 + 32` = **57** ❌ | `25 * 1.8 + 32` = 77.0 → **77.0** ✅ |

### The answer
Floats keep the `1.8`, and `{:.1}` shows the decimal — so `25` prints `25 Celsius = 77.0
Fahrenheit`, correct in both value and display.

## The Code

### Rust
```rust
use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let celsius_temperature: f32 = input_line.trim().parse().unwrap();

    let fahrenheit_temperature = (celsius_temperature * (9.0 / 5.0)) + 32.0;

    println!(
        "{} Celsius = {:.1} Fahrenheit",
        celsius_temperature, fahrenheit_temperature
    );
}
```

**Time:** O(1) — a fixed handful of operations, whatever the input. **Space:** O(1) — a couple
of numbers. **Syntax notes:** [solution.rs.md](solution.rs.md).

## Remember This
- **`/` between two integers truncates.** `9 / 5` is `1`, not `1.8` — the `.8` is discarded, not
  rounded. If you need the decimal, at least one side must be a float. Same trap lives in C,
  Java, and Go.
- **Type and display are different things.** `{:.1}` controls how many decimals *print*,
  independent of the stored value — `77.0` can show as `77` or `77.0` depending only on the
  format specifier. Formatting gets its own lesson:
  [From-Zero Interlude 01a](../../from-zero/rust/01a-printing-and-formatting/use-it.md).

Reusable write-ups: [integer division truncates](../../languages/rust.md#int-division) ·
[`{:.1}` format specifiers](../../languages/rust.md#format-spec).
