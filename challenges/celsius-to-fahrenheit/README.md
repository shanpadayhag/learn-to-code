# Celsius → Fahrenheit

| | |
|---|---|
| Date       | 2026-08-17 |
| Language   | Rust |
| Source     | [W3Schools Practice — VARIABLES01](https://www.w3schools.com/practice/practice.php?problem=VARIABLES01&lang=rust) |
| Lessons    | [Integer division truncates](../../languages/rust.md#int-division) · [`{:.1}` format specifiers](../../languages/rust.md#format-spec) |

## The Task
Read a Celsius temperature from input, convert it to Fahrenheit with

```
fahrenheit = celsius * 9/5 + 32
```

and print `[celsius] Celsius = [fahrenheit] Fahrenheit`. The Fahrenheit value has
to show a decimal — `77.0`, not `77`.

## The Bug I Hit

My first attempt typed everything as `i32` ([initial.rs](initial.rs)):

```rust
let celsius: i32 = input.trim().parse().unwrap();
let fahrenheit: i32 = (celsius * (9 / 5)) + 32;
```

For `100` it printed `100 Celsius = 132 Fahrenheit`. The real answer is `212`, so
something was eating the math.

### Why

Both `9` and `5` are integers, so `9 / 5` is **integer division**. Rust doesn't
round — it throws the fractional part away. `1.8` becomes `1`. So my formula
silently turned into:

```
celsius * 1 + 32   →   celsius + 32
```

`100 + 32 = 132`. The conversion never actually ran; the *types* decided the
result before the arithmetic could. Nothing warned me, because as far as Rust was
concerned this was a perfectly valid integer calculation — just not the one I
meant.

## The Fix

Make the numbers floats — both the variable and the literals ([solution.rs](solution.rs)):

```rust
let celsius: f32 = input.trim().parse().unwrap();
let fahrenheit = (celsius * (9.0 / 5.0)) + 32.0;
```

Now `9.0 / 5.0` keeps its `1.8`, and `100` prints `212`. Writing the literals as
`9.0` / `5.0` matters: because `celsius` is `f32`, those literals are inferred as
`f32` too, so there's no type-mismatch error.

## The Second Thing: Making It Print `77.0`

Fixing the type got the *math* right, but the display was still off. The task wants
`77.0`, and plain `{}` prints an `f32` of `77.0` as just `77`. That's a **formatting**
problem, separate from the type — the value is already `77.0` in memory; it just
isn't *shown* with the decimal. The fix was `{:.1}`:

```rust
println!("{} Celsius = {:.1} Fahrenheit", celsius, fahrenheit);
// 25 -> "25 Celsius = 77.0 Fahrenheit"
```

Formatting is its own topic, separate from the type fix, so it gets a full lesson:
**From-Zero → [Interlude 01a — Printing and formatting](../../from-zero/rust/01a-printing-and-formatting/use-it.md)**.
Quick reference in the handbook: [`{:.1}` format specifiers](../../languages/rust.md#format-spec).

## Takeaway

Two separate lessons hid in one tiny problem:

1. **Value:** in Rust, `/` between two integers **truncates** — `9 / 5` is `1`, not
   `1.8`. If you need the decimal, at least one side has to be a float. Same trap
   lives in C, Java, Go, and most typed languages. First time `f32` actually clicked.
2. **Display:** `{:.1}` controls how many decimals *print*, independent of the value —
   `77.0` can print as `77` or `77.0` depending only on the specifier.

Reusable write-ups: [integer division truncates](../../languages/rust.md#int-division)
· [`{:.1}` format specifiers](../../languages/rust.md#format-spec).
