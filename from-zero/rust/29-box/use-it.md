# Concept 29 · `Box<T>` (put one value on the heap) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 28b](../28b-zip-take-takewhile-count/use-it.md)

## The idea
Way back in [Concept 07](../07-the-heap-and-string/use-it.md) you met the two places a value
can live: the **stack** (fast, small, known size, cleaned up automatically) and the **heap**
(the big pool for things that are large or whose size isn't known until the program runs). So
far only a handful of built-in types — `String`, `Vec`, `HashMap` — quietly kept their contents
on the heap for you. Everything *you* declared lived **inline**: an `i32`, a struct, an enum
all sat right where you wrote them, on the stack.

`Box<T>` is the first tool that lets *you* say "take this value and put it on the heap." That's
the whole job:

> **`Box<T>` moves one value onto the heap and keeps a pointer to it on the stack.**

The `Box` **owns** that heap value — one owner, exactly like a plain variable — and when the
`Box` goes out of scope, the heap value is freed automatically. No manual cleanup, no garbage
collector. It is the simplest of Rust's **smart pointers**: a pointer that also owns what it
points at and tidies up after itself.

![A Box on the stack is a small pointer; the value it owns lives out on the heap](diagrams/box-on-heap.svg)

## Creating one, and reaching inside
```rust
let boxed: Box<i32> = Box::new(10);   // the 10 now lives on the heap
let value = *boxed + 5;               // *boxed follows the pointer to the 10
println!("{value}");                  // 15
```

`Box::new(10)` allocates space on the heap, moves `10` into it, and hands you back a `Box<i32>`
— a pointer that owns that heap slot. To read the value *through* the pointer you
[dereference](../10a-dereferencing-with-star/use-it.md) it with `*`, exactly like following a
`&` reference back to what it points at. The difference from `&` is ownership: a `&i32` borrows
a number someone else owns; a `Box<i32>` **owns** the number it points to.

In practice you rarely write the `*` by hand, because field and method access **auto-dereference
through the box** for you:

```rust
struct Point { x: i32, y: i32 }
let boxed = Box::new(Point { x: 3, y: 4 });
println!("{}", boxed.x);   // 3 — no *, Rust follows the box automatically
```

## Why you'd ever want this: a type that contains itself
For a plain `i32` a box is pointless overhead — the number is tiny and lives happily on the
stack. The reason `Box` *exists* shows up the moment you try to build a type that **holds more
of its own kind**. The classic is a linked list, where each node points to the next node:

```rust
struct Node {
    val: i32,
    next: Option<Node>,   // ❌ does not compile
}
```

To lay a `Node` out in memory, the compiler must know how many bytes one `Node` takes. But a
`Node` contains a `Node`, which contains a `Node`, ... forever. The size is infinite, and the
compiler says so literally:

```
error[E0072]: recursive type `Node` has infinite size
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
```

A `Box` breaks the cycle. A pointer is a **fixed size** (8 bytes on a 64-bit machine) no matter
how big — or how recursive — the thing it points at. So the next node lives *elsewhere on the
heap*, and the node itself only has to store a pointer to it:

```rust
struct Node {
    val: i32,
    next: Option<Box<Node>>,   // ✅ compiles: a pointer is a known, fixed size
}
```

Read `Option<Box<Node>>` as the two honest halves of "the rest of the list": either
`Some(box pointing at the next node)`, or `None` meaning *this is the end*. It reuses two things
you already know — [`Option`](../15-option/use-it.md) for "maybe nothing" and the box for "on
the heap" — with no null pointers anywhere.

```rust
let list = Node {
    val: 1,
    next: Some(Box::new(Node {
        val: 2,
        next: None,
    })),
};
println!("{} then {}", list.val, list.next.unwrap().val);   // 1 then 2
```

## You've already been using boxes
Trait objects, back in [Concept 21](../21-trait-objects/use-it.md), stored each value behind a
`Box<dyn Trait>`:

```rust
let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 2.0 }), Box::new(Square { s: 3.0 })];
```

That's the same `Box` doing the same job for a different reason: a `Circle` and a `Square` are
*different sizes*, so they can't share one `Vec` slot directly — but a **pointer** to each is
always the same 8 bytes, so `Vec<Box<dyn Shape>>` lines them up as uniform slots. "Different or
unknown size → put it behind a box, which is always one fixed size" is the thread running
through every use of `Box`.

## When to reach for `Box`
- **A recursive type** — a node that holds a node (lists, trees). This is the one you *can't*
  build without it.
- **A trait object** — `Box<dyn Trait>` for "some type that can do this, decided at runtime."
- **A genuinely large value** you want to pass around by pointer instead of copying the whole
  thing on the stack. (Rare; reach for it only when a value is big.)

If none of those apply, don't box — inline is simpler and faster. `Box` is for when a value
*needs* to be on the heap, not a default wrapper.

> Quick reference: the [`Box<T>` handbook entry](../../../languages/rust.md#box) is the terse
> lookup version, with the linked-list example worked through.

## Exercises
1. **A two-node list** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Define `struct Node { val: i32, next: Option<Box<Node>> }`. Build a list of `10 -> 20 -> 30`
   by hand with `Box::new`, then print the three values by walking `.next`.
2. **Box a value and read it back** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Put an `i32` on the heap with `Box::new`, dereference it with `*` to add `100`, and print the
   result. Then box a small struct and read a field *without* writing `*`, to see
   auto-dereference in action.

## Next
- What a `Box` actually *is* in memory — an 8-byte pointer on the stack aimed at a value on the
  heap — why `Box<i32>` and `Box<[i32; 100]>` are the **same size**, and why moving a `Box` is
  cheap while the value it owns never budges: [Under the hood](under-the-hood.md).
- Then [Interlude 29a — Walking and building a linked list](../29a-walking-a-linked-list/use-it.md):
  now that you can *define* a boxed node, how to *read* a chain you're handed and *build* one node
  at a time — the pattern behind [Add Two Numbers](../../../problems/0002-add-two-numbers/README.md).
- Then [Concept 30 — `Rc<T>`](../30-rc/use-it.md): a `Box` allows exactly **one** owner. When
  several parts of a program need to own the *same* value and you can't say which will finish
  last, `Rc` shares one heap value between many owners by counting them.
