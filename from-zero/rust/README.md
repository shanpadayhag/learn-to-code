# From-Zero: Rust

A memory-first path through Rust, from "a number in a variable" all the way to
`async` and `unsafe`. The through-line for every lesson is one growing question:
**where does this value live, and who owns it?** Ordered this way, the famously hard
parts (ownership, borrowing) arrive as the *obvious next step*, not a wall.

New here? Read [how to learn](../README.md#how-to-learn-here) first.

## Progress

**Current:** Concept 21 — *Trait objects (`dyn Trait`)* (up next)

Each concept is a folder with a **Use it** lesson, an **Under the hood** lesson, and
exercises. **Interludes** (numbered like `01a`) are lighter, single-lesson detours
that teach an everyday basic where it first comes up — no separate memory page, since
they aren't memory topics. This table is the source of truth for where you are.

### Phase 1 — The stack (values with a known size)

| # | Concept | Status |
|---|---------|--------|
| 01 | [A number in a variable](01-a-number-in-a-variable/use-it.md) | ✅ done |
| 01a | [Printing and formatting](01a-printing-and-formatting/use-it.md) *(interlude)* | ✅ done |
| 02 | [Frozen by default, and `mut`](02-frozen-by-default-and-mut/use-it.md) | ✅ done |
| 03 | [Types have sizes](03-types-have-sizes/use-it.md) | ✅ done |
| 04 | [Functions and the call stack](04-functions-and-the-call-stack/use-it.md) | ✅ done |
| 05 | [Expressions, statements, and return](05-expressions-statements-and-return/use-it.md) | ✅ done |
| 05a | [Loops and ranges](05a-loops-and-ranges/use-it.md) *(interlude)* | ✅ done |

### Phase 2 — The heap, and where ownership becomes obvious

| # | Concept | Status |
|---|---------|--------|
| 06 | [`Copy` types](06-copy-types/use-it.md) | ✅ done |
| 07 | [The heap, and `String`](07-the-heap-and-string/use-it.md) | ✅ done |
| 08 | [Ownership and moves](08-ownership-and-moves/use-it.md) | ✅ done |
| 09 | [`.clone()` (the inefficient fix)](09-clone-the-inefficient-fix/use-it.md) | ✅ done |
| 10 | [Borrowing with `&` (the efficient answer)](10-borrowing-with-ref/use-it.md) | ✅ done |
| 11 | [`&mut` and the borrow rules](11-mut-references-and-borrow-rules/use-it.md) | ✅ done |
| 12 | [Slices](12-slices/use-it.md) | ✅ done |
| 12a | [Why you can't index a string by position](12a-string-indexing/use-it.md) *(interlude)* | ✅ done |

### Phase 3 — Compound data (types you design)

| # | Concept | Status |
|---|---------|--------|
| 13 | [Structs](13-structs/use-it.md) | ✅ done |
| 14 | [Enums](14-enums/use-it.md) | ✅ done |
| 15 | [`Option` (no more null)](15-option/use-it.md) | ✅ done |
| 16 | [Pattern matching with `match`](16-match/use-it.md) | ✅ done |

### Phase 4 — Collections and generics (many values, any type)

| # | Concept | Status |
|---|---------|--------|
| 17 | [`Vec<T>` (a growable list)](17-vec/use-it.md) | ✅ done |
| 18 | [`HashMap<K, V>` (look up by key)](18-hashmap/use-it.md) | ✅ done |
| 19 | [Generics `<T>` (one definition, any type)](19-generics/use-it.md) | ✅ done |
| 20 | [Traits (what a type can do)](20-traits/use-it.md) | ✅ done |
| 21 | Trait objects (`dyn Trait`) | ⬜ up next |

### Later — macro-phases (firmed up as we reach them)

Trait objects → more collections → lifetimes → error handling
(`Result` · `?`) → smart pointers (`Box` · `Rc` · `RefCell`) → closures & iterators →
advanced (concurrency · `async` · `unsafe`).

## How this relates to the rest of the repo

- The [Rust syntax handbook](../../languages/rust.md) is the *terse reference* — quick
  lookup once you know a construct. These lessons *teach* and link to it.
- The [glossary](../../glossary/README.md) holds language-agnostic concepts shared
  across the repo.
