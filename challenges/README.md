# Practice Challenges

Small coding challenges I do on my own, one a day, to test what I've actually
learned. Unlike the [From-Zero](../from-zero/README.md) course's built-in
**exercises** (which drill a single concept in isolation), these come from outside
sources and mix ideas the way real problems do — so the interesting part is usually
the *bug I hit*, not the solution.

Each challenge gets the **same full write-up as a [LeetCode problem](../problems/)**: a
paraphrased task, a five-beat "Understand It" walkthrough (*in plain words → the slow,
obvious way → the trick → watch it run → the answer*), the code with its complexity, and a
`solution.rs.md` explaining the syntax line by line. The real **bug I hit** lives inside the
"slow, obvious way" beat — that's the part these are for. Like the problems folder, a
challenge is `README.md` + `solution.rs` + `solution.rs.md` (no separate first-attempt file).
Reusable ideas get promoted to the [glossary](../glossary/README.md) and the
[Rust handbook](../languages/rust.md) and linked back here.

| Date | Challenge | Language | Source | Lesson |
|------|-----------|----------|--------|--------|
| 2026-08-17 | [Celsius → Fahrenheit](celsius-to-fahrenheit/README.md) | Rust | [W3Schools VARIABLES01](https://www.w3schools.com/practice/practice.php?problem=VARIABLES01&lang=rust) | [Integer division truncates](../languages/rust.md#int-division) · [`{:.1}` format specifiers](../languages/rust.md#format-spec) |
| 2026-08-19 | [Palindrome Number](palindrome-number/README.md) | Rust | [LeetCode 9](https://leetcode.com/problems/palindrome-number/) | [`%` remainder](../languages/rust.md#remainder) · [`.rev()`](../languages/rust.md#rev) |
| 2026-08-24 | [Longest Common Prefix](longest-common-prefix/README.md) | Rust | [LeetCode 14](https://leetcode.com/problems/longest-common-prefix/) | [`break` + labeled loops](../languages/rust.md#loop-control) · [`.unwrap()` risk](../languages/rust.md#unwrap) · [`.zip()`/`.take_while()`/`.count()`](../languages/rust.md#iterator-adapters) |
