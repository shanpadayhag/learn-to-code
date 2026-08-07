# Concept 03 · Types have sizes — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 02](../02-frozen-by-default-and-mut/use-it.md)

## The idea
Every value in Rust has a **type** — a kind of thing. `5` is a whole number, `true` is
a yes/no, `'A'` is a single character. Each type has a name, and Rust cares about the
type a lot — for one big reason you'll see in [Under the hood](under-the-hood.md):
**each type is a fixed number of bytes.** (A byte is just a tiny unit of memory — think
of it as one little cell on the shelf.)

Some common types:

| type | what it holds | size |
|---|---|---|
| `i32` | a whole number (Rust's default) | 4 bytes |
| `i64` | a bigger whole number | 8 bytes |
| `u8` | a small whole number, 0–255 | 1 byte |
| `bool` | `true` or `false` | 1 byte |
| `char` | one single character | 4 bytes |
| `f64` | a number with a decimal point | 8 bytes |

When you write `let x = 5;`, Rust quietly picks `i32` for you. You can also spell the
type out yourself with a colon:

```rust
let big: i64 = 5;   // "make this one an i64"
```

## Peeking at a size
Rust can tell you exactly how many bytes a type takes, using `size_of`:

```rust
fn main() {
    println!("{}", std::mem::size_of::<i32>());  // prints 4
    println!("{}", std::mem::size_of::<i64>());  // prints 8
}
```

The `::<i32>` part just tells `size_of` *which* type to measure. Same value idea from
before — but now you can see that different types take up different amounts of room.

## Exercises
1. **Number sizes** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Print how many bytes an `i32` and an `i64` take. (Expect `4`, then `8`.)
2. **A surprising size** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Guess first, then print the size of a `bool` and a `char`. One of them may surprise
   you. (Expect `1`, then `4`.)

## Next
- Why Rust insists on knowing the type at all: [Under the hood](under-the-hood.md).
