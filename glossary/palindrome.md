# Palindrome

**In one line:** something that reads the same forwards and backwards.

## Plain explanation
Hold a word or a number up to a mirror. If the reflection is identical to the original, it's a
palindrome. The word *"level"* spelled backwards is still *"level"*. The number `121` reversed
is still `121`. But `123` reversed is `321` — different, so `123` is not a palindrome.

The test is always the same shape: pair up the outermost items and work inward — first with
last, second with second-to-last, and so on. If every pair matches, it's a palindrome; the
first mismatch proves it isn't. A single item in the exact middle (odd length) has no partner,
so it never has to match anything.

## Why you care
"Is this a palindrome?" is a classic warm-up because it shows up in two flavours that teach
different skills:
- **Text palindromes** (`"racecar"`) — practice walking a sequence from both ends, often with a
  pointer at each end moving toward the middle.
- **Number palindromes** (`12321`) — practice taking a number apart with arithmetic (`% 10` for
  the last digit, `/ 10` to drop it) instead of converting it to text. The neat optimization is
  that you only need to reverse *half* the digits and meet in the middle.

## Quick examples
- `"level"`, `"noon"`, `12321`, `7` → palindromes (a single character/digit always is).
- `"hello"`, `123`, `10` → not palindromes.
- Edge case for numbers: any number ending in `0` except `0` itself can't be a palindrome — its
  first digit would have to be `0`, and numbers don't keep leading zeros.

## Related
- [Big-O Notation](big-o-notation.md)

## Shows up in
- [Palindrome Number](../challenges/palindrome-number/README.md)
