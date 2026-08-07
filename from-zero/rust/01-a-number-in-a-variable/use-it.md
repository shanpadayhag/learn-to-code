# Concept 01 · A number in a variable — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md)

## The idea
A *variable* is a name for a value. In Rust you create one with `let`:

```rust
let x = 5;
```

Read it left to right: `let` says "I'm naming a value," `x` is the name you chose,
`=` binds the name to the value, and `5` is the value. The `;` ends the statement.
That's it — `x` now means `5`.

## A whole program
Rust starts every program by running a function called `main`. Here's the smallest
program that stores a number and shows it:

```rust
fn main() {
    let x = 5;
    println!("{x}");
}
```

Line by line:
- `fn main() { … }` — the starting point of every Rust program. Your code goes between
  the `{` and `}`.
- `let x = 5;` — name the value `5` as `x`.
- `println!("{x}");` — print it, followed by a newline. Inside the quotes, `{x}` is a
  *hole* that gets filled with the value of `x`. `println!` is a **macro** (the `!`
  marks it as one), and it prints a line of text.

Run it, and the output is:

```
5
```

## Poke at it
Small changes to build intuition — try each and predict the output first:
- Change `5` to a different number.
- Add `let y = 8;` and a second `println!("{y}");`.
- Print a sentence: `println!("my number is {x}");`.

> `let` here creates a value you don't change again. Whether a variable can be
> *changed* is the very next concept — see the [Rust handbook: `let` / `let mut`](../../../languages/rust.md#let-mut)
> for the quick reference, and [Concept 02](../README.md) for the full lesson.

## Exercises
Write, run, and check each against its solution file.

1. **Store and print a number** — [starter](exercises/1-starter.rs) ·
   [solution](exercises/1-solution.rs). Put your favorite number in a variable and
   print it on its own line.
2. **Two variables, one sentence** — [starter](exercises/2-starter.rs) ·
   [solution](exercises/2-solution.rs). Store a number of apples, then print
   `I have 3 apples` (with your number in place of `3`).

You've passed when both compile, run, and print what you expect.

## Next
- The memory side of this same idea: [Under the hood](under-the-hood.md) — where `5`
  physically goes, and when it disappears.
