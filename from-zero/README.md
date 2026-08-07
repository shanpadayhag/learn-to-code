# From-Zero

Learn a programming language from **absolute zero**, the way it actually works —
by *seeing what happens in memory*, one small idea at a time.

Most language tutorials hand you syntax and hope the meaning sinks in later. This
course does the opposite: it draws the **memory picture first** (where a value lives,
who owns it, what a line physically does), and then the syntax makes sense because
you can *see* what it's for. It's built for people who learn by visualizing.

## How to learn here

- **One new idea per lesson.** Never two at once. If something needs two ideas, it's
  split into two lessons.
- **Every concept comes as a pair** you can read independently:
  - **Use it** — the surface: how you write it and what it does.
  - **Under the hood** — the memory picture beneath it (stack, heap, ownership).
  Read just the *Use it* lessons to get productive, or read both to truly understand.
- **You learn by doing.** Each lesson ends with tiny **run-it exercises** (write the
  code, run it, check it against the solution). *Under the hood* lessons add a
  **predict-the-memory** check. Passing them means you're ready for the next lesson.
- **Inefficient → efficient.** Often you'll write the clumsy-but-clear version first,
  feel why it's clumsy, then earn the better one.
- **It's interactive.** Working through this with the tutor? Just ask *"what should I
  learn today?"* and it resumes from where you left off, one problem at a time.

## How lessons are structured

Each concept is a numbered folder, so the repo reads in learning order:

```
<track>/NN-concept-slug/
├── use-it.md          # surface syntax: how to write it, what it does
├── under-the-hood.md  # the memory picture beneath it
├── diagrams/          # memory diagrams as .svg (crisp, theme-aware)
│   └── *.svg
└── exercises/
    ├── N-starter.rs   # what you write ( // your code here )
    └── N-solution.rs  # the answer to check against
```

Every solution file is **verified to actually compile and run** before it ships — no
hand-waved code. (For contributors: drop the solution into a throwaway
`cargo new` project, or run it with `rustc solution.rs && ./solution`, and confirm the
output the lesson claims.) The memory pictures are checked against real Rust behavior
the same way.

## Tracks

| Language | Status | Start here |
|---|---|---|
| [Rust](rust/README.md) | In progress | [Rust roadmap](rust/README.md) |

More languages slot in later as sibling tracks with the same shape.
