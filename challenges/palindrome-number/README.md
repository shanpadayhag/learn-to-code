# Palindrome Number

| | |
|---|---|
| Date       | 2026-08-19 |
| Language   | Rust |
| Source     | [LeetCode 9 — Palindrome Number](https://leetcode.com/problems/palindrome-number/) (practice variant: read a number from input, print `Yes` / `No`) |
| Lessons    | [`%` — the remainder operator](../../languages/rust.md#remainder) · [`.rev()` — reverse an iterator](../../languages/rust.md#rev) |

## The Task
Read one non-negative whole number from input. Print `Yes` if it reads the same
forwards and backwards, `No` if it doesn't. `121` → `Yes`; `123` → `No`; `0` → `Yes`.

The catch: **solve it with arithmetic, not by turning the number into text.** That
rule is the whole point of the challenge — it forces you to work with the digits as
numbers.

## My First Attempt — the obvious way

Turn the number into a string, reverse the string, compare ([initial.rs](initial.rs)):

```rust
let number = input.trim().to_string();

let mut reversed = String::new();
for character in number.chars().rev() {   // .rev() walks the characters backwards
    reversed.push(character);
}

number == reversed        // "121" == "121"  ->  Yes
```

This **works** — every test passes. The new piece for me was
[`.rev()`](../../languages/rust.md#rev): it takes the sequence of characters and
hands them back in reverse order, so I just push them into a fresh string.

### Why it's not the answer

Two problems, one of them fatal for this challenge:

1. **It breaks the rule.** The task said *no string conversion* — the lesson is
   digit arithmetic, and this sidesteps it entirely.
2. **It allocates.** `to_string()` and the `reversed` string both live on the
   [heap](../../from-zero/rust/README.md). For a number, that's a whole growable
   text buffer built just to check a shape we could read straight off the digits.
   It's `O(n)` extra memory for something that needs none.

So it passes, but it's the wrong tool. The real challenge is to never leave
numbers at all.

## The Trick — reverse the number with math, and only do half

You can peel digits off a number with two operations, no text involved:

- `number % 10` — the **remainder** after dividing by 10 — is the **last digit**.
  `1234 % 10` is `4`. ([what `%` does](../../languages/rust.md#remainder))
- `number / 10` — integer division — **drops** that last digit. `1234 / 10` is
  `123`, because [`/` throws the fraction away](../../languages/rust.md#int-division).

With those two, you can build a number's reverse digit by digit. But there's a
sharper idea: **you don't have to reverse the whole thing.** A number is a
palindrome when its front half mirrors its back half — so reverse *only the back
half*, and stop when the shrinking front and the growing reversed-back meet in the
middle. That's half the work, and the reversed half can never overflow, because
it's only ever half the size of the original.

The loop:

```rust
while remaining_number > reversed_half {
    reversed_half = reversed_half * 10 + remaining_number % 10;  // push last digit onto reversed
    remaining_number /= 10;                                      // drop it from the front
}
```

Each turn: take the last digit of `remaining_number` and stick it onto the end of
`reversed_half` (multiply by 10 to make room, then add). Meanwhile `remaining_number`
gets shorter. When `reversed_half` catches up to `remaining_number`, we've crossed
the middle.

### Watch it run — `1221`

`remaining_number` shrinks from the front; `reversed_half` grows the back, reversed:

| step | `remaining_number` | `reversed_half` | still `remaining > reversed`? |
|---|---|---|---|
| start | 1221 | 0    | 1221 > 0 → go |
| 1     | 122  | 1    | 122 > 1 → go |
| 2     | 12   | 12   | 12 > 12 → **stop** |

Now `remaining_number == reversed_half` (both `12`) → palindrome. **Yes.**

And an odd-length one, `12321`, where the middle digit is left over:

| step | `remaining_number` | `reversed_half` | still `remaining > reversed`? |
|---|---|---|---|
| start | 12321 | 0   | go |
| 1     | 1232  | 1   | go |
| 2     | 123   | 12  | go |
| 3     | 12    | 123 | 12 > 123? no → **stop** |

Here they never land equal, because the middle `3` sits in `reversed_half`. Drop it
with `reversed_half / 10` (`123 / 10 = 12`) and compare: `12 == 12` → **Yes**. That's
why the final check has two halves:

```rust
remaining_number == reversed_half || remaining_number == reversed_half / 10
//        even-length case         ||        odd-length case (ignore middle digit)
```

### The one edge case — trailing zeros

Any number ending in `0` (except `0` itself) can't be a palindrome: the first digit
would have to be `0` too, but numbers don't keep leading zeros. `10` reversed is
`01` = `1`. So we rule those out up front:

```rust
if number % 10 == 0 && number != 0 {
    return false;
}
```

Without this guard, `10` would sneak through: the loop stops immediately
(`10 > 0` is true once → `remaining=1, reversed=0`, then `1 > 0`... actually it
keeps going), and you'd get a wrong `Yes`. The guard makes the intent explicit and
correct.

## The Answer

[solution.rs](solution.rs) — no strings, no heap, only ever touches half the digits:

```rust
fn is_palindrome(number: u64) -> bool {
    if number % 10 == 0 && number != 0 {
        return false;
    }

    let mut remaining_number = number;
    let mut reversed_half = 0;
    while remaining_number > reversed_half {
        reversed_half = reversed_half * 10 + remaining_number % 10;
        remaining_number /= 10;
    }

    remaining_number == reversed_half || remaining_number == reversed_half / 10
}
```

**Time:** `O(log₁₀ n)` — the number of digits, and we only walk *half* of them.
There is no faster class: you must look at the digits to know if they mirror.
**Space:** `O(1)` — two `u64`s, no allocation, whatever the size of the number.

That's the full "fastest, then most memory-efficient" target: the string version was
already optimal *time*, but this matches it while dropping the `O(n)` heap string to
`O(1)`. You can't do better on either axis.

## Takeaway

- **Digits are just arithmetic.** `% 10` reads the last digit, `/ 10` removes it.
  Those two are the whole toolkit for taking a number apart — no string needed, and
  no allocation.
- **Reverse only half.** To test a mirror you don't need the whole reflection —
  build the back half and stop when it meets the front. Half the work, and the
  half-size reversed value can't overflow.
- **"It passes" ≠ "it's the answer."** My string version passed every test but
  broke the rule *and* allocated. The graded thing was the arithmetic; the tests
  just didn't check for it. Read what the problem is really teaching.

Reusable write-ups: [`%` remainder](../../languages/rust.md#remainder) ·
[`.rev()`](../../languages/rust.md#rev) · [`/` truncates](../../languages/rust.md#int-division).
