# Concept 05 · Expressions, statements, and return — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 04](../04-functions-and-the-call-stack/use-it.md)

## The idea
In Concept 04 a function handed back a value in a slightly odd-looking way — the last
line had **no semicolon**:

```rust
fn double(n: i32) -> i32 {
    n * 2          // no semicolon — this IS the value handed back
}
```

That's not a typo, and it's worth really understanding, because you'll see it
everywhere in Rust.

There are **two ways** a function gives a value back:

**1. The last line, with no semicolon** (the usual, tidy way):
```rust
fn double(n: i32) -> i32 {
    n * 2
}
```

**2. The `return` keyword** (hands it back right away):
```rust
fn double(n: i32) -> i32 {
    return n * 2;
}
```

Both do exactly the same thing here. The difference in practice: `return` is mostly for
handing something back **early**, before the end of the function; the no-semicolon last
line is the normal way to give the *final* value.

## The semicolon is the switch
Here's the rule hiding underneath: **a line without a semicolon produces a value;
adding a semicolon throws that value away.** So this **breaks**:

```rust
fn double(n: i32) -> i32 {
    n * 2;     // ❌ the ; throws the 10 away, so the function hands back nothing
}
```

Rust stops you: you promised to hand back an `i32`, but handed back nothing. Removing
that one semicolon fixes it. The full "why" is in [Under the hood](under-the-hood.md).

## A block hands back a value — so `if` can too
A function's body is a block, and its value is its last expression. But `if`/`else` is built
from blocks too — one block per arm — so the **whole `if` is itself an expression** that hands
back whichever arm ran. That means you can store its result straight into a variable, instead of
declaring the variable first and assigning inside each branch:

```rust
let parity = if n % 2 == 0 { "Even" } else { "Odd" };
println!("{parity}");
```

Coming from JavaScript, this is the job the ternary `cond ? a : b` does — except Rust has no
special ternary, because the ordinary `if` *already* produces a value. Two rules to keep
straight: every arm must produce the **same type** (both `&str` here), and when you use `if` for
its value the `else` is **required** — without it, the "no" path would produce nothing (`()`) and
the types wouldn't line up. (`match` and `loop` are expressions in exactly the same way — you'll
meet those later.)

## Exercises
1. **One semicolon too many** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   The function won't compile because its last line has a stray semicolon. Remove it so
   it returns `10`.
2. **The tidy way** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Rewrite an `add_one` that uses `return` into the last-line, no-semicolon style. It
   should still print `6`.
3. **Store an `if` in a variable** — [starter](exercises/3-starter.rs) · [solution](exercises/3-solution.rs).
   Using a single `let parity = if … { … } else { … };`, store `"Even"` or `"Odd"` for the given
   `n`, then print it. (For `n = 7`, expect `Odd`.)

## Next
- Why one little semicolon changes everything: [Under the hood](under-the-hood.md).
