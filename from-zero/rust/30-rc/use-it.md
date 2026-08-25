# Concept 30 · `Rc<T>` (many owners, one value) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 29](../29-box/use-it.md)

## The idea
Every owning type you've met has followed one iron rule: **exactly one owner at a time**. A
[`String`](../07-the-heap-and-string/use-it.md) has one owner; give it away and the old
variable is retired. A [`Box`](../29-box/use-it.md) has one owner; move it and the old one is
done. That single-owner rule is what makes cleanup unambiguous — when the one owner goes out of
scope, the value is freed, no confusion about who's responsible.

But some shapes don't fit "one owner." Picture two lists that need to **share the same tail**,
or a family tree where several children point at the *same* parent node. Who owns that shared
node? Not any one of them — they *all* do, and it should stay alive as long as **any** of them
still needs it. With a `Box` you'd be stuck: move the node into one owner and the others can't
reach it; clone it and now there are separate copies that drift apart.

`Rc<T>` is the answer:

> **`Rc<T>` lets many owners share one value on the heap. It counts them, and frees the value
> only when the *last* owner lets go.**

`Rc` stands for **reference counted**. It keeps a tally of how many owners currently exist. Each
new owner adds one to the count; each owner that goes away subtracts one. When the count hits
zero — nobody left — the value is freed. It's cooperative ownership with an automatic
last-one-out-frees-it rule.

![Two Rc handles pointing at one heap value that carries a shared owner count](diagrams/rc-shared.svg)

## Creating owners with `Rc::new` and `Rc::clone`
```rust
use std::rc::Rc;

let first = Rc::new(String::from("shared text"));   // count is now 1
let second = Rc::clone(&first);                      // count is now 2 — a second owner
let third = Rc::clone(&first);                       // count is now 3

println!("{first} / {second} / {third}");            // all three read the SAME string
```

`Rc::new(value)` puts the value on the heap and gives you the **first** owner. `Rc::clone(&first)`
makes **another owner of the same value** — and this is the important part: it does **not** copy
the string. It bumps the shared count by one and hands back a second pointer to the *same*
allocation. All three variables read the exact same heap string; there's only ever one copy of
`"shared text"` in memory.

> **`Rc::clone` is cheap.** Unlike [`String::clone`](../09-clone-the-inefficient-fix/use-it.md),
> which duplicates the whole heap buffer, `Rc::clone` copies nothing but the pointer and adds
> `1` to a counter. That's why it's written `Rc::clone(&x)` and not `x.clone()` in idiomatic code
> — the explicit form is a visual flag that says "this is a *cheap* reference-count bump, not a
> deep copy."

## Reading the count
`Rc::strong_count` tells you how many owners exist right now — handy for *seeing* the mechanism:

```rust
use std::rc::Rc;

let first = Rc::new(String::from("hi"));
println!("{}", Rc::strong_count(&first));   // 1

let second = Rc::clone(&first);
println!("{}", Rc::strong_count(&first));   // 2

{
    let third = Rc::clone(&first);
    println!("{}", Rc::strong_count(&first)); // 3
}   // third goes out of scope here → count drops back to 2

println!("{}", Rc::strong_count(&first));   // 2
```

Watch the count rise as owners are made and fall as they go out of scope. When `third` reaches
the end of its inner block, its ownership is dropped and the count ticks back down — automatically,
no code to write. The value itself stays alive because `first` and `second` still hold it.

## Cloning is the point — moving would retire the original
Because an `Rc` is still an owning value, handing it somewhere by plain assignment **moves** it,
just like a `Box`:

```rust
use std::rc::Rc;
fn takes(_owner: Rc<String>) {}

let a = Rc::new(String::from("hi"));
takes(a);        // a is MOVED into the function
takes(a);        // ❌ error: value used here after move
```

The compiler even suggests the fix:

```
error[E0382]: use of moved value: `a`
help: this move could be avoided by cloning the original `Rc`, which is inexpensive
```

That's the whole workflow: when you want *another* owner (not to give up the one you have), call
`Rc::clone` — inexpensive, because it's just a count bump:

```rust
takes(Rc::clone(&a));   // hand over a fresh owner
takes(Rc::clone(&a));   // and another — a is still yours
```

## The catch: `Rc` gives you shared reads, not writes
Shared ownership comes with a limit that follows straight from the [borrow
rules](../11-mut-references-and-borrow-rules/use-it.md): if several owners can all reach one
value, letting any of them *mutate* it would be exactly the "many aliases + mutation" that Rust
forbids. So `Rc<T>` only ever hands out **shared, read-only** access to the value. You can read
through any owner; you cannot get a `&mut` to the inside:

```rust
let shared = Rc::new(String::from("hi"));
// shared.push_str("!");   // ❌ can't mutate — Rc gives read-only shared access
println!("{shared}");       // ✅ reading is fine
```

That's not a dead end — it's the exact seam the **next** tool fills. `RefCell<T>` re-introduces
mutation *safely* by moving the borrow check from compile time to run time, and `Rc<RefCell<T>>`
— "shared ownership *and* the ability to mutate" — is one of Rust's workhorse combinations.
This lesson is the "shared ownership" half.

## One more note: single-threaded
`Rc`'s counter is *not* safe to touch from two threads at once (two threads bumping the same
count could corrupt it). So `Rc` is for **single-threaded** sharing only — the compiler will
stop you from sending one across threads. When you need the same sharing *across* threads, the
thread-safe sibling is `Arc<T>` (**a**tomically **r**eference **c**ounted); it works identically
but pays a small cost to make the count safe. You'll meet `Arc` in the concurrency phase; for now,
`Rc` within one thread.

> Quick reference: the [`Rc<T>` handbook entry](../../../languages/rust.md#rc) is the terse
> lookup version.

## Exercises
1. **Watch the count move** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Make an `Rc<String>`, print `Rc::strong_count` (1). Clone it into a second owner and print
   again (2). Open an inner `{ }` block, clone a third owner inside it and print (3), then let the
   block end and print once more (back to 2).
2. **One config, many readers** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Put a `String` app-name in an `Rc`. Write a function `fn show(name: Rc<String>)` that prints it.
   Call `show` twice, passing a fresh `Rc::clone` each time, and confirm the original is still
   usable afterwards by printing it and its final count.

## Next
- What the counter physically *is* — a second number sitting next to your value on the heap — why
  `Rc::clone` is one pointer copy plus `+1`, and how the count reaching zero triggers the free:
  [Under the hood](under-the-hood.md).
- Then the smart-pointer story continues with **`RefCell<T>`** and interior mutability — the
  read-only limit above, lifted safely — and the famous `Rc<RefCell<T>>` pairing.
