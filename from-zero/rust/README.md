# From-Zero: Rust

A memory-first path through Rust, from "a number in a variable" all the way to
`async` and `unsafe`. The through-line for every lesson is one growing question:
**where does this value live, and who owns it?** Ordered this way, the famously hard
parts (ownership, borrowing) arrive as the *obvious next step*, not a wall.

New here? Read [how to learn](../README.md#how-to-learn-here) first.

## Progress

**Current:** Concept 03 — *types have sizes* (up next)

Each concept is a folder with a **Use it** lesson, an **Under the hood** lesson, and
exercises. This table is the source of truth for where you are.

### Phase 1 — The stack (values with a known size)

| # | Concept | Status |
|---|---------|--------|
| 01 | [A number in a variable](01-a-number-in-a-variable/use-it.md) | ✅ done |
| 02 | [Frozen by default, and `mut`](02-frozen-by-default-and-mut/use-it.md) | ✅ done |
| 03 | Types have sizes | ⬜ up next |
| 04 | Functions and the call stack | ⬜ planned |

### Phase 2 — The heap, and where ownership becomes obvious

| # | Concept | Status |
|---|---------|--------|
| 05 | `Copy` types | ⬜ planned |
| 06 | The heap, and `String` | ⬜ planned |
| 07 | Ownership and moves | ⬜ planned |
| 08 | `.clone()` (the inefficient fix) | ⬜ planned |
| 09 | Borrowing with `&` (the efficient answer) | ⬜ planned |
| 10 | `&mut` and the borrow rules | ⬜ planned |
| 11 | Slices | ⬜ planned |

### Later — macro-phases (firmed up as we reach them)

Compound data (structs · enums · `Option` · pattern matching) → collections &
generics (`Vec` · `HashMap` · `<T>`) → traits → lifetimes → error handling
(`Result` · `?`) → smart pointers (`Box` · `Rc` · `RefCell`) → closures & iterators →
advanced (concurrency · `async` · `unsafe`).

## How this relates to the rest of the repo

- The [Rust syntax handbook](../../languages/rust.md) is the *terse reference* — quick
  lookup once you know a construct. These lessons *teach* and link to it.
- The [glossary](../../glossary/README.md) holds language-agnostic concepts shared
  across the repo.
