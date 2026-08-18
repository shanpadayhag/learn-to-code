# Concept 19 · Generics `<T>` (one definition, any type) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 18](../18-hashmap/use-it.md)

## The idea
You've now met `<T>` three times: [`Option<T>`](../15-option/use-it.md),
[`Vec<T>`](../17-vec/use-it.md), and [`HashMap<K, V>`](../18-hashmap/use-it.md). Each one works
with *any* type you put inside — a `Vec<i32>`, a `Vec<String>`, a `Vec<Point>`, all from one `Vec`.
Nobody wrote a separate `VecOfInt` and `VecOfString`. That "works for any type" power is called
**generics**, and now you get to use it in your *own* code.

The problem it solves: say you write a function to return the first item of a pair of `i32`s.

```rust
fn first_i32(pair: (i32, i32)) -> i32 {
    pair.0
}
```

Now you need the same thing for a pair of `String`s. And a pair of `bool`s. You'd copy-paste the
*identical* body three times, changing only the type in the signature. That's silly — the logic
doesn't care what type it is; it just returns `.0`. **Generics** let you write it **once** with a
stand-in name for "some type," and use it with every type.

## A generic function
Put a `<T>` after the function name to introduce a **type parameter** — a placeholder that means
"some type, decided later." Then use `T` wherever you'd normally write a concrete type:

```rust
fn first<T>(pair: (T, T)) -> T {
    pair.0
}

fn main() {
    let a = first((10, 20));                 // T is i32  → returns 10
    let b = first(("hi", "bye"));            // T is &str → returns "hi"
    let c = first((true, false));            // T is bool → returns true
    println!("{a} {b} {c}");                 // 10 hi true
}
```

One definition, three types. Read `<T>` as "for **any** type `T`." You never say what `T` is — Rust
looks at how you *call* `first` and fills it in: `first((10, 20))` has `i32`s, so `T` is `i32` there.
`T` is just a name; `<U>`, `<Item>`, or `<Elem>` work identically. `T` is only convention for "Type."

## A generic struct
[Structs](../13-structs/use-it.md) can be generic too. Here's a point that holds *any* type of
coordinate — two `i32`s, or two `f64`s, from one definition:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let whole = Point { x: 3, y: 4 };        // Point<i32>
    let precise = Point { x: 1.5, y: 2.5 };  // Point<f64>
    println!("{} {}", whole.x, precise.y);   // 3 2.5
}
```

Both fields are `T`, so they must be the *same* type: `Point { x: 3, y: 4 }` makes both `i32`. If you
wanted `x` and `y` to be *independent* types, you'd give the struct two parameters — `Point<T, U>`
with `x: T, y: U` — exactly like `HashMap<K, V>` uses two.

## The honest limit: a generic `T` can't be *inspected* yet
Here's the catch that trips everyone up. This looks reasonable but **won't compile**:

```rust
fn larger<T>(a: T, b: T) -> T {
    if a > b { a } else { b }   // ❌ error: can't compare two T with `>`
}
```

Why? Inside `larger`, Rust knows *nothing* about `T` — it could be a type that has no `>` operator at
all. Since the function promises to work for **every** type, Rust refuses any operation that isn't
guaranteed for every type. Comparing, printing, adding — all off-limits on a bare `T`.

So what *can* you do with a bare `T`? Only the things every type supports: **move it, return it, store
it, put it in a tuple or struct, hand it back out.** That's why the examples above only *pass values
around* — they never look inside them.

To actually *compare* or *print* a `T`, you have to promise Rust "`T` is a type that can be compared"
— a **trait bound**, written `fn larger<T: PartialOrd>(...)`. That's the very next concept, and it's
what unlocks generics that *do* things. For now: generics let you write logic once for any type, as
long as that logic only shuffles values around.

## Exercises
1. **A generic `swap`** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Write `fn swap<T>(a: T, b: T) -> (T, T)` that returns the two values in the opposite order. Call
   it once with two `i32`s and once with two `&str`s. (Expect `(20, 10)` then `("world", "hello")`.)
2. **A generic wrapper struct** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Define `struct Wrapper<T> { value: T }`, then build a `Wrapper<i32>` holding `42` and a
   `Wrapper<&str>` holding `"hi"`, and print each `.value`. (Expect `42` then `hi`.)

## Next
- Why writing `first` once doesn't make it *slower* than three hand-written copies — the compiler's
  trick of **monomorphization**: at compile time it stamps out a separate concrete version of your
  generic code for each type you actually use, so `<T>` costs **nothing** at runtime. This is also
  why a `T` needs a known size, tying straight back to
  [Concept 03](../03-types-have-sizes/use-it.md): [Under the hood](under-the-hood.md).
