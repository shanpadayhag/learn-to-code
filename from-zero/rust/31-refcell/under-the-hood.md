# Concept 31 · `RefCell<T>` (mutate through a shared reference) — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## A value plus a tiny borrow-flag
A `RefCell<T>` is barely more than the `T` it wraps. Sitting next to your value is one small
integer — the **borrow flag** — that records what's borrowed *right now*:

- `0` → nobody is borrowing (free).
- a positive number → that many **shared reads** are out (`.borrow()` handles).
- a special "written" state → exactly **one write** is out (`.borrow_mut()` handle).

That flag is the whole mechanism. The compile-time borrow checker keeps this same bookkeeping *in
its head* while compiling and then erases it; `RefCell` keeps it as a real number that lives in
memory and is consulted while the program runs.

![RefCell stores the value alongside a borrow-flag counter that borrow() and borrow_mut() consult](diagrams/refcell-runtime-check.svg)

## What `.borrow()` and `.borrow_mut()` actually do
Each call is a tiny runtime check-and-update against that flag:

- **`.borrow()`** — if the flag is "written," **panic** (`already mutably borrowed`); otherwise add
  one to the read count and hand back a `Ref<T>`.
- **`.borrow_mut()`** — if the flag is anything but `0`, **panic** (`already borrowed`); otherwise
  set it to "written" and hand back a `RefMut<T>`.

The returned `Ref<T>` / `RefMut<T>` are smart-pointer **guards**: you use them like a `&`/`&mut`
(via `*`), and when the guard goes out of scope its [`Drop`](../08-ownership-and-moves/under-the-hood.md)
runs and **restores the flag** — a read guard subtracts one, a write guard returns to `0`. This is
why "keep borrows short" works: the borrow lasts exactly as long as the guard is alive, so ending
its statement or scope frees the cell for the next borrow.

```rust
use std::cell::RefCell;

let cell = RefCell::new(String::from("hi"));
{
    let mut w = cell.borrow_mut();   // flag → written
    w.push_str("!");
}                                     // w dropped → flag back to 0
println!("{}", cell.borrow());        // flag → 1 read, printed, then back to 0
```

## The rules are identical — only the *timing* moved
It's worth seeing the two side by side, because `RefCell` isn't a *new* rule, it's the same rule
enforced later:

| | `&` / `&mut` (Concepts 10–11) | `RefCell` (`.borrow()` / `.borrow_mut()`) |
|---|---|---|
| The rule | many readers **XOR** one writer | *same* — many readers XOR one writer |
| Checked | at **compile time** | at **run time** |
| A violation is | a compile **error** (won't build) | a **panic** (crashes at runtime) |
| Runtime cost | none (checks erased) | a small counter check per borrow |
| Mutate through `&`? | no | **yes** (interior mutability) |

The takeaway: `RefCell` doesn't make anything *legal* that `&mut` forbids — a data race is still
impossible. It just lets you defer the proof to runtime, for patterns the compiler can't verify
ahead of time. You pay with a tiny per-borrow check and the risk of a panic if you're wrong.

## Cost and size
`RefCell<T>` is the size of `T` plus that one small flag (a word). There's no heap allocation of
its own — a `RefCell<i32>` lives inline on the stack just like an `i32`, only slightly larger. The
runtime cost is a couple of integer comparisons per `.borrow()` / `.borrow_mut()` — cheap, but not
*free* the way a compile-time `&` is. That's the honest price of the flexibility.

`RefCell` is also **single-threaded**, for the same reason as [`Rc`](../30-rc/under-the-hood.md):
its flag isn't safe to update from two threads at once. The thread-safe siblings are `Mutex<T>` and
`RwLock<T>`, which you'll meet in the concurrency phase.

## Predict the behavior
```rust
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(10);

    let a = cell.borrow();          // (1)
    let b = cell.borrow();          // (2)
    println!("{} {}", a, b);
    drop(a);
    drop(b);                        // (3)

    *cell.borrow_mut() += 5;        // (4)
    println!("{}", cell.borrow());  // (5)
}
```

1. `a = cell.borrow()` — what does the flag become, and is this allowed?
2. `b = cell.borrow()` — a *second* shared read while `a` is still alive. Allowed or panic?
3. After both `drop`s — what's the flag now?
4. `cell.borrow_mut()` — does this succeed here?
5. The final read — what prints?

<details>
<summary>Show the answer</summary>
<ol>
<li><strong>Allowed.</strong> The flag goes to "1 read." A shared read is always fine when nothing is written.</li>
<li><strong>Allowed — flag "2 reads."</strong> Many shared reads at once is exactly what the rules permit; <code>a</code> and <code>b</code> coexist happily, so it prints <code>10 10</code>.</li>
<li><strong>Flag back to <code>0</code>.</strong> Each <code>drop</code> restores the flag; after both, nobody is borrowing.</li>
<li><strong>Succeeds.</strong> Because the flag is <code>0</code>, <code>.borrow_mut()</code> can take the exclusive write handle, set the flag to "written," add <code>5</code>, and drop the handle at the end of the statement (flag back to <code>0</code>). If either <code>a</code> or <code>b</code> had <em>still</em> been alive here, this line would have <strong>panicked</strong>.</li>
<li><strong><code>15</code>.</strong> The final <code>.borrow()</code> reads the updated value.</li>
</ol>
</details>

## Next
- **[Concept 32 — `Rc<RefCell<T>>`](../32-rc-refcell/use-it.md):** put this lesson's "mutate through
  a shared `&`" *inside* Concept 30's "many owners." `Rc` shares the ownership; `RefCell` supplies
  the mutation the `Rc` alone couldn't. Together they're Rust's standard shape for a value that many
  parts of a program both **own** and **change**. Next lesson.
