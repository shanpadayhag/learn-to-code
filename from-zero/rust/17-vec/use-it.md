# Concept 17 · `Vec<T>` (a growable list) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 16](../16-match/use-it.md)

## The idea
So far every value has been *one thing* — a number, a `String`, one `struct`. But you
constantly need **many** things: a list of scores, the lines of a file, every user. And you
usually don't know how many up front.

A plain array (`[i32; 3]`) can't grow — its length is fixed and baked into its type
([Concept 03](../03-types-have-sizes/use-it.md)). What you want is a list you can keep adding
to. That's a **`Vec<T>`** — a *vector*, Rust's growable list. The `<T>` is the type of thing
inside: a `Vec<i32>` is a list of `i32`s, a `Vec<String>` a list of strings. (It's the same
`<T>` placeholder you met on [`Option`](../15-option/use-it.md) — "a `Vec` of *whatever*.")

Picture a row of numbered boxes that can always add one more box on the end:

```rust
let scores = vec![88, 92, 79];   // a Vec<i32> with three items
```

## Making one
Two common ways:

```rust
let a = vec![88, 92, 79];        // the vec![] macro: start with these items
let mut b: Vec<i32> = Vec::new(); // start empty, fill it later
```

Use `vec![...]` when you know the starting items; use `Vec::new()` when you'll build it up.
To add to the end, `.push`:

```rust
let mut b = Vec::new();
b.push(88);   // b is now [88]
b.push(92);   // b is now [88, 92]
```

Note the `mut` ([Concept 02](../02-frozen-by-default-and-mut/use-it.md)) — pushing *changes*
the Vec, so it must be mutable. And you never wrote the type on `b`: Rust sees `b.push(88)` and
figures out it's a `Vec<i32>` on its own.

## Reading items
By position, with `[i]` (positions start at **0**):

```rust
let scores = vec![88, 92, 79];
println!("{}", scores[0]);   // 88  (the first)
println!("{}", scores[1]);   // 92
```

But there's a trap: `scores[5]` on a 3-item list **crashes the program**. When you're not
sure the index exists, use `.get(i)` instead — it hands back an [`Option`](../15-option/use-it.md),
so "no such index" becomes a `None` you handle instead of a crash:

```rust
match scores.get(5) {
    Some(value) => println!("got {value}"),
    None => println!("nothing at index 5"),
}
```

This is `Option` earning its keep: the "might not be there" case is visible and can't sneak
past you.

## Walking every item
A `for` loop over `&the_vec` visits each element in turn (the `&` borrows the list so the loop
just *reads* it, leaving the Vec yours afterward — [Concept 10](../10-borrowing-with-ref/use-it.md)):

```rust
let scores = vec![88, 92, 79];
let mut total = 0;
for score in &scores {
    total += score;
}
println!("{total}");   // 259
```

## A few more everyday moves
- `scores.len()` — how many items (here `3`).
- `scores.pop()` — remove and return the **last** item, as an `Option` (`None` if empty).
- `scores.is_empty()` — `true` when there's nothing in it.

```rust
let mut stack = vec![1, 2, 3];
let last = stack.pop();     // Some(3); stack is now [1, 2]
println!("{}", stack.len()); // 2
```

## Exercises
1. **Build a list of squares** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Write `fn squares(n: u32) -> Vec<u32>` that returns `[1, 4, 9, …, n*n]` — start with
   `Vec::new()`, `push` each square, return it. Then sum them in `main`. (Expect
   `[1, 4, 9, 16, 25]` then `55`.)
2. **Safe lookup** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Given `vec![88, 92, 79]`, use `.get(index)` and `match` to print the score at index `1`
   and at index `5` (out of range). (Expect `score at 1: 92`, then `no score at index 5`.)

## Next
- What a `Vec` really is in memory — a tiny **three-number header** (pointer, length, capacity)
  on the stack pointing at a buffer on the heap — why `.push` is usually instant but
  occasionally has to move the whole buffer to a bigger home, and why that move can break old
  references: [Under the hood](under-the-hood.md).
