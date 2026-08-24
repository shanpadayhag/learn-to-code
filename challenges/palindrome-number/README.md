# Palindrome Number

| | |
|---|---|
| Date       | 2026-08-19 |
| Difficulty | Easy |
| Languages  | Rust |
| Pattern    | [Palindrome](../../glossary/palindrome.md) (reverse half the digits, with math) |
| Time/Space | O(log₁₀ n) / O(1) |
| Source     | [LeetCode 9 — Palindrome Number](https://leetcode.com/problems/palindrome-number/) (practice variant: read a number, print `Yes` / `No`) |

## The Problem
Read one non-negative whole number. Print `Yes` if it reads the same forwards and backwards,
`No` otherwise.

What matters:
- The catch: **solve it with arithmetic, not by turning the number into text.** That rule is the
  whole point — it forces you to handle the digits *as numbers*.
- Edge cases hide here: `0` is a palindrome; any other number ending in `0` (like `10`) can't be.

Tiny example:
```
121 -> Yes        123 -> No        0 -> Yes
```

## Understand It

### In plain words
A [palindrome](../../glossary/palindrome.md) reads the same both directions — like the word
*"level"* or the number *121*. Hold `121` up to a mirror and it's still `121`; hold up `123`
and you see `321`, a different number. So the question is just: *does this number equal its own
reflection?* The twist is we're told to answer it **without** writing the number out as text
and flipping the string — only with arithmetic on the digits.

### The slow, obvious way
The tempting shortcut is to turn the number into text, reverse the text, and compare:

```rust
let number = input.trim().to_string();
let mut reversed = String::new();
for character in number.chars().rev() {   // walk the characters backwards
    reversed.push(character);
}
number == reversed                         // "121" == "121" -> Yes
```

It **works** — every test passes. But it's the wrong answer for two reasons, one of them fatal
here:

1. **It breaks the rule.** The task said *no string conversion* — the whole lesson is digit
   arithmetic, and this sidesteps it.
2. **It allocates.** `to_string()` and `reversed` both build growable text buffers on the heap —
   `O(n)` extra memory to check a shape we could read straight off the digits.

So it passes, yet misses the point. The real challenge is to never leave numbers at all.

### The trick
You can take a number apart with two operations, no text involved:
- `number % 10` — the [remainder](../../languages/rust.md#remainder) after dividing by 10 — is
  the **last digit**. `1234 % 10` is `4`.
- `number / 10` — [integer division](../../languages/rust.md#int-division) — **drops** the last
  digit. `1234 / 10` is `123`.

With those you can rebuild a number reversed, digit by digit. But the sharp idea is: **you don't
need the whole reverse — only half.** A number is a palindrome when its front half mirrors its
back half, so reverse *only the back half* and stop when the shrinking front meets the growing
reversed-back in the middle. Half the work — and the half-size reversed value can't overflow,
because it's never bigger than half the original.

```rust
while remaining_number > reversed_half {
    reversed_half = reversed_half * 10 + remaining_number % 10;  // push last digit onto the reversed half
    remaining_number /= 10;                                      // drop it from the front
}
```

Each turn: peel the last digit off `remaining_number` and stick it onto `reversed_half`
(multiply by 10 to make room, then add). `remaining_number` shrinks; `reversed_half` grows. When
`reversed_half` catches up, we've crossed the middle. This works — instead of just looking
right — because `% 10` and `/ 10` are *exact* on integers: no rounding, no lost digits, so the
reversed half is a faithful mirror of the back half.

### Watch it run
`1221` (even length):

| step | `remaining_number` | `reversed_half` | `remaining > reversed`? |
|---|---|---|---|
| start | 1221 | 0  | 1221 > 0 → go |
| 1 | 122 | 1  | 122 > 1 → go |
| 2 | 12  | 12 | 12 > 12 → **stop** |

`remaining_number == reversed_half` (both `12`) → palindrome. And `12321` (odd length), where a
middle digit is left over:

| step | `remaining_number` | `reversed_half` | `remaining > reversed`? |
|---|---|---|---|
| start | 12321 | 0 | go |
| 1 | 1232 | 1 | go |
| 2 | 123 | 12 | go |
| 3 | 12 | 123 | 12 > 123? no → **stop** |

They never land equal, because the middle `3` sits in `reversed_half`. Drop it with
`reversed_half / 10` (`123 / 10 = 12`) and compare: `12 == 12` → palindrome.

### The answer
Compare the two halves, allowing for that leftover middle digit:

```rust
remaining_number == reversed_half || remaining_number == reversed_half / 10
//        even-length case         ||        odd-length case (ignore the middle)
```

It's correct because a palindrome is *exactly* the case where the back half, reversed, equals
the front half — which is what these two comparisons test (the second one shrugging off a lone
middle digit).

## The Code

### Rust
```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u64 = input.trim().parse().unwrap();

    println!("{}", if is_palindrome(number) { "Yes" } else { "No" });
}

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

**Time:** O(log₁₀ n) — the number of digits, and we walk only *half* of them. No class is
faster: you must look at the digits to know if they mirror. **Space:** O(1) — two `u64`s, no
allocation, whatever the size of the number.
**Syntax notes:** [solution.rs.md](solution.rs.md).

## Remember This
- **Digits are just arithmetic.** `% 10` reads the last digit, `/ 10` removes it — the whole
  toolkit for taking a number apart, with no string and no allocation.
- **Reverse only half.** To test a mirror you don't need the whole reflection; build the back
  half and stop when it meets the front. Half the work, and the half-size value can't overflow.
- **Mind the trailing-zero edge case.** Any number ending in `0` except `0` itself can't be a
  palindrome (its first digit would have to be `0`, but numbers don't keep leading zeros), so
  `if number % 10 == 0 && number != 0 { return false; }` rules them out up front.
- **"It passes" ≠ "it's the answer."** The string version passed every test but broke the rule
  *and* allocated. Read what the problem is actually teaching — here, the arithmetic was the
  point, and the tests just didn't check for it.

Reusable write-ups: [Palindrome](../../glossary/palindrome.md) ·
[`%` remainder](../../languages/rust.md#remainder) ·
[`/` truncates](../../languages/rust.md#int-division) · [`.rev()`](../../languages/rust.md#rev).
