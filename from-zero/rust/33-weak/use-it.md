# Concept 33 · `Weak<T>` (breaking reference cycles) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 32](../32-rc-refcell/use-it.md)

## The idea
[`Rc<T>`](../30-rc/use-it.md) keeps a value alive by **counting owners**: the value is freed only
when the last owner goes away, i.e. when the count hits `0`. That count is also the trap you met at
the end of Concept 32. If two nodes each hold an `Rc` to the other — A owns B, B owns A — then
dropping your own handles leaves each *still* owned by the other. Neither count ever reaches `0`, so
neither is ever freed: a **reference cycle**, which is a memory leak.

`Weak<T>` is the fix. It's an `Rc` handle that **points at the value without owning it** — it does
*not* raise the owner count. So a node can point back at something without keeping it alive, and the
cycle can't form.

> **`Rc<T>` = an owning handle (props the value up). `Weak<T>` = a non-owning handle (just points).**

The rule of thumb you'll reach for again and again:

> **Parent owns child → `Rc`. Child points back at parent → `Weak`.**

The parent keeps the child alive; the child's back-link is only a *view*, so it never keeps the
parent alive and never makes a cycle.

![A parent Rc owns a child with a strong reference, the child points back with a non-owning weak reference, and the two counts decide what gets freed](diagrams/weak-two-counts.svg)

## How to write it
Two moves, and they're mirror images:

- **Make a `Weak` from an `Rc`:** `Rc::downgrade(&some_rc)` → `Weak<T>`. "Downgrade" = drop from
  owning to just-pointing.
- **Use a `Weak`:** `weak.upgrade()` → `Option<Rc<T>>`. "Upgrade" = try to become a real owner
  again. You get `Some(rc)` if the value is **still alive**, or `None` if it's **already been
  dropped**.

```rust
use std::rc::{Rc, Weak};

let strong: Rc<i32> = Rc::new(42);      // an owning handle — strong count 1
let weak: Weak<i32> = Rc::downgrade(&strong); // a non-owning handle — strong count STILL 1

// To read through the Weak, upgrade it. It hands back Some while the value lives.
if let Some(alive) = weak.upgrade() {
    println!("still here: {}", alive);  // still here: 42
}

drop(strong);                            // the last OWNER is gone → value dropped

// Now upgrade fails — safely, with None instead of a dangling pointer.
println!("{:?}", weak.upgrade());        // None
```

The `Weak` never counted as an owner, so dropping `strong` really did free the value. And because
you can only touch the value *through* `upgrade()`, Rust forces you to handle the "it's gone now"
case — a `Weak` can never hand you a dangling pointer.

## Why `upgrade()` returns an `Option`
This is the whole safety story in one line. A `Weak` doesn't keep its value alive, so between making
the `Weak` and using it, the value might have been dropped. Rust won't let you read possibly-freed
memory, so `upgrade()` **checks at that moment** and returns:

- `Some(Rc<T>)` — the value is still there; you now hold a real (temporary) owner and can read it;
- `None` — every owner is gone and the value is freed; there's simply nothing to hand you.

That `Option` is the same "no more null" idea from [Concept 15](../15-option/use-it.md), doing a
job only it can: turning "this pointer might be dead" into a case you *must* handle, instead of a
crash.

## The classic use: a child that knows its parent
A tree where each child can look back up at its parent is the textbook cycle. Owning both directions
would leak. So: the parent **owns** its children with `Rc`, and each child holds a **`Weak`** back up
to the parent.

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,   // Weak: does NOT keep the parent alive
    children: RefCell<Vec<Rc<Node>>>, // Rc: the parent owns its children
}

let parent = Rc::new(Node {
    value: 1,
    parent: RefCell::new(Weak::new()),   // no parent yet
    children: RefCell::new(vec![]),
});

let child = Rc::new(Node {
    value: 2,
    parent: RefCell::new(Rc::downgrade(&parent)), // point back, don't own
    children: RefCell::new(vec![]),
});

parent.children.borrow_mut().push(Rc::clone(&child)); // parent owns child

// Walk up from the child — upgrade the Weak to reach the parent.
let up = child.parent.borrow().upgrade();
println!("{}", up.unwrap().value); // 1
```

The down-links are `Rc` (ownership flows parent → child); the up-link is `Weak` (a view, child → 
parent). Drop your handle to `parent` and it frees cleanly, because the only thing pointing *down*
at it from the child is a non-owning `Weak`.

## `Weak::new()` — an empty handle
`Weak::new()` makes a `Weak` that points at nothing at all; its `upgrade()` is always `None`. It's
the natural "no parent yet" placeholder (the root of a tree has no parent), which is why the root's
`parent` field above starts as `Weak::new()`.

## When to reach for it
- **The back-link in a two-way structure** — child→parent, node→graph-owner, observer→subject.
  Whichever direction *doesn't* own, make it `Weak`.
- **A cache or observer that shouldn't keep its target alive** — you want to reach the value *if* it
  still exists, but not be the reason it sticks around.
- **Not as a default pointer.** If nothing points in a cycle, plain [`Rc`](../30-rc/use-it.md) (or
  no `Rc` at all) is simpler. Reach for `Weak` specifically to break an ownership loop or to hold a
  deliberately non-keeping reference.

> Quick reference: the [`Rc<T>` handbook entry](../../../languages/rust.md#rc) covers owning
> handles; [`Weak<T>`](../../../languages/rust.md#weak) covers the non-owning half and `upgrade`.

## Exercises
1. **Downgrade, then outlive** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Make an `Rc<String>`, `Rc::downgrade` it into a `Weak`. `upgrade()` it and print the value while
   the `Rc` is alive. Then `drop` the `Rc` and `upgrade()` again — print the `Option` to see it is
   now `None`.
2. **A child that points back at its parent** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Build the two-node `Node` struct above: a `parent` owning one `child` with `Rc`, the `child`
   holding a `Weak` back up. From the child, `upgrade()` the weak link and print the parent's
   `value`. Confirm `Rc::strong_count(&parent)` is still `1` — the child's `Weak` did not bump it.

## Next
- What a `Weak` looks like **in memory** — the second counter (the *weak count*) living beside the
  strong count, why the value is freed when **strong** hits 0 but the allocation lingers until
  **both** do, and exactly how that breaks the Concept 32 leak: [Under the hood](under-the-hood.md).
- That closes the smart-pointer arc (Phase 8). Next phase, values start living on **more than one
  stack at once** — the first step into concurrency with **threads**.
