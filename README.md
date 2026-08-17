# Learn to Code

A personal study-and-reference repository for learning to program — worked LeetCode
solutions, reusable interview patterns, **from-scratch language courses**, and a
shared [glossary](glossary/README.md) of concepts. Everything is written
beginner-first, assuming **zero** programming background, and cross-linked so one
idea explains the next.

The goal is twofold: **learn** each idea deeply enough to re-derive it, and
**document** it well enough to grab at a glance later.

## Courses

From-scratch, learn-by-doing courses that build a language up one idea at a time.

- [From-Zero](from-zero/README.md) — learn a language by *seeing what happens in
  memory*, one concept per lesson, with run-it exercises. First track:
  [Rust](from-zero/rust/README.md).

## Problems

| # | Title | Difficulty | Languages | Pattern |
|---|-------|-----------|-----------|---------|
| 1 | [Two Sum](problems/0001-two-sum/README.md) | Easy | Rust | [Hash Map](glossary/hash-map.md) |
| 2 | [Add Two Numbers](problems/0002-add-two-numbers/README.md) | Medium | Rust | [Linked List](glossary/linked-list.md) |
| 3 | [Longest Substring Without Repeating Characters](problems/0003-longest-substring-without-repeating-characters/README.md) | Medium | Rust | [Sliding Window](glossary/sliding-window.md) |

## Challenges

Small, self-driven practice problems from outside sources (W3Schools and friends),
one a day. Written up **bug-first** — the mistake I hit is the lesson — with reusable
takeaways promoted into the syntax handbooks. See the [challenges index](challenges/README.md).

| Date | Challenge | Language | Lesson |
|------|-----------|----------|--------|
| 2026-08-17 | [Celsius → Fahrenheit](challenges/celsius-to-fahrenheit/README.md) | Rust | [Integer division truncates](languages/rust.md#int-division), [`{:.1}` formatting](languages/rust.md#format-spec) |

## Patterns

Not every worthwhile problem is a numbered LeetCode one. Common interview and
industry-coding patterns get the same treatment here — paraphrased in full, no
proprietary prompts — kept separate from the numbered table above.

| Pattern | Difficulty | Languages | Concepts |
|---------|-----------|-----------|----------|
| [In-Memory Database](patterns/in-memory-database/README.md) | Hard (multi-level) | Rust | [Sorted Map](glossary/sorted-map.md), [Lazy Expiration](glossary/lazy-expiration.md) |

## Glossary

Concepts are explained once and linked everywhere they're used. See the
[glossary index](glossary/README.md).

## Languages

Per-language syntax handbooks grow as solutions are added — every piece of
unfamiliar syntax is explained once and linked thereafter.

- [Rust](languages/rust.md)
