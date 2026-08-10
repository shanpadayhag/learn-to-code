# Concept 07 · The heap, and `String` — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 06](../06-copy-types/use-it.md)

## The idea
Every value so far had a size Rust knew while *writing* the program: an `i32` is always
4 bytes, a `bool` always 1 (that was [Concept 03](../03-types-have-sizes/use-it.md)). But
some text you can't measure ahead of time — a name someone types in, a file you read, a
message you keep adding words to. Rust can't reserve a fixed box for something that can
**grow at runtime**.

For that, Rust uses a second area of memory called the **heap** — a big open space where
a program can ask for room *while it runs*, and ask for more later. The everyday type
that lives there is **`String`**: owned text you can grow.

## Making one
```rust
let a = String::from("Hello");   // from a literal
let b = "Hello".to_string();     // same thing, different spelling
let c = String::new();           // an empty String, ready to grow
```

A bare `"Hello"` in your code is *not* a `String` — it's a fixed literal baked into the
program (its type is `&str`). `String::from` takes that literal and makes a growable,
owned copy on the heap. (The full `String` vs `&str` split is in the
[handbook](../../../languages/rust.md#string); we only need `String` here.)

## Growing it
```rust
let mut greeting = String::from("Hello");
greeting.push_str(", world");   // add a piece of text
greeting.push('!');             // add a single character
// greeting is now "Hello, world!"
```

- `.push_str("...")` appends a run of text.
- `.push('c')` appends **one** character (note the single quotes — that's a `char`).
- `.len()` tells you how many **bytes** it holds right now.

```rust
let mut name = String::from("Ann");
println!("{}", name.len());   // 3
name.push_str("ie");
println!("{}", name.len());   // 5  — it grew
```

Notice `greeting` and `name` are `let mut` — growing a string *changes* it, and
[Concept 02](../02-frozen-by-default-and-mut/use-it.md) said changing needs `mut`.

## Exercises
1. **Build it up** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Start from `"Hello"`, append `", world"`, then a `'!'`, and print it.
   (Expect `Hello, world!`.)
2. **Watch it grow** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Print a String's `.len()`, push more onto it, print `.len()` again.
   (Expect `3`, then `5`.)

## Next
- Where those characters actually live, and why a `String` variable is only a tiny
  *handle* on the stack: [Under the hood](under-the-hood.md). This is the picture that
  makes ownership (Concept 08) obvious.
