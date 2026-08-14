# Concept 14 · Enums — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 13](../13-structs/use-it.md)

## The idea
A [struct](../13-structs/use-it.md) bundles values that are all true *at once* — a `User`
has a name **and** an age, together. An **enum** is the mirror image: a value that is
**exactly one of several** shapes. Not "all of these," but "pick one of these."

Think of a traffic light. At any moment it shows **red**, *or* **yellow**, *or* **green** —
never two at once, never anything off the list. A fixed set of possibilities, always
sitting on exactly one:

```rust
enum Light {
    Red,
    Yellow,
    Green,
}
```

That reads: "a `Light` is `Red` **or** `Yellow` **or** `Green`." Each line is a
**variant** — one allowed possibility. Hold the two ideas side by side:

- **struct** → `x` **and** `y` (fields, all present together)
- **enum** → `Red` **or** `Yellow` **or** `Green` (variants, choose one)

## Creating one
Name the type, then the variant, joined by `::`

```rust
let stop = Light::Red;
let go   = Light::Green;
```

`Light::Red` means "a `Light`, specifically the `Red` one." The `::` just reaches inside
`Light` and grabs the variant.

## The superpower: variants can carry data
This is where enums stop being a plain list and become the workhorse of Rust. A variant
can **hold values of its own**, and different variants can hold *different* things:

```rust
enum Shape {
    Circle(f64),          // a circle carries one number: its radius
    Rectangle(f64, f64),  // a rectangle carries two: width and height
}

let round = Shape::Circle(2.0);
let boxy  = Shape::Rectangle(3.0, 4.0);
```

Now a single type `Shape` says: "I am **either** a circle with a radius **or** a rectangle
with a width and height." One value, one of two shapes, each carrying exactly the data
that shape needs — and nothing it doesn't.

## Doing something with it: a peek at `match`
To *act* on an enum you have to answer "which variant is it?" — and unpack whatever that
variant carries. Rust's tool for that is `match`. It gets its own lesson
([Concept 16](../README.md)); here's just enough to make an enum useful:

```rust
fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14159 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
    }
}
```

Read `match` as "check which variant, and pull its data out into names." If `shape` is a
`Circle`, `radius` is bound to the number inside it; if it's a `Rectangle`, `width` and
`height` are bound to its two numbers. Each arm handles one variant. The compiler also
checks you've covered **every** variant — forget one and it won't compile, so you can
never silently miss a case. (That guarantee is the whole point of `match`; more in its
own concept.)

## Exercises
1. **Name a direction** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Define `enum Direction { North, East, South, West }`, then a function that `match`es a
   `Direction` to its name and prints it. (Expect `heading West`.)
2. **Area of a shape** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Define `enum Shape { Circle(f64), Rectangle(f64, f64) }` and an `area` function using
   `match`. Print the area of a circle and a rectangle. (Expect `12.56636` then `12`.)

## Next
- What an enum actually *is* in memory — why it's a "tag plus a shared slot," why it takes
  as much room as its **biggest** variant even when holding the smallest, and why a plain
  `Light` is just 1 byte: [Under the hood](under-the-hood.md).
