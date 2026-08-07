# Concept 04 · Functions and the call stack — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 03](../03-types-have-sizes/use-it.md)

## The idea
A **function** is a little reusable machine: you feed it some input, it does a job, and
it hands back a result. You've actually been inside one this whole time — `main` is a
function. Now let's build our own.

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let x = 5;
    let result = double(x);
    println!("{result}");   // prints 10
}
```

Reading the line `fn double(n: i32) -> i32`:
- `fn` — "here comes a function."
- `double` — the name you're giving it.
- `(n: i32)` — its input: one value called `n`, of type `i32`. Inputs are called
  **parameters**, and each one needs a type (remember Concept 03 — Rust needs to know
  the size).
- `-> i32` — the type of the value it hands back.
- The body `n * 2` — the last line, written **with no semicolon**, is the value the
  function gives back. (You could also write `return n * 2;`.)

Then `double(x)` **calls** the machine: it runs `double` with `n` set to `5`, and the
`10` it returns lands in `result`.

## Exercises
1. **A square machine** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Write `square(n)` that returns `n * n`, call it with `5`, and print the result
   (should be `25`).
2. **Add two numbers** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Fill in `add(a, b)` so it returns their sum; print `add(3, 4)` (should be `7`).

## Next
- What actually happens in memory when one function calls another:
  [Under the hood](under-the-hood.md).
