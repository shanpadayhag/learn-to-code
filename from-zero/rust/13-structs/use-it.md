# Concept 13 · Structs — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 12](../12-slices/use-it.md)
> **Phase 3 — compound data** starts here.

## The idea
Every value so far held **one** thing: a number, a `bool`, a `String`. But real things
have several parts at once — a point has an `x` *and* a `y`; a user has a name *and* an
age. A **struct** lets you bundle several named values into a single custom type you can
name and pass around as one.

You define the shape once:

```rust
struct Point {
    x: i32,
    y: i32,
}
```

That reads: "a `Point` is an `x` **and** a `y`, together." Each line is a **field** — a
named slot with its own type.

## Creating one and reading it
You build a value by naming every field, then read a field with a **dot**:

```rust
let p = Point { x: 3, y: 4 };
println!("{} {}", p.x, p.y);   // 3 4
```

`Point { x: 3, y: 4 }` fills in each slot; `p.x` and `p.y` pull them back out. The field
order in `{ ... }` doesn't have to match the definition — the names do the matching.

## Changing a field
As always in Rust, changing needs `mut` ([Concept 02](../02-frozen-by-default-and-mut/use-it.md)) —
and it's the *whole struct value* that must be `mut`, not the individual field:

```rust
let mut c = Counter { count: 0 };
c.count += 5;
c.count += 1;
println!("{}", c.count);   // 6
```

If `c` were a plain `let`, `c.count += 1` wouldn't compile
(`error[E0594]: cannot assign to c.count, as c is not declared as mutable`).

## Fields can be any type — including owning ones
A field can hold anything, including a `String`:

```rust
struct User {
    name: String,
    age: u32,
}

let u = User {
    name: String::from("Sam"),
    age: 30,
};
println!("{} is {}", u.name, u.age);   // Sam is 30
```

Everything you learned in Phase 2 still applies, now to the *fields*: the struct **owns**
its fields, so because `name` owns heap text, the whole `User` is not `Copy`, and moving
the `User` moves all its fields at once. Structs don't add new ownership rules — they just
group values that follow the rules you already know. The memory picture is in
[Under the hood](under-the-hood.md).

## Exercises
1. **Make a Point** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Create a `Point { x: 3, y: 4 }` and print both fields. (Expect `3 4`.)
2. **Change a field** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Make a `mut` `Counter`, add to its `count` twice, print it. (Expect `6`.)

## Next
- How a struct is laid out in memory, and why a struct with a `String` field lives partly
  on the stack and partly on the heap: [Under the hood](under-the-hood.md).
