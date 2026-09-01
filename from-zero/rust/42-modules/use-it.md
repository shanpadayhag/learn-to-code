# Concept 42 · Modules (`mod`, `pub`, and paths) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 41](../41-raw-pointers/use-it.md)

## The idea
Every program in this course so far has been one file with everything in it. At forty lines that is the right answer. At four hundred it stops being one — not because the file is long, but because *everything can reach everything*. Any function can call any other function. Any line can reach into any struct's fields. When a value is wrong, the list of suspects is the whole program.

A **module** is a named box you put items in, with a wall around the box. The wall is **closed by default**: the things inside are invisible from outside until you say otherwise. That is the whole feature. Two consequences fall out of it, and the second is the one that matters:

1. **Names stop colliding.** Two `parse` functions can coexist if they live in different boxes.
2. **You can rely on things.** If a piece of data is only reachable through three functions you wrote, then whatever those three functions guarantee is *true of that data, always*. Nothing else can have touched it. That is not tidiness — it is the difference between "I hope this stays in sync" and "this cannot go out of sync."

![Three panels: a module tree with a private constant at the crate root, a private and a public fn in mod weather, and a nested pub mod sensors — with a green arrow showing a child may look up into its parent's private items and a red arrow showing a parent may only reach in through pub; a files-on-disk column mapping to the compiler's module tree with an undeclared scratch.rs never read; and the difference between a bare name failing with E0425 and the same name reached through crate::](diagrams/module-tree.svg)

## Making one
`mod name { ... }` declares a module and puts its body right there:

```rust
mod weather {
    pub fn read_celsius() -> f64 {
        21.5
    }

    fn calibration_offset() -> f64 {
        0.4
    }
}

fn main() {
    println!("{}", weather::read_celsius());        // fine — it says pub
    println!("{}", weather::calibration_offset());  // error[E0603]: private
}
```

`::` is the path separator, the same one you have been typing since `std::collections::HashMap` in [Concept 18](../18-hashmap/use-it.md). You now know what it was separating: modules. `std` is a crate, `collections` is a module inside it, `HashMap` is a public item inside that.

Note where the error is. Not `E0425 cannot find value` — the compiler found `calibration_offset` perfectly well, and says so by name. It is refusing to let you *use* it. **E0603 is the wall talking.**

## The wall is one-way glass
Privacy in Rust has exactly one rule, and it is easier than it looks:

> An item is visible to the module it is in, **and to every module inside that one**, however deep. To anything else, it exists only if it is marked `pub`.

So a child can see up into its parent, and up into its grandparent, private items and all. A parent cannot see down into its child except through the `pub` windows. Looking up: always allowed. Looking down: only what was offered.

```rust
const SITE_NAME: &str = "north ridge";   // private to the crate root

mod weather {
    fn label() -> &'static str {         // private to weather
        "hourly reading"
    }

    pub mod sensors {
        pub fn describe() -> String {
            format!("{}, {}", super::label(), crate::SITE_NAME)
        }
    }
}
```

`sensors` reaches `weather::label` and the root's `SITE_NAME`, both private, and nothing complains. It is inside both of them. But `main` cannot call `weather::label()`, because `main` is standing outside that box.

This is why the wall is worth anything. The private things are private *to the outside*, which means the module can use them freely among its own functions while still promising the world a small, fixed set of entrances.

## Three ways to spell a path
A path is directions to an item. There are three starting points:

| path | means |
|---|---|
| `crate::weather::sensors::read_celsius()` | start at the crate root — absolute, works from anywhere |
| `super::label()` | start one module up — relative |
| `sensors::read_celsius()` | start here — relative to the module you are writing in |

Prefer `crate::` when you are naming something far away, and `super::`/relative when you are naming a close neighbour, because a relative path survives the whole subtree being moved somewhere else.

## `use` shortens a path; it does not open a door
```rust
use weather::sensors::read_celsius;
use weather::sensors as probe;

fn main() {
    println!("{}", read_celsius());          // the short name
    println!("{}", probe::read_celsius());   // the renamed module
}
```

Two things `use` is **not**. It does not grant permission — the item still has to be `pub`, and a `use` of a private item is the same E0603 one line earlier. And it does not move or copy anything; there is exactly one `read_celsius` in the program either way. `use` writes a nickname on your file's wall, nothing more. `as` renames the nickname, which is how you keep two `Result` types straight.

There is one thing `use` does that is genuinely different, and it is worth knowing early:

```rust
mod gauge {
    pub struct Gauge { /* ... */ }
}

pub use gauge::Gauge;   // now the root ALSO answers to `Gauge`
```

`pub use` is a **re-export**: it says "and let outsiders reach it by this shorter name too." It is how a library with fifteen modules inside offers you five names at the top. `std::collections::HashMap` is exactly this — the real type is buried further down, and `collections` re-exports it.

## Splitting into files
Once a module gets long, move its body to a file. Replace the braces with a semicolon:

```rust
// main.rs
mod weather;        // "the body of weather is in another file"

fn main() {
    println!("{}", weather::report());
}
```

```rust
// weather.rs
pub fn report() -> String {
    String::from("hourly reading")
}
```

The compiler looks for `weather.rs` next to the file that declared it, or `weather/mod.rs`. For a module with children of its own, make a folder: `weather.rs` holding `pub mod sensors;`, and `weather/sensors.rs` holding the body.

Now the sentence that trips up nearly everyone, and it is the whole model in one line:

> **The `mod` line creates the module. The file is only where its body is kept.**

A `.rs` file that no `mod` line ever mentions is not part of your program. It is not compiled, not checked, not warned about — it is a text file that happens to be in your folder. That is why deleting a `mod` line "deletes" a file's worth of code, and why the file layout is a convention the compiler follows rather than a thing it discovers.

## Public is not all-or-nothing
`pub` on a struct opens the *type*. Every field stays private unless it says so itself:

```rust
mod gauge {
    pub struct Gauge {
        readings: Vec<f64>,
        total: f64,        // invariant: always equals readings.iter().sum()
    }

    impl Gauge {
        pub fn new() -> Self { Gauge { readings: Vec::new(), total: 0.0 } }

        pub fn record(&mut self, celsius: f64) {
            self.readings.push(celsius);
            self.total += celsius;
        }

        pub fn average(&self) -> Option<f64> {
            if self.readings.is_empty() { None } else { Some(self.total / self.readings.len() as f64) }
        }
    }
}
```

`average` does no addition. It cannot be wrong about the total, because the only way a reading ever entered was `record`, and `record` updates both fields together. From outside, `gauge.readings.push(50.0)` is `error[E0616]: field 'readings' of struct 'Gauge' is private`, and even `Gauge { readings: vec![1.0], total: 99.0 }` is `E0451` — you cannot hand-build one.

That is the payoff promised at the top. Not "the code is organised" but "this cannot go out of sync," enforced at compile time, costing nothing at run time.

When you *do* want a plainly public field, say so per field: `pub readings: Vec<f64>`. And for the middle ground there is `pub(crate)`, meaning "public everywhere inside this crate, invisible to anyone who depends on it" — the right marking for a helper that several of your own modules share but that is nobody else's business.

Enums are the exception: `pub enum` makes every variant and every variant's fields public. An enum's whole purpose is that callers `match` on it, and they cannot match what they cannot see.

> Quick reference: [modules](../../../languages/rust.md#modules) in the handbook.

## Exercises
```bash
rustc --edition 2024 1-solution.rs && ./1-solution
```

1. **Building the tree** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs). Nest `sensors` inside `weather`, reach up to a private parent item with `super::` and to the crate root with `crate::`, shorten two paths with `use` and `as`, then trigger both E0603s on purpose. One step asks you to write a bare `SITE_NAME` and work out why E0425 is a *different* complaint from E0603.
2. **A field nobody can desynchronise** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs). Build the `Gauge` above with both fields private, add an `invariant_holds` function *inside* the module that can read them, re-export the type with `pub use`, then try three different ways to corrupt it from outside and read the error each one earns.

## Next
- Why `mod` is nothing like `#include` or a Python `import`, why a module costs exactly zero bytes and zero cycles at run time, how to read your own module path out of the compiled binary, and the one thing privacy buys the *optimizer* rather than you: [Under the hood](under-the-hood.md).
