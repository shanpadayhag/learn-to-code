# Rust syntax handbook

A growing reference of Rust syntax, built from the solutions in this repo. Each
entry is explained once here and linked from wherever it's used. Assumes you know
how to program in *some* language — just not Rust yet.

## Contents
- [`use` declarations](#use)
- [`impl` blocks](#impl)
- [`pub fn` — functions](#pub-fn)
- [`Vec<T>` — growable arrays](#vec-type)
- [`vec![]` — the vector macro](#vec-macro)
- [`let` and `let mut`](#let-mut)
- [shadowing — reusing a name](#shadowing)
- [`HashMap<K, V>`](#hashmap)
- [`for` + `.iter()` + `.enumerate()`](#for-iter-enumerate)
- [`&` in patterns — destructuring a reference](#ref-pattern)
- [`*` — dereference (follow a reference)](#deref)
- [`if let` with `Option` / `Some`](#if-let)
- [`as` — numeric casts](#as-cast)
- [`Box<T>` — a pointer to the heap](#box)
- [`Rc<T>` — shared ownership](#rc)
- [`Weak<T>` — a non-owning `Rc` handle](#weak)
- [`RefCell<T>` — mutate through a shared reference](#refcell)
- [`&mut` — mutable references](#mut-ref)
- [`while` loops](#while)
- [`for` + ranges — counting loops (`..`, `..=`, `.rev()`)](#for-ranges)
- [`Option::is_some`](#is-some)
- [`Option::take`](#option-take)
- [`Option::as_mut`](#option-as-mut)
- [`.unwrap()`](#unwrap)
- [`String` — an owned string](#string)
- [`.chars()` — iterate a string by character](#chars)
- [`.rev()` — reverse an iterator](#rev)
- [`usize` and underflow](#usize)
- [integer division — `/` truncates](#int-division)
- [`%` — the remainder operator](#remainder)
- [`.max()` / `.min()` — pick the larger or smaller](#ord-max)
- [`struct` — defining your own type](#struct)
- [`enum` — one of several shapes](#enum)
- [`type` — type aliases](#type-alias)
- [`#[derive(Default)]` and `Self::default()`](#derive-default)
- [`match` expressions](#match)
- [`BTreeMap<K, V>` — a sorted map](#btreemap)
- [`.range(...)` on a `BTreeMap`](#btreemap-range)
- [`Option<T>` — a value that may be absent](#option)
- [`?` — the question-mark operator](#question-mark)
- [`let ... else` — bind or bail](#let-else)
- [`.entry(...).or_default()`](#entry-or-default)
- [closures — `|x| ...`](#closures)
- [iterator adapters — `.filter()`, `.map()`, `.collect()`, …](#iterator-adapters)
- [the `Iterator` trait — `.next()` and building your own](#iterator-trait)
- [`.bytes()` — iterate a string's raw bytes](#bytes)
- [`.zip()` — pair two iterators](#zip)
- [`.take()` — at most `n` items](#take)
- [`.copied()` — turn `&T` items into owned `T`](#copied)
- [`.count()` — consume and count](#count)
- [`impl Trait` in argument position](#impl-trait-arg)
- [lifetimes — `<'a>`](#lifetimes)
- [`format!` — building strings](#format)
- [`{:.1}` — format specifiers (decimals, width, …)](#format-spec)
- [`.to_owned()` / `.clone()` — making an owned copy](#to-owned-clone)
- [`Copy` types — values duplicated on assignment](#copy)
- [`&T` — shared references (borrowing)](#borrow)
- [slices — `&str` and `&[T]`](#slice)
- [generics — `<T>`](#generics)
- [string indexing — why `s[i]` is forbidden](#string-indexing)
- [`.trim()` — a borrowed slice without the outer whitespace](#trim)
- [`trait` — defining and implementing shared behaviour](#trait)
- [`thread::spawn` — run code on a new thread](#thread-spawn)
- [`Arc<T>` — shared ownership across threads](#arc)
- [`Mutex<T>` — one thread at a time](#mutex)
- [`mpsc::channel` — send values between threads](#mpsc-channel)
- [`Send` and `Sync` — what may cross a thread](#send-sync)
- [`async` / `.await` — a function that can pause](#async-await)
- [`Future`, `poll`, and executors](#future-poll)
- [`unsafe` — the door out of the rules](#unsafe)
- [raw pointers — `*const T` / `*mut T`](#raw-pointers)
- [`mod` — modules, paths and privacy](#modules)
- [`fn main` — the program's entry point](#main)
- [unit structs — `struct Solution;`](#unit-struct)
- [`println!` — print a line](#println)
- [`assert_eq!` — a check that stops the program](#assert-eq)
- [`while let` — loop while a pattern still matches](#while-let)
- [`.fold()` — collapse an iterator into one value](#fold)
- [`where` clauses — bounds moved below the signature](#where)

## `use` declarations {#use}

**In one line:** pulls a name into scope so you can write `HashMap` instead of the
full `std::collections::HashMap` every time.

Rust's standard library is organized into modules with `::` as the separator
(like folders). `use std::collections::HashMap;` says "for the rest of this file,
`HashMap` means that one." Without it the code still works — you'd just have to
type the whole path at each use. Put `use` lines at the top of the file.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `impl` blocks {#impl}

**In one line:** the place where you attach functions to a type.

`impl Solution { ... }` means "here are functions that belong to the `Solution`
type." LeetCode's Rust judge defines an empty `Solution` struct for you and calls
your method on it, which is why the harness code is wrapped in `impl Solution`.
For everyday Rust, `impl` is how you give a struct its methods.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `pub fn` — functions {#pub-fn}

**In one line:** `fn` declares a function; `pub` makes it visible outside its
module.

`pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>` reads as: a public
function named `two_sum`, taking a `Vec<i32>` and an `i32`, returning a `Vec<i32>`.
Parameter types come *after* the name with a colon; the return type follows `->`.
A function with no `->` returns nothing (the unit type `()`). The last expression
in the body — with no semicolon — is the return value, so explicit `return` is
only needed for early exits.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `Vec<T>` — growable arrays {#vec-type}

**In one line:** a resizable list of values that all share one type `T`.

`Vec<i32>` is a vector of 32-bit signed integers. It's Rust's everyday "array that
can grow", living on the heap. The `<i32>` is a *type parameter* — `Vec<String>`,
`Vec<bool>`, etc. all work the same way. You index with `v[0]` and read its length
with `v.len()`.

Taught from zero — the ptr/len/cap header, doubling growth (amortized O(1)), and why a
regrow breaks old borrows — in [From-Zero Concept 17](../from-zero/rust/17-vec/use-it.md).

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `vec![]` — the vector macro {#vec-macro}

**In one line:** a shorthand for building a `Vec` from listed values.

`vec![earlier_index, current_index]` creates a two-element vector; `vec![]` creates
an empty one. The `!` marks it as a *macro* (code that expands into more code at
compile time), not a function — that's how it can take any number of arguments and
figure out the element type from them.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `let` and `let mut` {#let-mut}

**In one line:** `let` declares a variable; by default it can't be reassigned, and
`mut` is what makes it changeable.

In Rust, variables are *immutable* unless you opt out. `let x = 5;` binds `x` once
and forever. `let mut x = 5;` lets you later do `x = 6;` or call methods that
mutate it. This default-immutable rule is a safety feature: the compiler stops you
from changing things you didn't mean to. We use `let mut index_of_seen_value`
because we insert into the map as we scan.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `HashMap<K, V>` {#hashmap}

**In one line:** Rust's key → value store, where keys are type `K` and values type
`V`.

This is Rust's spelling of the [hash map](../glossary/hash-map.md) concept.
`HashMap<i32, i32>` maps integer keys to integer values. Create one with
`HashMap::new()` (needs `use std::collections::HashMap;` first). Key methods:
`.insert(key, value)` to store, and `.get(&key)` to look up — note `.get` takes a
*reference* to the key and returns an [`Option`](#if-let) (`Some(value)` if present,
`None` if not), because the key might be missing.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `HashSet<T>` — a set of unique values {#hashset}

**In one line:** a collection that answers "is X in here?" in ~O(1) and never keeps duplicates —
a [`HashMap`](#hashmap) storing only keys (`HashSet<T>` is `HashMap<T, ()>`).

Needs `use std::collections::HashSet;`. Build with `HashSet::new()` or `HashSet::from([a, b, c])`.
Key methods:

- `.insert(x)` — adds `x`; **returns a `bool`**: `true` if it was new, `false` if already present.
  That bool is a ready-made "have I seen this?" check.
- `.contains(&x)` — takes a *reference*, returns `bool` (only looks, doesn't take ownership).
- `.len()` — how many *distinct* values are in the set (duplicates were never stored).

```rust
let mut seen = HashSet::new();
seen.insert(7);
let repeat = !seen.insert(7);       // true — 7 was already there
let unique_count = seen.len();      // 1
```

A set is **unordered** (values live wherever their hash sends them) and holds **no duplicates**. Pick
it over a [`Vec`](#slice)'s `.contains()` when you need fast membership or automatic dedup — the Vec
scans (O(n)), the set hashes to one slot (~O(1)). Need sorted values instead? Use `BTreeSet` (built on
[`BTreeMap`](#btreemap)).

First seen in: [From-Zero Concept 22](../from-zero/rust/22-hashset/use-it.md)

## `for` + `.iter()` + `.enumerate()` {#for-iter-enumerate}

**In one line:** loop over a collection, getting both each item and its position.

`for (i, &v) in numbers.iter().enumerate()` breaks down as:
- `.iter()` produces a sequence of *references* to the elements (it borrows the
  vector rather than consuming it, so `numbers` is still usable afterward).
- `.enumerate()` wraps each item into a `(index, item)` pair, counting from 0.
- `for (i, v) in ...` destructures that pair into two names each loop.

The result: `i` walks `0, 1, 2, ...` while `v` walks the values. (For why it's
`&v` and not `v`, see [`&` in patterns](#ref-pattern).)

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `&` in patterns — destructuring a reference {#ref-pattern}

**In one line:** an `&` on the *left* of `=` unwraps a reference, giving you the
value by copy instead of a pointer to it.

`.iter()` yields `&i32` (references to integers), but arithmetic like
`target - current_value` is cleaner on a plain `i32`. Writing the loop variable as
`&current_value` says "the thing I'm receiving is a reference — peel it off and
bind the value underneath." Since `i32` is cheaply copyable, this just copies the
number out. The same move appears in `if let Some(&earlier_index) = ...`, peeling
the reference that `.get()` hands back. Without the `&`, you'd be holding a `&i32`
and would have to [dereference with `*`](#deref) everywhere you used it.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `*` — dereference (follow a reference) {#deref}

**In one line:** `*r` follows a reference to the value it points at — the exact inverse
of `&`, which makes a reference.

**The pair.** `&` and `*` are opposites: `&x` *borrows* `x` and yields a **reference**
(`i32` → `&i32`); `*r` *dereferences* and yields the **value** back (`&i32` → `i32`).
Under the hood a reference is just an address, and `*` is "go to that address and read
it" (see [From-Zero 10a — Under the hood](../from-zero/rust/10a-dereferencing-with-star/under-the-hood.md)).

**When you need it.** Whenever you hold a `&T` but the surrounding code wants an owned
`T`. Classic case — putting values fetched from a map into a `Vec`:

```rust
if let Some(difference_index) = number_bank.get(&needed) {  // &usize
    return vec![*difference_index as i32, index as i32];    // * → usize
}
```

Without the `*`, the types mismatch: `vec![]` wants numbers, not references.

**When you don't.** Rust auto-dereferences in two everyday spots, which is why references
sometimes "just work" with no `*`:
- **method calls**: `r.to_string()` works on a `&i32` — Rust derefs to find the method.
- **formatting**: `println!("{}", r)` prints the value, not an address.

Rule of thumb: if the compiler says it *expected `T`, found `&T`*, add a `*`.

**The pattern alternative.** [`&` in a pattern](#ref-pattern) does the same peel at the
binding site instead — `if let Some(&x) = ...` gives you the value directly, no later
`*`.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md) · taught in [From-Zero 10a](../from-zero/rust/10a-dereferencing-with-star/use-it.md)

## `if let` with `Option` / `Some` {#if-let}

**In one line:** run a block only when a value is present, and pull that value out
in the same line.

Rust has no `null`. A value that might be absent has type `Option<T>`, which is
either `Some(value)` or `None`. `.get()` on a map returns one of these.
`if let Some(&earlier_index) = index_of_seen_value.get(&needed_value) { ... }` means
"if the lookup returned `Some`, bind what's inside to `earlier_index` and run the
block; otherwise skip it." It's the concise alternative to a full `match` when you
only care about one case.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `as` — numeric casts {#as-cast}

**In one line:** converts a number from one type to another, explicitly.

Rust never silently mixes number types, so `current_index` (a `usize`, the type
used for indexes and lengths) must be converted before it can sit in a `Vec<i32>`.
`current_index as i32` performs that conversion. `as` is the blunt cast for
primitives — quick and direct, though for conversions that could lose data Rust
also offers safer checked options elsewhere.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `Box<T>` — a pointer to the heap {#box}

**In one line:** a pointer that owns one value stored on the heap, used when a type
needs to point at "more of itself."

**What it is.** Most values live *inline* — right where they're declared. `Box<T>`
instead puts the `T` on the heap (the program's big pool of long-lived memory) and
keeps just a pointer to it. The `Box` owns that value: when the `Box` goes away, the
heap value is freed automatically.

**Why a [linked list](../glossary/linked-list.md) can't live without it.** A node
holds the *next* node. Picture writing it without a box:
```rust
struct ListNode { val: i32, next: Option<ListNode> }   // ❌ does not compile
```
To lay this out, the compiler must know a node's size. But a node contains a node,
which contains a node... forever — an infinitely large type. The error literally says
*"recursive type has infinite size."* A `Box` breaks the cycle: a pointer is a fixed,
known size no matter how big the thing it points to, so the next node lives elsewhere
on the heap and the node's size is finally knowable:
```rust
struct ListNode { val: i32, next: Option<Box<ListNode>> }   // ✅
```

**Trace the types.** `Box::new(ListNode::new(0))` allocates a `ListNode` on the heap
and hands back a `Box<ListNode>` that owns it. You rarely write `*` to reach inside —
field access like `node.val` and `result_tail.next` *auto-dereferences* through the
box for you.

**Why this way.** It's the smallest possible indirection that makes a self-referential
type have a finite size, while still owning its contents (no manual free, no garbage
collector). Pairing it with [`Option`](#if-let) — `Option<Box<ListNode>>` — gives the
two halves of a linked list: "a pointer to the next node" *or* "nothing, this is the
end," with no null pointers involved.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Rc<T>` — shared ownership {#rc}

**In one line:** a reference-counted pointer that lets **many** owners share one heap
value, freeing it only when the last owner goes away.

**What it is.** A [`Box<T>`](#box) allows exactly one owner. `Rc<T>` (*reference
counted*) relaxes that: it stores the value on the heap next to a **count** of how many
owners point at it. Every new owner bumps the count; every owner dropped lowers it; when
it reaches zero the value is freed. Single-threaded only — the cross-thread sibling is
`Arc<T>`.

```rust
use std::rc::Rc;
let a = Rc::new(String::from("hi")); // count 1
let b = Rc::clone(&a);               // count 2 — same allocation, NOT a copy
println!("{}", Rc::strong_count(&a)); // 2
```

**`Rc::clone` vs `.clone()`.** `Rc::clone(&x)` copies only the pointer and adds `1` to
the count — cheap and fixed-cost, unlike [`String::clone`](#string) which duplicates the
whole buffer. It's written in the explicit `Rc::clone(&x)` form on purpose, to flag "cheap
count bump, not a deep copy."

**Trace the types.** `Rc::new(v)` gives `Rc<T>` (the first owner). `Rc::clone(&x)` takes
`&Rc<T>` and gives another `Rc<T>` aimed at the same heap block. `Rc::strong_count(&x)`
reads the current owner count as a `usize`.

**Why this way.** Use it when a value has no single obvious owner — several parts of a
structure share one node and it must live until the last of them is done. The trade: `Rc`
hands out **read-only** shared access (mutating a value with many aliases would break the
[borrow rules](#mut-ref)); pair it with `RefCell<T>` as `Rc<RefCell<T>>` when the shared
value also needs to change.

First seen in: [From-Zero concept 30 — `Rc<T>`](../from-zero/rust/30-rc/use-it.md)

## `Weak<T>` — a non-owning `Rc` handle {#weak}

**In one line:** a handle that *points at* an [`Rc`](#rc) value without owning it, so it never
keeps the value alive — the tool for breaking reference cycles.

**What it is.** An `Rc` allocation carries **two** counts: a **strong** count (owning handles) and
a **weak** count (non-owning handles). The value is dropped when **strong** hits `0`; the allocation
itself is freed when **both** hit `0`. A `Weak<T>` bumps only the weak count, so it has no say over
the value's lifetime.

```rust
use std::rc::{Rc, Weak};
let strong = Rc::new(42);
let weak: Weak<i32> = Rc::downgrade(&strong); // strong count still 1
println!("{:?}", weak.upgrade().map(|r| *r)); // Some(42)
drop(strong);
println!("{:?}", weak.upgrade());              // None — freed, safely
```

**`downgrade` / `upgrade`.** `Rc::downgrade(&rc)` turns an owning `Rc<T>` into a non-owning
`Weak<T>`. To read through it you must `weak.upgrade()`, which returns `Option<Rc<T>>`: `Some(rc)`
(a fresh temporary owner) while the value lives, `None` once every strong owner is gone. That
[`Option`](#option) is the guard rail — a `Weak` can never hand you a dangling pointer.
`Weak::new()` makes an empty handle whose `upgrade()` is always `None` (a "nothing here yet"
placeholder).

**Why this way.** In a two-way structure — child↔parent, node↔graph — making *both* directions
`Rc` creates a cycle whose counts never reach `0`, i.e. a leak. Fix it by the rule
**parent-owns-child uses `Rc`, child-points-back uses `Weak`**: the back-link points home without
propping the count up, so the loop can always come apart. Reach for `Weak` specifically to break an
ownership loop or to hold a deliberately non-keeping reference (a cache/observer that shouldn't keep
its target alive).

First seen in: [From-Zero concept 33 — `Weak<T>`](../from-zero/rust/33-weak/use-it.md)

## `RefCell<T>` — mutate through a shared reference {#refcell}

**In one line:** a wrapper that moves the borrow check from compile time to run time,
letting you mutate a value through a shared `&` — and **panicking** if you break the
borrow rules.

**What it is.** [`&mut`](#mut-ref) enforces "many readers XOR one writer" at compile time,
for free. `RefCell<T>` enforces the *same* rule at **run time** instead, using a small
**borrow flag** stored next to the value. This lets you change a value you only hold a
shared `&` to — called **interior mutability** — for patterns the compiler can't prove
safe ahead of time.

```rust
use std::cell::RefCell;
let cell = RefCell::new(5);   // no `mut` needed
*cell.borrow_mut() += 10;     // write handle → change through a shared &
println!("{}", cell.borrow()); // 15  (read handle)
```

**`.borrow()` / `.borrow_mut()`.** `.borrow()` returns a `Ref<T>` (a shared read; many at
once); `.borrow_mut()` returns a `RefMut<T>` (one exclusive write). You reach the value
through the handle with [`*`](#deref); the handle restores the flag when it drops, so
**keep borrows short**.

**The trade.** Break the rule — e.g. two live `.borrow_mut()` at once — and it compiles
but **panics** (`already borrowed: BorrowMutError`). You swapped a compile-time guarantee
for a runtime one, plus a tiny per-borrow check. Prefer plain `&mut`/`let mut` when it
already compiles; reach for `RefCell` when it doesn't but you can prove the access is safe.
Single-threaded only (the cross-thread siblings are `Mutex`/`RwLock`); most often seen
paired with [`Rc`](#rc) as `Rc<RefCell<T>>` for shared **mutable** state.

First seen in: [From-Zero concept 31 — `RefCell<T>`](../from-zero/rust/31-refcell/use-it.md)

## `&mut` — mutable references {#mut-ref}

**In one line:** a borrow you're allowed to *change* the value through — the
read-write counterpart to the read-only [`&`](#borrow).

**The two kinds of borrow.** Two Sum used `&`, a *shared* reference: you can look but
not touch. `&mut` is an *exclusive* reference: you can modify the value in place,
without taking ownership of it. The rule Rust enforces: at any moment a value can have
**many** `&` readers **or exactly one** `&mut` writer — never both at once. That
single-writer guarantee is what makes data races impossible.

**Here.** `let mut result_tail = &mut result_head;` borrows the head node mutably, so
we can grow the list *through* the borrow — `result_tail.next = Some(...)` writes into
the real node, not a copy. Later `result_tail = result_tail.next.as_mut().unwrap();`
re-points the borrow at the freshly added node so the next write lands at the new end.

**Without it.** With a plain `&result_head` the line `result_tail.next = Some(...)`
won't compile — you can't assign through a read-only borrow. The alternative would be
to pass *ownership* of the list around and hand it back each step, which is far
clumsier than borrowing it once and writing through the borrow.

**One confusing overlap.** `mut` shows up in two different roles. In `let mut x`, the
`mut` makes the *binding* reassignable (`x = ...` later). In `&mut x`, the `mut` makes
a *reference through which you can mutate* the pointed-to value. `let mut result_tail =
&mut result_head;` uses both: the binding is reassignable (we re-point it each loop)
*and* it's a mutable reference (we write through it).

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)
· the borrow rules taught in [From-Zero Concept 11](../from-zero/rust/11-mut-references-and-borrow-rules/use-it.md)

## `while` loops {#while}

**In one line:** repeat a block as long as a condition stays true.

`while condition { ... }` checks `condition` before each pass and stops the moment it's
false. Two Sum used a `for` loop because it walked a known sequence to its end. Here we
use `while` because we stop on a *dynamic* condition — "both lists are used up **and**
no carry is left" — not a fixed number of steps:
```rust
while first_digit.is_some() || second_digit.is_some() || carry != 0 { ... }
```

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `break` · `continue` · labeled loops {#loop-control}

**In one line:** `break` stops a loop, `continue` skips to its next turn — and both act on
the **nearest** loop only, so a *label* (`'name:`) is how you target an outer one.

```rust
'search: for row in grid {
    for cell in row {
        if cell == target { break 'search; }   // leaves BOTH loops
    }
}
```
- `break` / `continue` with no label affect the innermost enclosing loop. Nesting two loops
  and expecting an inner `break` to stop the outer one is a classic bug — the outer loop
  keeps running.
- Prefix a loop with `'name:` to label it, then `break 'name` / `continue 'name` act on
  *that* loop from any depth inside. The leading `'` is the same tick used for
  [lifetimes](#lifetimes); here it just names a loop.
- `loop { … }` (infinite until `break`) can also `break value` to return a value.

Taught from zero — the nearest-loop trap (from a real nested-loop bug) and labels as the fix
— in [From-Zero Interlude 05b](../from-zero/rust/05b-break-continue-and-labels/use-it.md).

First seen in: taught from zero in [From-Zero Interlude 05b](../from-zero/rust/05b-break-continue-and-labels/use-it.md)

## `for` + ranges — counting loops (`..`, `..=`, `.rev()`) {#for-ranges}

**In one line:** `for i in a..b { … }` runs the block once for each number in the range;
`..` excludes the end, `..=` includes it.

A **range** is a value standing for a run of numbers, and a `for` loop walks it:

```rust
for i in 1..=5 { print!("{i} "); }   // 1 2 3 4 5
```

**The `..` vs `..=` distinction** (the usual off-by-one bug):
- `a..b` — **exclusive**: `0..3` is `0, 1, 2`. `0..n` yields exactly `n` values — ideal for
  "n times" and for list positions.
- `a..=b` — **inclusive**: `1..=3` is `1, 2, 3`. Use it when you want the end (`1..=10`).

**The ends are just values** — variables and expressions work: `low..=high`,
`0..items.len()`.

**Counting down.** A range only counts up; `10..=1` is *empty*, not reversed. Reverse an
upward range instead: `for i in (1..=10).rev()` → `10, 9, … 1` (parentheses required).

**Ranges carry math.** A range can fold itself: `(1..=100).sum::<i32>()` is `5050`;
`(1..=n).product()` is `n!`. (These are iterator methods; a range is iterable. Full
iterator story comes later.)

**`for` vs [`while`](#while).** Use `for` when you know the sequence/count; use `while`
when you only have a stopping condition ("keep going until…"). The same `..` also appears
in [slicing](#slice) (`&s[0..2]`) — one "start to end" idea across loops, slices, and sums.

First seen in: [From-Zero Interlude 05a](../from-zero/rust/05a-loops-and-ranges/use-it.md)

## `Option::is_some` {#is-some}

**In one line:** asks an [`Option`](#if-let) "are you holding a value?" and answers
`true` or `false`.

`first_digit.is_some()` is `true` when `first_digit` is `Some(...)` and `false` when
it's `None`. It only *peeks* — it doesn't take the value out — which is exactly what a
loop condition wants: we check whether digits remain without disturbing them. (Its
mirror image is `.is_none()`.)

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::take` {#option-take}

**In one line:** rips the value out of an `Option`, leaves `None` in its place, and
hands you what was there.

**The problem it solves.** `first_digit` is an owned `Option<Box<ListNode>>`, and we
want the node inside to read its digit and step to `.next`. The obvious
`if let Some(node) = first_digit` *moves* `first_digit` into the match — and Rust then
considers `first_digit` used-up for the rest of the loop, so the next iteration's
`first_digit.is_some()` won't compile. We need the inside *without* destroying the
variable.

**What `.take()` does.** It swaps the slot to `None` and returns the old contents,
working through a `&mut`:
- before: `first_digit` is `Some(box)`
- `first_digit.take()` returns `Some(box)` **and** sets `first_digit` to `None`
- we match the returned `Some(node)`, then immediately overwrite the now-`None`
  `first_digit` with `node.next`

So the variable is always left in a valid state. If the list was already empty,
`.take()` returns `None`, the `if let` simply doesn't fire, and `first_digit` stays
`None` — precisely the "treat a missing digit as nothing" behavior we want.

**Without it.** You'd reach for `std::mem::replace(&mut first_digit, None)` by hand —
which is exactly what `.take()` is shorthand for.

Taught from zero — `.take()` alongside `.as_ref()` / `.as_mut()` as the three ways to reach
inside an `Option` without consuming it — in
[From-Zero Interlude 15b](../from-zero/rust/15b-taking-and-borrowing-inside-option/use-it.md).

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::as_mut` {#option-as-mut}

**In one line:** turns a `&mut Option<T>` into an `Option<&mut T>` — lets you reach a
mutable pointer to the value *inside* without removing it.

After `result_tail.next = Some(Box::new(...))`, we want to advance the tail to that
brand-new node. `.take()` would be wrong here — it would yank the node back out, the
opposite of what we want. `.as_mut()` instead borrows into the `Option`:
- `result_tail.next` is an `Option<Box<ListNode>>`
- `.as_mut()` gives `Option<&mut Box<ListNode>>` — a mutable peek, value left in place
- [`.unwrap()`](#unwrap) pulls out the `&mut Box<ListNode>` we re-point the tail to

So: `.take()` when you mean to *remove* the value, `.as_mut()` when you mean to *keep
it and borrow it*.

Taught from zero — pushing the borrow *inside* the Option — in
[From-Zero Interlude 15b](../from-zero/rust/15b-taking-and-borrowing-inside-option/use-it.md);
applied to the tail-cursor list build in
[From-Zero Interlude 29a](../from-zero/rust/29a-walking-a-linked-list/use-it.md).

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `.unwrap()` {#unwrap}

**In one line:** pulls the value out of a `Some` (or an `Ok`), and crashes the program
if it's `None` instead.

`.unwrap()` is the blunt way to get inside an [`Option`](#if-let): on `Some(x)` it
returns `x`, on `None` it panics. That makes it risky in general — a `None` you didn't
expect takes the whole program down. It's safe *here* only because the line right above
just set `result_tail.next = Some(...)`, so the value is provably present; we use
`.unwrap()` to say "I know this is `Some`." When you *can't* prove that, reach for
[`if let`](#if-let) or a `match`, which handle the `None` case instead of exploding.

Taught from zero — `.unwrap()` as a crash-on-`None` bet, plus opening two Options at once
with a tuple-match or [`Option::zip`](#option) — in
[From-Zero Interlude 15a](../from-zero/rust/15a-opening-options-safely/use-it.md).

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `String` — an owned string {#string}

**In one line:** a growable, owned piece of text — the string type you get handed
when a function *owns* its text input.

Rust has two main string types, and the split trips up newcomers:
- `String` — owns its text on the heap, can grow and shrink. This is what
  `length_of_longest_substring(text: String)` receives: the function takes ownership
  of the whole string.
- `&str` — a *borrowed view* into text someone else owns (a "string slice"). A
  literal like `"abc"` is a `&str`.

Think of `String` as owning the notebook and `&str` as being allowed to read a page
of someone else's notebook. Here we only read `text`, so `&str` would also have worked —
but LeetCode's signature hands us an owned `String`, so that's what we take. We never
need to reach for the difference in this solution; we immediately walk it with
[`.chars()`](#chars).

Building and growing one: `String::from("Hi")` (or `"Hi".to_string()`) makes an owned
`String` from a literal; `String::new()` starts an empty one. `.push_str("...")` appends
text, `.push('c')` appends a single `char`, and `.len()` reports how many **bytes** it
currently holds. On the stack a `String` is just a small fixed-size handle — a pointer to
the heap, a length, and a capacity — which is why it isn't a [`Copy`](#copy) type.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)
· grow/len methods in [From-Zero Concept 07](../from-zero/rust/07-the-heap-and-string/use-it.md)

## `.chars()` — iterate a string by character {#chars}

**In one line:** walks a string one *character* at a time, rather than one raw byte
at a time.

Rust text is stored as UTF-8, where one character can take several bytes. So Rust
makes you say *how* you want to walk it, and `.chars()` is the "give me whole
characters" choice.

**What types are flowing.** Trace `for (current_index, current_char) in text.chars().enumerate()`:
- `text` is a `String`
- `.chars()` yields `char` — each Unicode character, **by value** (a `char` is a
  cheap 4-byte `Copy` type, so you get your own copy, not a reference)
- `.enumerate()` wraps each into `(usize, char)`, counting from 0

So each loop item is a `(usize, char)`, unpacked into `current_index` and
`current_char`.

**Why no `&` peel here?** Compare with the [`.iter()` loop in Two Sum](#for-iter-enumerate),
where we wrote `&current_value` to strip a reference. That was needed because
`.iter()` yields *references* (`&i32`). `.chars()` is different — it yields owned
`char` values outright, so there's nothing to peel and the pattern is a plain
`current_char`. One fewer `&` to remember, purely because of what the iterator
produces.

**A word on the index.** `current_index` here counts **characters**, not bytes —
because `.enumerate()` numbers the items `.chars()` produces. That's exactly what we
want for measuring a substring's length in characters.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)

## `.rev()` — reverse an iterator {#rev}

**In one line:** `.rev()` walks a sequence **backwards** — last item first — without
building a second copy of it.

Put `.rev()` in an iterator chain and everything after it sees the items in reverse
order. Reversing a string's characters:

```rust
for character in text.chars().rev() {  // 'c', 'b', 'a' for "abc"
    reversed.push(character);
}
```

**What types are flowing.**
- `text.chars()` yields `char`s front-to-back
- `.rev()` yields those same `char`s back-to-front

The type of each item is unchanged — `.rev()` only flips the *order*, so the loop
body reads exactly as it would forwards.

**Why not reverse it yourself?** You could collect the characters into a `Vec` and
index it from the end, but that allocates a whole vector just to read it backwards.
`.rev()` walks the original in place — no second buffer. (It works because
`.chars()` is a *double-ended* iterator, one that can be pulled from either end;
most standard iterators are.)

First seen in: [Palindrome Number](../challenges/palindrome-number/initial.rs) — used
in the first attempt; the final [arithmetic solution](../challenges/palindrome-number/README.md)
drops strings entirely.

## `usize` and underflow {#usize}

**In one line:** `usize` is Rust's *unsigned* integer for sizes and positions — it
can't go negative, and subtracting past zero **crashes** rather than wrapping to a
minus number.

Positions and lengths in Rust are `usize` (an unsigned integer: zero or above,
never negative). That's why `current_index` and `window_start` are `usize`. The
catch: because it can't represent `-1`, a subtraction like `a - b` where `b > a`
doesn't give a negative — in debug builds it **panics** (crashes), and in release
builds it silently wraps to a huge number. Either way it's a bug.

So `current_index - window_start + 1` is only safe because we can *prove*
`window_start` never passes `current_index`. And we can: `window_start` only ever
jumps to `previous_index + 1`, and `previous_index` is always an earlier position
than `current_index`, so `window_start ≤ current_index` at that line — the
subtraction is always `≥ 0`. When you subtract `usize` values, always check that the
left side can't dip below the right.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)

## integer division — `/` truncates {#int-division}

**In one line:** when both sides of `/` are integers, Rust does *integer division*
— it throws the fractional part away, so `9 / 5` is `1`, not `1.8`.

Rust picks the kind of division from the **types**, not from what you meant. If both
operands are integers (`i32`, `u64`, `usize`, …), the result is an integer, and any
remainder is silently dropped — no rounding, no warning. So `9 / 5` is `1`, `7 / 2`
is `3`, and `1 / 2` is `0`. This bites hardest when the division is buried in a
formula: `celsius * (9 / 5) + 32` looks right, but `9 / 5` becomes `1` first, and the
whole thing collapses to `celsius + 32`.

To keep the decimal, make at least one side a **float** — either annotate the value
(`let celsius: f32 = ...`) or write the literals with a dot (`9.0 / 5.0`, which is
`1.8`):

```rust
let celsius: f32 = 100.0;
let fahrenheit = celsius * (9.0 / 5.0) + 32.0; // 212.0, not 132
```

Because `celsius` is `f32`, the `9.0 / 5.0` literals are inferred as `f32` too, so
the types line up with no cast needed. (Rust's *default* float, when nothing forces
a choice, is `f64`.) The same trap exists in C, Java, and Go — any language where
`int / int` stays an `int`.

First seen in: [Celsius → Fahrenheit](../challenges/celsius-to-fahrenheit/README.md)

## `%` — the remainder operator {#remainder}

**In one line:** `a % b` is what's *left over* after dividing `a` by `b` — so
`% 10` hands you the last digit of a number.

`%` is the partner of [`/`](#int-division). Integer division tells you how many whole
times `b` fits into `a`; `%` tells you the leftover. Together they split a number:

```rust
1234 / 10   // 123  — everything except the last digit
1234 % 10   //   4  — the last digit on its own
```

That pairing is the whole trick for taking a number apart digit by digit, no text
needed: `% 10` reads the last digit, `/ 10` drops it, repeat until the number is
gone. It's also the everyday tool for "is this even?" (`n % 2 == 0`) and "wrap
around" (a clock is `hour % 12`).

**One gotcha:** `%` follows the sign of the *left* side, so `-7 % 3` is `-1`, not
`2`. It's a remainder, not a mathematician's modulo. With unsigned types like `u64`
that never comes up.

First seen in: [Palindrome Number](../challenges/palindrome-number/README.md)

## `.max()` / `.min()` — pick the larger or smaller {#ord-max}

**In one line:** `a.max(b)` returns whichever of `a` and `b` is bigger; `.min()`
returns the smaller.

Any two values that can be ordered (all the number types, for instance) support
`.max()` and `.min()` as methods. `longest = longest.max(current_index - window_start + 1)`
reads as "set `longest` to the bigger of its current value and the new window
width" — the standard way to keep a running maximum without an `if`.

**The "without it" version.** You could write it by hand:
```rust
let width = current_index - window_start + 1;
if width > longest {
    longest = width;
}
```
Same effect, three lines instead of one. `.max()` is just the tidy, idiomatic form
of that comparison, and it reads as exactly what it does.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)

## `struct` — defining your own type {#struct}

**In one line:** bundles a few named values into one new type you can name and pass
around.

`impl Solution` in earlier problems used a type LeetCode handed us. Here we define
our own:
```rust
struct TimedValue {
    value: String,
    expires_at: Option<Timestamp>,
}
```
This says "a `TimedValue` is a `value` *and* an `expires_at`, together." You build one
by naming every field — `TimedValue { value: v, expires_at: None }` — and read a field
with a dot — `stored.value`. Fields are private to the defining module by default,
which is fine here since everything lives in one file. A struct with methods gets an
[`impl` block](#impl) (that's where `is_alive_at` attaches to `TimedValue`).

A struct owns its fields, and its fields sit next to each other in memory: a
`Copy`-only struct lives entirely on the stack, while a field like `String` keeps its
handle inline in the struct and its text on the heap. Moving the struct moves every
field with it.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)
· taught from zero in [From-Zero Concept 13](../from-zero/rust/13-structs/use-it.md)

## `enum` — one of several shapes {#enum}

**In one line:** defines a type whose value is *exactly one* of a fixed list of variants —
the "or" to a [struct](#struct)'s "and."

```rust
enum Shape {
    Circle(f64),          // a variant can carry its own data
    Rectangle(f64, f64),  // different variants, different data
}
let s = Shape::Circle(2.0);   // build with Type::Variant(...)
```

A struct holds all its fields at once; an enum holds one variant at a time. Variants may
be fieldless (`enum Light { Red, Yellow, Green }`) or carry data — a tuple like
`Circle(f64)`, or named fields like `Move { x: i32, y: i32 }`. Construct with `::` and the
variant name.

**In memory** it's a **tag** (which variant) plus **one shared slot** sized for the
largest variant, so an enum is as big as its biggest variant plus the tag — never the sum.
A fieldless enum is just the tag (`Light` is 1 byte). Ownership carries over unchanged: if
any variant owns heap data it's a move type, otherwise it can be `Copy`.

To read one you [`match`](#if-let) on it, binding each variant's data into names. `Option`
and `Result` are just enums from the standard library.

First seen in: taught from zero in [From-Zero Concept 14](../from-zero/rust/14-enums/use-it.md)

## `type` — type aliases {#type-alias}

**In one line:** gives a long type a short, meaningful name — no new type, just a
nickname.

`type Record = BTreeMap<String, TimedValue>;` means "wherever I write `Record`, read
`BTreeMap<String, TimedValue>`." It's purely a readability tool: `Record` and the full
spelling are interchangeable to the compiler. We use it so signatures read as
`fn capture_live_fields(record: &Record, ...)` instead of dragging the whole nested map
type through every line. Distinct aliases (`Record` vs `SnapshotRecord`) also *document
intent* even though both are `BTreeMap`s.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `#[derive(Default)]` and `Self::default()` {#derive-default}

**In one line:** asks the compiler to write the "empty starting value" of a type for
you.

`#[derive(Default)]` on top of a struct auto-generates a `default()` constructor that
fills every field with *its* default (an empty `HashMap`, an empty `BTreeMap`, and so
on). Then:
```rust
pub fn new() -> Self {
    Self::default()
}
```
`Self` is a stand-in for "the type this `impl` is for" (`InMemoryDatabase`), so
`Self::default()` builds a fresh empty database. We expose it as `new()` by convention.
Without the derive you'd hand-write `InMemoryDatabase { records: HashMap::new(),
backups: BTreeMap::new() }` — fine now, but the derive keeps `new()` correct
automatically if you add a field later.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `match` expressions {#match}

**In one line:** picks a branch by comparing a value against several *patterns*, and
must cover every case.

[`if let`](#if-let) handles one case and ignores the rest; `match` is the full form
when you want to handle all of them:
```rust
match self.expires_at {
    Some(expiry) => timestamp < expiry,
    None => true,
}
```
Read it as "if there's a deadline, are we before it? if there's none, it's alive." The
compiler *forces* you to cover every variant of an [`Option`](#option) — leave off the
`None` arm and it won't compile — which is how `match` stops you forgetting a case.
Each arm is an expression, so the whole `match` evaluates to a value (here, the `bool`
the function returns).

Taught from zero — patterns (literals, ranges, `|`, bindings, `_`), exhaustiveness, and
the move-vs-borrow catch when binding a variant's data — in
[From-Zero Concept 16](../from-zero/rust/16-match/use-it.md).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `BTreeMap<K, V>` — a sorted map {#btreemap}

**In one line:** like a [`HashMap`](#hashmap), but it keeps its keys in **sorted
order**, so it can answer range and prefix questions.

This is Rust's spelling of the [sorted map](../glossary/sorted-map.md) concept, backed
by a balanced tree. Same everyday methods as `HashMap` — `.insert`, `.get`, `.remove`,
`.iter` — with two differences that matter:
- **Ordering:** `.iter()` yields entries **sorted by key**, always. That's why
  `scan_at` comes out sorted with no sort step.
- **Cost:** lookups are `O(log n)` instead of `HashMap`'s average `O(1)` — the price of
  keeping order.

We use it for the *inner* record (`BTreeMap<String, TimedValue>`) so fields stay sorted
for prefix scans, and for the *backup shelf* (`BTreeMap<Timestamp, Snapshot>`) so
"latest backup at or before T" is a fast [range](#btreemap-range) query. The outer map
stays a `HashMap` because it's only ever looked up by exact key.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `.range(...)` on a `BTreeMap` {#btreemap-range}

**In one line:** walks only the slice of the map between two bounds, instead of the
whole thing — the payoff for keeping keys sorted.

**What it does.** `map.range(a..)` iterates entries with key ≥ `a`, in order;
`map.range(..=b)` iterates entries with key ≤ `b`. Because a `BTreeMap` is sorted, it
jumps to the boundary in `O(log n)` and then walks — it never touches keys outside the
range.

**The two uses here, traced.**
- Prefix scan: `record.range(prefix.to_owned()..)` starts at the first field ≥ `prefix`,
  and [`.take_while(...)`](#iterator-adapters) stops as soon as a field no longer starts
  with `prefix`. Together that reads exactly the prefix block — `O(log F + M)`.
- Latest backup: `self.backups.range(..=timestamp_to_restore).next_back()` takes all
  backups at or before the target and `.next_back()` grabs the **last** one — i.e. the
  most recent. `.next_back()` works because a range is a double-ended iterator you can
  read from either end.

**The gotcha we hit — why `prefix.to_owned()`.** The natural `record.range(prefix..)`
where `prefix: &str` **does not compile**. Rust tries to compare using `&str`, then
needs `String: Borrow<&str>` (so it can view a stored `String` key as your bound's
type), and that trait impl doesn't exist — you get *"the trait `Borrow<&str>` is not
implemented for `String`."* The clean fix is to make the bound an owned `String` with
`prefix.to_owned()`, so both sides are `String` and the comparison just works. It costs
one small allocation, alongside the `Vec` the scan already builds — a fair price for a
line that reads plainly.

**Why bother instead of filtering.** `record.iter().filter(|(f, _)| f.starts_with(prefix))`
would also work — but it scans **all** `F` fields every time. The range walk touches
only the matches. On a large record that's the whole difference between `O(F)` and
`O(log F + M)`.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `Option<T>` — a value that may be absent {#option}

**In one line:** Rust's answer to `null` — a value is either `Some(thing)` or `None`,
and the compiler makes you handle both.

[`if let`](#if-let) already showed `Option` in a lookup; here it's also a *stored* and
*returned* type. `expires_at: Option<Timestamp>` means "a deadline **or** nothing"
(`None` = never expires). `get_at(...) -> Option<&str>` means "the value **or** nothing"
(`None` = absent or expired). Because there's no `null`, a missing value can't sneak
past you — you can't use the inner value without first opening the `Option` (via
[`match`](#match), [`if let`](#if-let), [`?`](#question-mark), or a method).

**`.map` on an `Option`.** `stored.expires_at.map(|expiry| expiry - timestamp)`
transforms the value *inside* a `Some`, leaving `None` untouched: `Some(25)` becomes
`Some(15)`, `None` stays `None`. It's the tidy way to say "if there is a deadline,
convert it to a remaining duration; if there isn't, there still isn't."

Taught from zero — why null is the problem, and how `Option` is just an enum — in
[From-Zero Concept 15](../from-zero/rust/15-option/use-it.md); the concept behind it lives
in [Null (and the billion-dollar mistake)](../glossary/null-and-the-billion-dollar-mistake.md).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `Result<T, E>` — success or a reason for failure {#result}

**In one line:** the sibling of [`Option`](#option) for operations that can *fail* — a
value is either `Ok(the value)` or `Err(the error)`, and the compiler makes you handle
both.

Where `Option`'s `None` is empty ("nothing here"), `Result`'s `Err` **carries a reason**
("here's what went wrong"). It's a plain [enum](#enum) from the standard library with two
type parameters — `T` for the success value, `E` for the error:
```rust
enum Result<T, E> { Ok(T), Err(E) }

let parsed: Result<i32, _> = "42".parse::<i32>();   // Ok(42)
let broken: Result<i32, _> = "abc".parse::<i32>();  // Err(ParseIntError)
```
A function that can fail says so in its return type (`fn half(n: i32) -> Result<i32,
String>`), so failure can't sneak past a caller the way an exception can. Open it the same
ways you open any enum — [`match`](#match), [`if let`](#if-let), the [`?`](#question-mark)
operator, or the crash-on-`Err` escape hatch [`.unwrap()`](#unwrap).

**In memory** it's the enum "tag + one shared slot" — but since *both* variants carry data,
the slot is sized for the larger of `T` and `E`. The [niche trick](#option) still applies
when one side is empty and the other has a spare pattern (`Result<Box<T>, ()>` is free).

Taught from zero — why exceptions hide, and how `Result` is just an enum whose `Err` side
carries the story — in [From-Zero Concept 23](../from-zero/rust/23-result/use-it.md).

First seen in: taught from zero in [From-Zero Concept 23](../from-zero/rust/23-result/use-it.md)

## `?` — the question-mark operator {#question-mark}

**In one line:** unwraps a `Some`/`Ok` and, on `None`/`Err`, *returns early* from the
whole function with that same empty result.

**Trace it.**
```rust
let stored = self.records.get(key)?.get(field)?;
```
- `self.records.get(key)` is `Option<&Record>`.
- `?` says: if that's `None`, **stop and return `None` from `get_at` right now**;
  otherwise hand back the `&Record` inside and keep going.
- `.get(field)?` does the same one level deeper.

So by the line after, `stored` is a plain `&TimedValue` — both "missing key" and
"missing field" have already been dealt with by bailing out. It only works because
`get_at` itself returns an `Option`, so there's a `None` for `?` to return.

**Without it.** You'd nest two matches:
```rust
let record = match self.records.get(key) {
    Some(r) => r,
    None => return None,
};
let stored = match record.get(field) {
    Some(s) => s,
    None => return None,
};
```
Six lines of ceremony for what `?` says in two characters. That "bail out on absence,
otherwise unwrap and continue" is the pattern `?` exists for.

Taught from zero — the desugaring to a `match`, why the function must return a `Result`,
and the `From::from` error conversion — in
[From-Zero Concept 24](../from-zero/rust/24-question-mark/use-it.md).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `let ... else` — bind or bail {#let-else}

**In one line:** unwrap a value into a variable that stays in scope, and if it isn't
there, run an `else` block that must *leave* (return, break, continue).

**Trace it.**
```rust
let Some(record) = self.records.get_mut(key) else {
    return false;
};
record.remove(field);
```
- `self.records.get_mut(key)` is `Option<&mut Record>`.
- `let Some(record) = ... else { return false; }` says: if it's `Some`, bind the inside
  to `record` **and carry on with `record` usable below**; if it's `None`, run the
  `else`, which here returns `false`.

The key contrast with [`if let`](#if-let): with `if let`, the unwrapped value lives only
*inside* the `if` block, which pushes the rest of your logic one indent deeper each
time. `let ... else` unwraps and keeps the value at the **top level**, so the happy path
reads as a flat sequence instead of a staircase of nested blocks. That's why it fits the
"check a precondition, then get on with it" style of `delete_at`, `scan_by_prefix_at`,
and `restore`.

**The rule:** the `else` block must diverge — `return`, `break`, `continue`, or panic —
because after it, Rust needs `record` to definitely exist. An `else` that fell through
without leaving wouldn't compile.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `.entry(...).or_default()` {#entry-or-default}

**In one line:** looks up a key and, if it's missing, inserts a fresh empty value —
then hands you a mutable reference either way.

```rust
self.records.entry(key.to_owned()).or_default().insert(field, ...);
```
Reading it: `.entry(key)` finds the slot for `key`; `.or_default()` says "if that slot
is empty, put a [`Default`](#derive-default) value there first" — for a `BTreeMap` that's
an empty map — and returns a `&mut` to whatever's now in the slot; then `.insert(...)`
adds the field to that record. It's the one-liner for "get this key's record, creating an
empty one on first use."

**Without it** you'd branch by hand:
```rust
if !self.records.contains_key(key) {
    self.records.insert(key.to_owned(), BTreeMap::new());
}
self.records.get_mut(key).unwrap().insert(field, ...);
```
— two lookups and an [`.unwrap()`](#unwrap). `.entry().or_default()` does it in one
lookup and no unwrap. (Its cousin `.or_insert_with(...)` lets you supply a custom
starting value instead of the default.)

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## closures — `|x| ...` {#closures}

**In one line:** a small anonymous function written inline, often to tell an iterator
adapter *what* to do to each item.

`|expiry| expiry - timestamp` is a function with one parameter `expiry` whose body is
`expiry - timestamp`. The `| |` hold the parameters; what follows is the body. Closures
can **capture** variables from around them — here `timestamp` isn't a parameter, it's
grabbed from the enclosing function — which is exactly why they're handier than a
top-level `fn` for these one-off transforms.

You'll see them destructure tuples too: `|(field, _)| field.starts_with(prefix)` takes
one `(key, value)` item, binds `field` to the first part, and `_` throws away the
second. They feed the [iterator adapters](#iterator-adapters) below and methods like
[`Option::map`](#option).

Taught from zero — capturing the environment (vs a `fn` that can't), a closure as a
struct of "captured data + code", the `Fn`/`FnMut`/`FnOnce` capture modes, `move`, and
why it's zero-cost — in [From-Zero Concept 26](../from-zero/rust/26-closures/use-it.md).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## iterator adapters — `.filter()`, `.map()`, `.collect()`, … {#iterator-adapters}

**In one line:** chainable steps that describe a transformation over a sequence, doing
no work until something asks for the results.

An iterator is a lazy stream of items. **Adapters** reshape the stream; a **consumer**
runs it. Trace the chain in `format_live_fields`:
```rust
fields
    .filter(|(_, stored)| stored.is_alive_at(timestamp))
    .map(|(field, stored)| format!("{field}({})", stored.value))
    .collect()
```
- start: a stream of `(&String, &TimedValue)` pairs.
- `.filter(pred)` keeps only items where the [closure](#closures) returns `true` — here,
  the live ones. Still a stream.
- `.map(f)` transforms each surviving item — here into a `String` like `name(alice)`.
  Still a stream.
- `.collect()` is the **consumer**: it runs the whole chain and gathers the results into
  a collection — a `Vec<String>` here, inferred from the return type.

Two more used in this solution:
- `.filter_map(f)` combines filter + map: the closure returns an [`Option`](#option), and
  `Some(x)` keeps `x` while `None` drops the item. `capture_live_state` uses it to build
  a snapshot key only when its record has live fields.
- `.take_while(pred)` yields items until the first time `pred` is false, then **stops** —
  which is what ends a prefix [range](#btreemap-range) walk at the edge of the block.

**Why chains over loops.** Each adapter is zero-cost (it compiles down to the same loop
you'd write by hand) but reads as a sentence: *filter to live, format each, collect*. And
nothing allocates until `.collect()`.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `impl Trait` in argument position {#impl-trait-arg}

**In one line:** a parameter type that says "any type that can do X," instead of naming
one concrete type.

```rust
fn format_live_fields<'a>(
    fields: impl Iterator<Item = (&'a String, &'a TimedValue)>,
    timestamp: Timestamp,
) -> Vec<String>
```
`fields: impl Iterator<Item = ...>` means "pass me **anything** that iterates over
`(&String, &TimedValue)` pairs." That's what lets **one** function serve both callers:
`scan_at` passes a plain map iterator, and `scan_by_prefix_at` passes a
`range(...).take_while(...)` chain — two *different* concrete iterator types, both
accepted here.

**Why not name the type?** You can't, sanely. The type of a `.range(..).take_while(..)`
chain is a monstrous nested generic the compiler builds internally; writing it out would
be unreadable and would change if you tweaked the chain. `impl Trait` lets you say what
the value *does* (it iterates) rather than *what it is*.

**Why not `&[...]` or `Vec`?** Taking a slice or vector would force each caller to first
*collect* its items into an allocation just to hand them over — defeating the point.
Accepting `impl Iterator` means the items stream straight through with no middle
collection.

(The `<'a>` is a [lifetime](#lifetimes) — see below — tying the borrowed `&String`s in
the items to the same borrow, so the compiler knows how long they live.)

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## lifetimes — `<'a>` {#lifetimes}

**In one line:** a label the compiler uses to check that a borrow doesn't outlive the
data it points into — you're naming a borrow so its span can be tracked, not creating
anything at runtime.

**The idea first.** Every reference (`&T`) borrows something that lives somewhere. Rust
must be sure the reference is gone before the thing it points to is. Usually it figures
this out silently. Sometimes — when references flow *through* a function's signature in a
non-obvious way — you have to give the borrow a name so the compiler can connect the
dots. That name is a lifetime, written `'a` (an apostrophe plus a word).

**Where it shows up here.**
```rust
fn format_live_fields<'a>(
    fields: impl Iterator<Item = (&'a String, &'a TimedValue)>,
    ...
```
The iterator yields items that *contain borrows* — `&String` and `&TimedValue`. The
`<'a>` declares a lifetime named `a`, and tagging both references with it says "these two
borrows share one lifetime `'a`, and the iterator's items are valid for as long as `'a`
lasts." It ties the borrowed field names and values to the record they came from, so the
compiler can guarantee we don't hold them after that record is gone.

**Without it.** Drop the `<'a>` and write `Item = (&String, &TimedValue)` and the
compiler objects that it can't tell how long those references are meant to live — the
borrows are "unconstrained." Naming the lifetime resolves it. (It's needed here, but not
on simpler borrows like `&self` methods, where Rust infers the lifetime for you — that
inference is called *elision*.)

**Runtime cost:** none. Lifetimes are erased after checking; they exist only to prove the
code is memory-safe at compile time.

Taught from zero — the dangling-reference danger, elision, why `longest` needs a named
lifetime, and the zero-runtime-cost erasure — in
[From-Zero Concept 25](../from-zero/rust/25-lifetimes/use-it.md).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `format!` — building strings {#format}

**In one line:** builds a `String` from a template with `{}` holes filled by values.

`format!("{field}({})", stored.value)` produces a string like `name(alice)`. Two filling
styles appear: `{field}` pulls the variable named `field` directly into the hole (a
"captured identifier"), while the empty `{}` is filled by the next argument,
`stored.value`. It's the same templating as `println!`, except `format!` *returns* the
string instead of printing it.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `{:.1}` — format specifiers (decimals, width, …) {#format-spec}

**In one line:** the part after the `:` inside `{}` controls how a value is
*displayed* — `{:.1}` means "one digit after the decimal point" — without changing
the value itself.

A `{}` hole can carry formatting instructions after a colon: `{:.1}`, `{:5}`,
`{:08.2}`, and so on. The most common is **precision** for numbers, written
`.N` — it fixes how many digits show after the decimal point, rounding (and padding
with zeros) to fit:

```rust
let f = 77.0_f32;
println!("{}", f);      // 77      <- plain Display drops the trailing .0
println!("{:.1}", f);   // 77.0    <- forced to one decimal place
println!("{:.2}", 3.14159); // 3.14    <- rounded to two places
```

This is why a Celsius→Fahrenheit result of `77.0` prints as `77` under plain `{}`
but as `77.0` under `{:.1}`. Key distinction: this is about **display only**. The
`f32` in memory is unchanged — `{:.1}` doesn't round the stored value, just the
text. So in `println!("{} Celsius = {:.1} Fahrenheit", celsius, fahrenheit)` the
Celsius side (plain `{}`) shows `25` while the Fahrenheit side shows `77.0`, even
though both are `f32`. If you want *both* shown with a decimal, give both a spec
(`{:.1}`).

Precision is just one specifier — the same slot also does width (`{:5}` → pad to 5
chars), zero-fill (`{:05}`), alignment, sign, and more.

First seen in: [Celsius → Fahrenheit](../challenges/celsius-to-fahrenheit/README.md)

## `.to_owned()` / `.clone()` — making an owned copy {#to-owned-clone}

**In one line:** turn a borrowed view into an owned value you can store, or duplicate an
owned value so you have your own.

Map keys and stored values must be **owned** — a `HashMap<String, ...>` owns its `String`
keys; it can't hold a borrowed `&str` that might vanish. So when we store, we convert:
- `key.to_owned()` turns the borrowed `&str` parameter into an owned `String` to use as a
  key. (`.to_owned()` and `.to_string()` do the same thing for `&str` → `String`;
  `to_owned` is the general "borrowed → owned" name.)
- `stored.value.clone()` duplicates an existing `String` when we need a second owned copy
  (building a snapshot from live data, or rebuilding live data from a snapshot).

Each of these allocates, so they're not free — but storing in an owned collection genuinely
*requires* ownership, so the copies are necessary, not waste. The rule of thumb: borrow
(`&str`, `&T`) in function signatures for cheap reads; reach for `.to_owned()`/`.clone()`
only at the moment you must *keep* the data.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `Copy` types — values duplicated on assignment {#copy}

**In one line:** for small, stack-only types (`i32`, `bool`, `char`, `f64`, …),
`let b = a` makes a second independent copy instead of sharing or moving — and `a`
stays usable.

**What actually happens.** These values live entirely on the stack; the value *is* its
bytes. So `let b = a` just duplicates those bytes into a new box:

```rust
let mut a = 5;
let b = a;   // b is a separate copy
a = 99;      // only a changes; b is still 5
```

The same duplication happens when you pass one into a function — the argument is copied
onto the callee's frame, so the caller's variable is untouched.

**Why this matters / the "without it" contrast.** Not every type is `Copy`. A type that
owns data elsewhere in memory (a `String`, which holds a pointer to heap data) is *not*
`Copy`: for those, `let b = a` **moves** ownership and `a` becomes unusable, because
duplicating the little stack pointer would leave two owners of one heap allocation. So
`Copy` is Rust's marker for "safe and cheap to duplicate byte-for-byte." Knowing which
camp a type is in is the foundation of ownership (moves).

First seen in: [From-Zero Concept 06](../from-zero/rust/06-copy-types/use-it.md)

## `&T` — shared references (borrowing) {#borrow}

**In one line:** `&value` hands out a *reference* — a pointer that lets code read a value
without owning it, so the owner keeps the value and nothing is copied.

**What it's for.** Taking a value by ownership (`s: String`) *moves* it in, so the caller
loses it; cloning copies the whole thing. When a function only needs to *read*, take a
shared reference instead:

```rust
fn length(s: &String) -> usize { s.len() }

let text = String::from("hello");
let n = length(&text);   // borrow, don't move
println!("{text}");      // still usable — text was never given away
```

The `&` appears twice: `&String` in the parameter type ("I take a reference"), and
`&text` at the call ("borrow mine"). See also [`&` in patterns](#ref-pattern), which is
the reverse move — peeling a reference back off in a pattern.

**Two rules that make it safe and cheap.**
- A reference may never **outlive** the value it points at — the compiler rejects any
  reference that could dangle, so a borrow always points at live data. (The mechanism is
  [lifetimes](#lifetimes).)
- A plain `&T` is **read-only** (`shared`). You can hold many `&T` to the same value at
  once, but none can mutate through it. To modify, use [`&mut`](#mut-ref) — the exclusive
  reference — which is the counterpart with its own borrow rules.

A reference is a fixed-size address regardless of how large the borrowed value is, so
borrowing to read is the default in idiomatic Rust; take ownership only when you must
*keep* the value.

First seen in: [From-Zero Concept 10](../from-zero/rust/10-borrowing-with-ref/use-it.md)

## slices — `&str` and `&[T]` {#slice}

**In one line:** a slice is a reference to a *contiguous range* of a value — a pointer to
where the range starts plus a length — so it borrows part of a collection without copying.

**String slices.** `&s[start..end]` on a `String` yields a `&str`, a view into the
existing heap buffer:

```rust
let s = String::from("hello world");
let hello = &s[0..5];   // "hello"  (end is exclusive)
let world = &s[6..11];  // "world"
```

Range shorthands: `[..n]` from the start, `[n..]` to the end, `[..]` the whole thing.

**`&str` vs `String`.** A `String` *owns* growable heap text (ptr + len + capacity); a
`&str` is a *borrowed* two-word view (ptr + len) that owns nothing. A string literal like
`"hello"` is a `&str` pointing at read-only bytes baked into the program — same type as a
slice of a `String`, just pointing at a different place. Prefer `&str` parameters for
read-only text: they accept both literals and slices of a `String`. See
[`String`](#string) and [`&T` borrowing](#borrow).

**They obey the borrow rules.** A slice is a shared borrow, so while one is alive you
can't take a `&mut` to grow the source (`push_str` fails with E0502) — the borrow checker
prevents the slice from being left dangling if the buffer reallocated.

**Array/vec slices.** The same idea gives `&[T]` — a window into an array or `Vec<T>`:
`&numbers[1..3]` is a slice of two elements. Identical shape: pointer + length, no copy.

First seen in: [From-Zero Concept 12](../from-zero/rust/12-slices/use-it.md)

## generics — `<T>` {#generics}

**In one line:** `<T>` is a stand-in for "some type, decided later," so you write a function
or type *once* and use it with every type instead of copy-pasting one version per type.

**On a function.** A `<T>` after the name introduces a type parameter; use `T` where a
concrete type would go. Rust infers what `T` is from how you call it:

```rust
fn first<T>(pair: (T, T)) -> T { pair.0 }

first((10, 20));      // T = i32
first(("hi", "bye")); // T = &str
```

**On a struct.** Types are generic too; multiple parameters (`<T, U>`) let fields differ:

```rust
struct Point<T> { x: T, y: T }        // both fields the same type
struct Pair<T, U> { a: T, b: U }      // two independent types — like HashMap<K, V>
```

**The catch — a bare `T` can only be shuffled, not inspected.** Since the code must work for
*every* type, Rust rejects any operation not guaranteed for all types: `if a > b` on two `T`
won't compile. You can move/return/store a `T`, but to compare, print, or add it you must add
a **trait bound** (`<T: PartialOrd>`) promising what `T` can do. See
[Concept 19](../from-zero/rust/19-generics/use-it.md).

**Zero-cost at runtime.** The compiler *monomorphizes*: it stamps out a separate concrete copy
of the generic code for each type actually used, so a generic call is byte-for-byte as fast as
a hand-written type-specific one. The cost is paid in compile time and binary size, never at
runtime — see [Concept 19 · Under the hood](../from-zero/rust/19-generics/under-the-hood.md).

First seen in: [From-Zero Concept 19](../from-zero/rust/19-generics/use-it.md)

## string indexing — why `s[i]` is forbidden {#string-indexing}

**In one line:** Rust won't let you index a string by an integer position (`s[0]`), because
a string is UTF-8 **bytes** and one character can span several of them — so "position `i`"
is ambiguous.

`"café"` is 4 characters but 5 bytes (`é` is 2). Byte 3 is only *half* of `é`, so `s[3]`
can't return a sensible character. Rather than hand back a broken fragment, Rust rejects
`str` indexing at compile time (`error[E0277]: the type str cannot be indexed by {integer}`).

**What to reach for instead:**
- **First / n-th character** — walk whole characters with [`.chars()`](#chars):
  `s.chars().next().unwrap()` (first, as an [`Option`](#option)); `s.chars().nth(i)`.
- **A raw byte, on purpose** — `s.as_bytes()[i]` gives a `u8`; you chose bytes explicitly.
- **A span** — [range slicing](#slice) still works: `&s[0..1]`. But a range that splits a
  character **panics** at runtime.
- **Length** — `.len()` is **bytes**; `.chars().count()` is **characters**.

See [Interlude 12a](../from-zero/rust/12a-string-indexing/use-it.md) for the full memory
picture.

First seen in: [Interlude 12a](../from-zero/rust/12a-string-indexing/use-it.md)

## `trait` — defining and implementing shared behaviour {#trait}

**In one line:** a `trait` is a named set of abilities (a contract); a type gains those abilities
with `impl Trait for Type`, and generics can then demand them with a bound (`<T: Trait>`).

**Define the contract.** List the method signatures a type must provide — signatures only, no body:

```rust
trait Greet {
    fn hello(&self) -> String;          // required: every implementer must write this

    fn greet_twice(&self) -> String {   // default: implementers get this free
        format!("{} {}", self.hello(), self.hello())
    }
}
```

`&self` means the method is called on a value (`x.hello()`), borrowing it — same `&self` as
[struct methods](#impl). A required method has no body; a **default** method has one and can be
overridden.

**Sign the contract.** `impl Trait for Type` fills in the required methods:

```rust
struct Dog;
impl Greet for Dog {
    fn hello(&self) -> String { String::from("Woof!") }
}
```

Now `Dog` can be used anywhere a `Greet` is asked for, and gets `greet_twice()` for free.

**Use it as a bound.** A trait bound on a generic unlocks the abilities *inside* the function and
restricts *which* types may be passed in:

```rust
fn larger<T: PartialOrd>(a: T, b: T) -> T {   // T can be ordered → `>` is allowed
    if a > b { a } else { b }
}
fn show<T: PartialOrd + Debug>(x: T) { }      // `+` requires several traits at once
```

Without the bound, a bare `T` can only be shuffled around (see [generics](#generics)); the bound is
what lets the body actually operate on it.

**You already used traits.** `>` / `<` are the `PartialOrd` trait; `{:?}` is the `Debug` trait;
`#[derive(Debug, Clone, PartialEq)]` auto-writes the routine `impl`s of those common traits for you.

**Cost.** A trait *bound* is [monomorphized](#generics) like any generic — the method call compiles
to a **direct jump** chosen at compile time (*static dispatch*), zero runtime overhead. Runtime method
lookup only happens with a `dyn` **trait object**, a separate tool. See
[Concept 20](../from-zero/rust/20-traits/use-it.md).

First seen in: [From-Zero Concept 20](../from-zero/rust/20-traits/use-it.md)

## `dyn Trait` — trait objects {#dyn}

**In one line:** `dyn Greet` means "some type implementing `Greet`, chosen at run time"; it lets a
single collection hold **different** types behind one trait — at the cost of a small per-call lookup.

**Why it exists.** A trait *bound* (`<T: Greet>`) is [monomorphized](#trait) — one stamped copy per
type — so it can never hold a **mix** of types in one `Vec`. A trait *object* can:

```rust
let animals: Vec<Box<dyn Greet>> = vec![Box::new(Dog), Box::new(Cat)];
for a in &animals {
    println!("{}", a.hello());   // Dog::hello on dogs, Cat::hello on cats — one loop
}
```

**Always behind a pointer.** A bare `dyn Greet` has no known size (the real type could be anything),
so it must sit behind [`Box<dyn Greet>`](#box) (owned, heap) or `&dyn Greet` (borrowed). `Vec<dyn
Greet>` won't compile; `Vec<Box<dyn Greet>>` will — every slot is one uniform pointer.

**How the call works — dynamic dispatch.** A trait object is a **fat pointer**: two pointers, one to
the data and one to a **vtable** (a per-`(type, trait)` table of method addresses). `a.hello()`
follows the vtable to the right function *at run time* — one pointer hop, no inlining. Contrast
[static dispatch](#trait) (a bound), which bakes the address in at compile time for free. Reach for a
bound by default; reach for `dyn` when you need a heterogeneous collection.

First seen in: [From-Zero Concept 21](../from-zero/rust/21-trait-objects/use-it.md)

## shadowing — reusing a name {#shadowing}

**In one line:** writing `let` with a name that already exists makes a **brand-new
variable** that reuses the name — it does not change the old one.

**Not the same as `mut`.** These look alike but are different events in memory:

```rust
let mut a = 5;
a = 6;          // mutation: same box, overwrite 5 with 6. Type is fixed.

let b = 5;
let b = "five"; // shadowing: a NEW box that hides the old one. Type can change.
```

`mut` reaches into one box and overwrites it, so the type can never change. Shadowing
leaves the old box untouched and builds a second box beside it; the name simply points
at the newer one from that line on. Because it's a fresh box, the new variable may have a
**different type** — that's why `let b = 5;` (an `i32`) can be followed by
`let b = "five";` (a `&str`).

**What happens to the old value.** It is *not* dropped at the shadow — it stays in
memory, just unreachable by that name, and is dropped at the end of its scope like any
other value. This matters when the new variable **borrows** the old one:

```rust
let s = String::from("  hi  ");
let s = s.trim();   // new `s` is a &str borrowing INTO the old String's buffer
```

The old `String` must stay alive for the `&str` to point into — and it does, precisely
because shadowing doesn't drop it. (You still can't `return` that `&str` past the old
`String`'s scope: the owner would die and the slice would dangle.)

**Why reach for it.** Idiomatic for *"same thing, refined"* — clean up or convert a value
and keep one honest name, so later code can't grab the raw version by accident. The
canonical shape is trim/parse pipelines: `let n = input.trim(); let n: i32 = n.parse()?;`

First seen in: [From-Zero Interlude 12b](../from-zero/rust/12b-trim-returns-str/use-it.md)

## `.trim()` — a borrowed slice without the outer whitespace {#trim}

**In one line:** `trim` returns a [`&str`](#slice) window over the same text with leading
and trailing whitespace skipped — it **borrows**, it doesn't move or copy.

Its signature is `pub fn trim(&self) -> &str`. The `&self` means it takes the string
*by reference*, so calling it never consumes the `String` (or `&str`) — the original is
left intact. The returned `&str` is a [slice](#slice) pointing into the original buffer;
no new text is allocated. Common with input:

```rust
let mut line = String::new();
std::io::stdin().read_line(&mut line).unwrap();
let line = line.trim();   // &str, no trailing '\n'; `line` the String still owns the buffer
```

Because the result borrows the source, the source has to outlive it (see
[shadowing](#shadowing) for how the common `let line = line.trim();` keeps that owner
alive). Relatives: `.trim_start()` / `.trim_end()` for one side only.

First seen in: [From-Zero Interlude 12b](../from-zero/rust/12b-trim-returns-str/use-it.md)

## the `Iterator` trait — `.next()` and building your own {#iterator-trait}

**In one line:** every iterator is a small value with one required method, `.next()`, that
hands back the next item as [`Some(item)`](#option) or `None` when it's exhausted; a `for`
loop is just that method called until `None`.

**What an iterator *is*.** Not a copy of your data — a **cursor**. It holds a reference to the
data plus a **position**, and each `.next()` reads the item at the position, advances the
position by one, and returns what it read. So `"hi".bytes()` doesn't build `[104, 105]`; it
remembers "here's the string, I'm at byte 0."

```rust
let mut bytes = "hi".bytes();
bytes.next();   // Some(104)  ('h'), position 0 → 1
bytes.next();   // Some(105)  ('i'), position 1 → 2
bytes.next();   // None       (ran off the end)
```

**A `for` loop desugars to `.next()`.** `for x in it { body }` is roughly
`loop { match it.next() { Some(x) => { body } None => break } }`. That's the whole contract:
anything with a `.next()` can be looped, mapped, filtered, zipped.

**Build your own** by implementing the trait — one associated `Item` type and one `next`:

```rust
struct CountUp { current: u32, limit: u32 }

impl Iterator for CountUp {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.current == self.limit { return None; }
        let value = self.current;
        self.current += 1;          // advance the position
        Some(value)                 // hand back what we read
    }
}
```

`&mut self` because moving the position mutates the cursor. Write just this, and `CountUp`
gains every adapter (`.map`, `.take`, …) and works in `for` — they're all built on `.next()`.
This is also *why* adapters are [lazy](#iterator-adapters): each adapter is a cursor wrapping a
cursor, and no `.next()` runs until a consumer pulls.

First seen in: [From-Zero Interlude 28a](../from-zero/rust/28a-how-next-works/use-it.md)

## `.bytes()` — iterate a string's raw bytes {#bytes}

**In one line:** walks a string one **raw byte** (`u8`) at a time — the byte-level sibling of
[`.chars()`](#chars).

Rust text is UTF-8, and Rust makes you choose *how* to walk it. [`.chars()`](#chars) gives
whole Unicode characters (`char`); `.bytes()` gives the underlying bytes as numbers:

```rust
for b in "hi".bytes() {   // 104, then 105
    println!("{b}");
}
```

**Why reach for it over `.chars()`?** When you're comparing plain ASCII/English text
position-by-position, bytes are simpler and cheaper — a `u8` compare is one machine
instruction, and each byte *is* one character in ASCII. The catch: for non-ASCII text a single
character spans several bytes, so counting or slicing by byte can split a character. Use
`.bytes()` for ASCII-only work (like a common-prefix scan on English words); reach for
`.chars()` the moment Unicode is in play.

First seen in: [From-Zero Interlude 28a](../from-zero/rust/28a-how-next-works/use-it.md)

## `.zip()` — pair two iterators {#zip}

**In one line:** `a.zip(b)` walks two iterators in lockstep, yielding pairs `(a_item, b_item)`,
and **stops as soon as either one runs out**.

```rust
let names  = ["ann", "bo"];
let scores = [10, 20, 30];
for (name, score) in names.iter().zip(scores.iter()) {
    println!("{name}: {score}");   // ann: 10 / bo: 20  — the extra 30 is never paired
}
```

The stopping rule is the whole personality of `.zip`: it can only make a pair when **both**
cursors can hand it a next item, so the result is as long as the *shorter* input. That's what
lets a common-prefix scan compare `"flower"` against `"flow"` safely — the pairs simply end
when `"flow"` does. The items are [tuples](#for-iter-enumerate), so the next adapter usually
destructures them with a `|(a, b)|` [closure](#closures).

First seen in: [From-Zero Interlude 28b](../from-zero/rust/28b-zip-take-takewhile-count/use-it.md)

## `.take()` — at most `n` items {#take}

**In one line:** `.take(n)` passes along **up to** `n` items and then reports the stream empty
— *up to*, not *exactly*: a shorter stream just ends early.

```rust
let first_three: Vec<u8> = "flower".bytes().take(3).collect();   // [102, 108, 111]
let all_two:     Vec<u8> = "hi".bytes().take(5).collect();       // [104, 105] — only 2 exist
```

It's a cheap cap on how far a lazy chain will walk. Its cousin [`.take_while(pred)`](#iterator-adapters)
caps by a *condition* instead of a count — it stops at the first item that fails the test.
Don't confuse `.take_while` with [`.filter`](#iterator-adapters): `take_while` **stops** at a
failure, `filter` **skips** it and keeps going.

First seen in: [From-Zero Interlude 28b](../from-zero/rust/28b-zip-take-takewhile-count/use-it.md)

## `.copied()` — turn `&T` items into owned `T` {#copied}

**In one line:** an adapter that dereferences each item, turning a stream of references
(`&i32`) into a stream of owned values (`i32`) — only for [`Copy`](#copy) types.

[`.iter()`](#for-iter-enumerate) borrows a collection, so it yields *references*. When the rest
of your chain (or a `Vec<i32>` you're collecting into) wants plain values, `.copied()` bridges
the gap without a manual `*`:

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter().copied().map(|n| n * 2).collect();   // [2, 4, 6]
```

Without it the closure would see `&i32` and you'd sprinkle `*` around, or `.collect()` would
refuse to build a `Vec<i32>` from `&i32`. It only works for `Copy` types (copying is cheap and
leaves the original intact); for non-`Copy` data you'd use `.cloned()` and pay for the clone.

First seen in: [From-Zero Interlude 28b](../from-zero/rust/28b-zip-take-takewhile-count/use-it.md)

## `.count()` — consume and count {#count}

**In one line:** a **consumer** that runs the iterator to the end and returns how many items
came through (a `usize`).

```rust
let vowels = "flower".bytes().filter(|b| b"aeiou".contains(b)).count();   // 2
```

Because it's a consumer, `.count()` is the thing that finally *pulls* a lazy chain — nothing
above it runs until `.count()` starts asking for items. Pair it with
[`.take_while`](#iterator-adapters) to answer "how many items at the front satisfy this?" — the
common-prefix length `first.bytes().zip(word.bytes()).take_while(|(a, b)| a == b).count()` is
exactly that shape.

First seen in: [From-Zero Interlude 28b](../from-zero/rust/28b-zip-take-takewhile-count/use-it.md)

## `thread::spawn` — run code on a new thread {#thread-spawn}

**In one line:** starts a **second** line of execution, on its own stack, running the closure you
hand it — so two pieces of code make progress **at the same time**.

**What it is.** `std::thread::spawn(f)` launches a new OS thread that runs the closure `f`, while the
current thread keeps going. It returns a **`JoinHandle`**; call `.join()` on it to block until that
thread finishes and receive what its closure returned, wrapped in a [`Result`](#result) (`Ok(value)`,
or `Err` if the thread panicked).

```rust
use std::thread;
let handle = thread::spawn(|| 2 + 2);   // runs on a new thread
let answer = handle.join().unwrap();     // wait; get the return value back
println!("{}", answer);                   // 4
```

**`move` is usually required.** A spawned thread may outlive the function that started it, so its
closure can't *borrow* the caller's local variables — that reference could dangle once the caller's
[stack frame](#pub-fn) is gone. Prefix the closure with `move` to make it **take ownership** of
everything it captures; the value is moved onto the thread and lives as long as the thread does. It's
the ordinary [move](#to-owned-clone) rule doing double duty as a concurrency safety guarantee.

```rust
let data = String::from("work");
let handle = thread::spawn(move || println!("{}", data)); // `data` moved into the thread
handle.join().unwrap();
// `data` is no longer usable here — ownership left.
```

**Why this way.** Threads suit independent work that can run at once (two files, a slow computation
off to the side). Order between threads is **not** guaranteed without `.join()`. Handing one value to
one thread is what `move` does; letting *several* threads share one value needs the thread-safe pair
`Arc<Mutex<T>>` (the concurrent siblings of [`Rc`](#rc)/[`RefCell`](#refcell)).

First seen in: [From-Zero concept 34 — threads](../from-zero/rust/34-threads/use-it.md)

## `Arc<T>` — shared ownership across threads {#arc}

**In one line:** [`Rc<T>`](#rc) with a thread-safe owner count — many threads own one heap value.

**What it is.** `std::sync::Arc<T>` ("**a**tomically **r**eference **c**ounted") is the same
count-the-owners pointer as `Rc`, except the count is updated with **atomic** instructions, so two
threads bumping it at the same instant can't corrupt it. `Arc::clone(&handle)` makes another owner
(a pointer copy plus `+1`); the value is freed when the last handle drops.

```rust
use std::sync::Arc;
use std::thread;

let names = Arc::new(vec![String::from("ada"), String::from("alan")]);

for _ in 0..2 {
    let names = Arc::clone(&names);            // one owner per thread
    thread::spawn(move || println!("{}", names.len()));
}
```

**Why not just `Rc`?** `Rc`'s count is a plain `+= 1`, which the machine performs as read → add →
write. Two threads can read the same old value and both write the same new one, losing an increment —
the count then hits zero while a handle is still alive, freeing the value underneath it. The compiler
rejects `Rc` across threads for exactly that reason. Atomics cost slightly more than plain arithmetic,
so keep using `Rc` on a single thread.

**`Arc` alone is read-only**, just like `Rc`. To *change* the shared value, pair it with a
[`Mutex`](#mutex): `Arc<Mutex<T>>` is the thread-safe echo of [`Rc<RefCell<T>>`](#refcell).

First seen in: [From-Zero concept 35 — `Arc<Mutex<T>>`](../from-zero/rust/35-arc-mutex/use-it.md)

## `Mutex<T>` — one thread at a time {#mutex}

**In one line:** a lock wrapped around a value: whoever holds the lock may change it, and everyone
else waits their turn.

**What it is.** `std::sync::Mutex<T>` ("**mut**ual **ex**clusion") stores a value plus a lock flag.
`.lock()` waits until the flag is free, flips it, and returns a **`MutexGuard`** inside a
[`Result`](#result). The guard acts like a [`&mut T`](#mut-ref); when it **drops** (end of scope) the
lock is released — there is no `unlock` to forget.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let handle = {
    let counter = Arc::clone(&counter);
    thread::spawn(move || {
        let mut value = counter.lock().unwrap();   // waits for its turn
        *value += 1;
    })                                              // guard drops here → unlocked
};
handle.join().unwrap();
println!("{}", *counter.lock().unwrap());          // 1
```

**Compared with [`RefCell`](#refcell)**, which does the same job on one thread: `RefCell` **panics**
if the value is already borrowed (a second borrow is a bug); `Mutex` **waits** instead (queuing is the
normal case). `.lock()`'s `Err` means a thread **panicked while holding the lock**, so the value may
be half-updated — Rust calls that a *poisoned* mutex.

**Why this way.** The guard is the only route to the data, so you cannot read or write without
locking, and scope-based release means an early `return` or a panic still unlocks. Keep the guard's
scope short — other threads are blocked for exactly as long as it lives. Locking a mutex you already
hold blocks forever: a **deadlock**.

First seen in: [From-Zero concept 35 — `Arc<Mutex<T>>`](../from-zero/rust/35-arc-mutex/use-it.md)

## `mpsc::channel` — send values between threads {#mpsc-channel}

**In one line:** a one-way pipe between threads — one end sends owned values in, the other receives
them out in order, and **sending moves ownership**, so nothing is shared and no lock is involved.

**What it is.** `std::sync::mpsc::channel()` returns a `(Sender<T>, Receiver<T>)` pair. `mpsc` is
**m**ultiple **p**roducer, **s**ingle **c**onsumer: clone the `Sender` for as many sending threads as
you like; there is exactly one `Receiver`.

```rust
use std::sync::mpsc;
use std::thread;

let (sender, receiver) = mpsc::channel();

for id in 1..=3 {
    let sender = sender.clone();                 // one sending end per thread
    thread::spawn(move || sender.send(id).unwrap());
}
drop(sender);                                     // ⚠️ close the pipe: see below

for value in receiver {                           // blocks between values, ends when closed
    println!("{}", value);
}
```

- **`.send(v)`** moves `v` into the channel; it returns [`Result`](#result), `Err` only if the
  `Receiver` was dropped (and the `Err` hands your value back).
- **`.recv()`** blocks until a value arrives; `Err` means every `Sender` has been dropped.
- **`for x in receiver`** yields values until the channel closes. **`.try_recv()`** never blocks.

**The one trap.** The channel counts live senders; the loop ends only when that count hits **0**. If
the thread that owns the receiver also still holds a `Sender`, it waits forever — hence the
`drop(sender)` above (or let it fall out of scope first).

**Channel or [`Arc<Mutex<T>>`](#mutex)?** Send when the value can travel — pipelines, work queues,
collecting results; share behind a lock when threads must all read *and* write one piece of state.
The channel needs no lock in your code because ownership means the value belongs to exactly one place
at every instant.

First seen in: [From-Zero concept 36 — channels](../from-zero/rust/36-channels/use-it.md)

## `Send` and `Sync` — what may cross a thread {#send-sync}

**In one line:** two marker traits the compiler checks at every thread boundary — **`Send`** = this
value may *move* to another thread, **`Sync`** = a `&` to it may be *shared* with another thread.

**What they are.** Traits with no methods, present only so a bound can ask about them. The exact
relationship is worth memorising:

> **`T: Sync`** if and only if **`&T: Send`.**

They are **auto traits**: never `impl`ed, never `derive`d. The compiler grants them structurally —
a struct or enum is `Send` if **every field** is `Send`, `Sync` if every field is `Sync`. So you
never gain them by writing code, you only *lose* them to one bad field. They cost nothing at
runtime: `Rc<i32>` and `Arc<i32>` are both 8 bytes and identical in memory.

| type | `Send` | `Sync` | why |
|---|---|---|---|
| `i32`, `String`, `Vec<T>` | ✅ | ✅ | plain data |
| `&T` | ✅ *(if `T: Sync`)* | ✅ | sending a reference **is** sharing the value |
| [`Rc<T>`](#rc) | ❌ | ❌ | non-atomic owner count |
| [`RefCell<T>`](#refcell) | ✅ *(if `T: Send`)* | ❌ | non-atomic borrow flag |
| [`Arc<T>`](#arc) | ✅ *(if `T: Send + Sync`)* | ✅ *(same)* | atomic count |
| [`Mutex<T>`](#mutex) | ✅ *(if `T: Send`)* | ✅ *(same)* | the lock makes sharing safe |
| `MutexGuard<'_, T>` | ❌ | ✅ *(if `T: Sync`)* | must be unlocked by the locking thread |

**Where the check happens.** Nowhere special — it's [`thread::spawn`](#thread-spawn)'s own signature,
`F: FnOnce() -> T + Send + 'static, T: Send`. A [closure](#closures) is `Send` exactly when all its
captures are, which is why the error points at the closure and then at the offending field inside it.

**Ask the compiler about any type** in two lines — the bound is the whole test:

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

assert_send::<Arc<i32>>();        // fine
// assert_send::<Rc<i32>>();      // `Rc<i32>` cannot be sent between threads safely
// assert_sync::<RefCell<i32>>(); // `RefCell<i32>` cannot be shared between threads safely
```

**Reading the errors.** They differ by one word, and the word names the trait: **"cannot be sent"** =
`Send`, **"cannot be shared"** = `Sync`. `note: required because it appears within the type X` is the
auto-trait rule naming the field that lost it. The fix is to change the field, never to fight the
trait: `Rc` → `Arc`, `RefCell` → `Mutex`.

**Escape hatches.** `impl !Send for MyType {}` opts out; `unsafe impl Send for MyType {}` claims a
trait the compiler wouldn't grant and means *"I have verified this by hand"* — how `Arc` and `Mutex`
get theirs, since atomics and locks are beyond what the compiler can reason about.

First seen in: [From-Zero concept 37 — `Send` and `Sync`](../from-zero/rust/37-send-and-sync/use-it.md)

## `async` / `.await` — a function that can pause {#async-await}

**In one line:** `async` changes what a function *returns* — instead of running and giving you a `T`,
calling it builds a **future**: a paused function, holding its own locals, that runs nothing until
something drives it.

**The rewrite.** `async fn` is a return type in disguise. These are the same function:

```rust
async fn add_one(x: u32) -> u32 { x + 1 }
fn      add_one(x: u32) -> impl Future<Output = u32> { async move { x + 1 } }
```

So calling it executes no statement of the body. Forget to drive it and the compiler says so:
`warning: unused implementer of Future that must be used` / `futures do nothing unless you .await or
poll them`.

**`.await`** is a **suffix** — `brew().await`, never `await brew()` — so it chains with `?` and method
calls. It means "drive this future to completion, and pause *me* while it can't finish". It is only
legal inside an `async` fn or `async` block (`error[E0728]` otherwise), which is why every async
program has one plain function at the bottom running an [executor](#future-poll).

**What the compiler builds.** An [enum](#enum) with one variant per suspend point, each holding the
locals that must survive that pause:

```rust
enum BreakfastFuture {
    NotStarted,
    AtBrew  { brew_future: BrewFuture },
    AtToast { cup: String, toast_future: ToastFuture },
    Finished,
}
```

An enum is as big as its largest variant, so **what you hold across an `.await` is what your task
costs** — and you can measure it with `size_of_val`:

| | size |
|---|---|
| `async fn nothing() {}` | 1 byte |
| three `.await`s nested (`level_c` → `level_b` → `level_a` → `nothing`) | 4 bytes |
| a `[u8; 512]` that dies *before* the await | 16 bytes |
| the same array held *across* the await | 514 bytes |

Nesting means a whole call chain is **one flat struct**, sized at compile time — no stack, no
allocation. Compare a thread's 2 MiB stack, held whether it works or waits.

**Consequences worth knowing.**
- Futures are **lazy**: an undriven future simply never happens, and cancelling a task is *dropping*
  it — [ownership](#let-mut) frees whatever it was holding.
- An `async fn` that awaits itself needs a [`Box`](#box), same as any recursive type.
- Holding a `MutexGuard` across an `.await` makes the future non-[`Send`](#send-sync), so it can't be
  spawned — the guard becomes a field, and the field's missing trait propagates.
- Bare `rustc` defaults to the 2015 edition, where `async` isn't a keyword. Use
  `rustc --edition 2024 file.rs`; `cargo` sets an edition for you.

First seen in: [From-Zero concept 38 — `async` and `.await`](../from-zero/rust/38-async-and-await/use-it.md)

## `Future`, `poll`, and executors {#future-poll}

**In one line:** a future is a value you poke until it says it's done, and an **executor** is the
ordinary non-async loop doing the poking — hold two futures in that loop and you get concurrency on
one thread.

**The trait, in full.** One method:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- **`Poll<T>`** — an enum, `Ready(T)` or `Pending`. [`Option`](#if-let)'s shape, named for *time*.
- **`Pin<&mut Self>`** — a `&mut self` carrying a promise: *this value will never move again*. A
  paused future's fields can point at each other, so moving it after the first poll would dangle.
  `Unpin` is the opt-out for types that don't care; `async`-generated futures are the exception
  (`error[E0277]: ... cannot be unpinned`).
- **`cx`** — carries a [`Waker`](#future-poll), the callback meaning "poll me, I can progress now".

**Writing one by hand** — state in the fields, the decision in `poll`:

```rust
struct Pause { polls_left: u32 }

impl Future for Pause {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.polls_left == 0 { Poll::Ready(()) }
        else { self.polls_left -= 1; Poll::Pending }
    }
}
```

**A complete executor for one task:**

```rust
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);                           // park it at one address
    let mut context = Context::from_waker(Waker::noop());    // a waker that does nothing
    loop {
        match future.as_mut().poll(&mut context) {           // .as_mut() re-borrows the pin
            Poll::Ready(value) => return value,
            Poll::Pending => {}
        }
    }
}
```

`pin!` pins on the stack (free, frame-bound); `Box::pin` pins on the heap (one allocation, and the
task can then be queued) — which is why a spawned task is typically `Pin<Box<dyn Future + Send>>`.

**`.await` is sequential; the executor is what makes things concurrent.** Same two tasks, same one
thread, no `spawn` anywhere — only the driving loop differs:

```text
awaited in one task:              polled together in one loop:
  toast 1, toast 2, toast 3,        toast 1, eggs 1, toast 2,
  eggs 1, eggs 2                    eggs 2, toast 3
```

That is what `join!` and `tokio::spawn` do for you. Writing two `.await`s in a row and expecting them
to overlap is the commonest async bug in every language.

**Rules the loop must keep.**
- Never poll a future after it returned `Ready` — a generated future panics with
  `` `async fn` resumed after completion ``. Keep an `Option` per task to remember.
- The poll count is *pauses + 1*: a future must be asked once more to report it's finished.
- A future may return `Pending` **only after** arranging for `wake()` to be called. Break that and
  the task hangs forever.

**Why `Waker::noop()` spins.** It's a real waker whose `wake()` does nothing, so on `Pending` the loop
can only ask again — fine for a future that becomes ready by being asked, useless for real I/O. A real
runtime sleeps on `epoll`/`kqueue`/IOCP and is woken by the OS, then re-queues the task. Building a
`Waker` by hand needs `unsafe`, since it's a hand-written vtable — hence the safe stand-in.

**What `tokio` adds:** a real waker per task, a sleeping run queue, `spawn` for thousands of tasks,
and a work-stealing thread pool — which is why `spawn` demands `F: Future + Send + 'static`. Same
shape, industrial version.

First seen in: [From-Zero concept 39 — `Future`, `poll`, and the executor](../from-zero/rust/39-future-poll-and-the-executor/use-it.md)

## `unsafe` — the door out of the rules {#unsafe}

**In one line:** a permission slip for five operations the compiler cannot verify — and *nothing*
else; the borrow checker, ownership, lifetimes and types all keep running inside an `unsafe` block.

**The five superpowers.** This is the complete list:

1. dereference a raw pointer — `*ptr` ([raw pointers](#raw-pointers))
2. call an `unsafe fn` — `slice.get_unchecked(i)`, `String::from_utf8_unchecked`
3. read or write a `static mut`
4. implement an `unsafe trait` — `unsafe impl Send for MyType {}` ([`Send`/`Sync`](#send-sync))
5. access a `union` field

Anything else you were hoping it would allow, it does not:

```rust
let mut owner = 5;
let borrowed = &owner;
unsafe { owner += 1; println!("{borrowed}"); }
```

```
error[E0506]: cannot assign to `owner` because it is borrowed
warning: unnecessary `unsafe` block
```

**Two spellings, mirror meanings.**

```rust
unsafe { *pointer }                             // block:     I have checked the contract here.
unsafe fn get_unchecked(&self, i: usize) -> &T  // signature: calling me is a promise. Yours to keep.
```

Since edition 2024 an `unsafe fn` body is still ordinary safe code, so it needs its own inner
`unsafe { }`. Document every precondition under a `# Safety` / `// SAFETY:` note — an unwritten
contract is one nobody can keep.

**The safe-wrapper pattern** — the reason any of this is bearable. Write the unsafe once, then bury
it under an API nobody can misuse:

```rust
// SAFETY: `count <= values.len()`.
unsafe fn sum_first_unchecked(values: &[u32], count: usize) -> u32 { /* get_unchecked in a loop */ }

fn sum_first(values: &[u32], count: usize) -> Option<u32> {
    if count > values.len() { return None; }        // the check the core cannot do
    Some(unsafe { sum_first_unchecked(values, count) })
}
```

`Vec`, `String`, `Box`, `Rc`, `RefCell`, `Mutex`, channels and `Waker` are all this shape.

**Vocabulary that makes bugs describable.** *Safe* = callable without the keyword. *Unsafe* = has an
uncheckable precondition. *Sound* = cannot cause UB however it is called. **Unsound** = a **safe**
thing that can — `get_unchecked` is unsafe-and-sound; `sum_first` with its check deleted is
safe-and-unsound, and that is the real bug class.

**The audit boundary is the module, not the block.** An unsafe block leans on an invariant that
ordinary safe code maintains, so a safe `set_len`-style method can break it from outside the braces.
Keep those fields private and the module small.

**`static mut` is the door to avoid.** One copy for the whole program, writable by any thread, so a
reference to it is unsound by construction — a hard error in edition 2024:

```rust
static mut COUNT: u32 = 0;
unsafe { println!("{COUNT}") }   // error: creating a shared reference to mutable static
unsafe { *(&raw const COUNT) }   // ok — address, never a reference
```

Prefer `AtomicU32` or a [`Mutex`](#mutex).

**The keyword compiles to nothing.** `unsafe { *ptr }` and `*reference` emit the same load. There is
no runtime cost and no runtime protection — only a compile-time permission and a note that a human
checked.

**Undefined behaviour is not "it might crash."** It is "the compiler was allowed to assume this never
happens," so it optimizes on your promise — deleting the bounds check you wrote *after* an
unchecked read, for instance. The symptom lands far from the cause, in release builds only. Check
unsafe code with **Miri**: `cargo +nightly miri test`.

First seen in: [From-Zero concept 40 — `unsafe`](../from-zero/rust/40-unsafe/use-it.md)

## raw pointers — `*const T` / `*mut T` {#raw-pointers}

**In one line:** an address with every promise stripped off — the same 8 bytes as a
[reference](#borrow), minus the four things the compiler had proved about it.

**What a reference carries that a raw pointer doesn't.** A `&i32` is an address *plus* four proofs:
not null · correctly aligned · pointing at a live `i32` right now · no `&mut` to it exists at the
same time. Those proofs are what a lifetime and the borrow checker produce, and they're why `*r`
needs no keyword. A raw pointer has none of them, so every one becomes your promise.

**Making one is safe; dereferencing is unsafe.** Writing down a number can't hurt anyone; believing
it can.

```rust
let mut reading = 42;

let a: *mut i32   = &raw mut reading;      // preferred: address, no reference ever created
let b: *const i32 = &raw const reading;
let c: *const i32 = &reading;              // a reference coerces
let d: *const i32 = std::ptr::null();
let e: *const i32 = numbers.as_ptr();      // a Vec's buffer

println!("{a:p}");                          // safe
let value = unsafe { *b };                  // unsafe
```

Prefer `&raw mut` / `&raw const` over `&mut x as *mut _` — the old spelling creates a real reference
for an instant, which is occasionally the bug.

**What they allow that references can't:** two `*mut` to one place, null, dangling (no lifetime, so
no `E0597`), and arithmetic. `.add(n)` moves by **n elements, not n bytes** (`.byte_add()` is spelled
differently on purpose).

**Sizes are unchanged** — thin stays thin, fat stays fat:

| | size | |
|---|---|---|
| `*const i32` / `&i32` | 8 | address |
| `*const [i32]` / `&[i32]` | 16 | address + element count |
| `*const dyn Trait` / `&dyn Trait` | 16 | address + vtable |

Which is why `slice::from_raw_parts_mut(pointer, count)` takes two arguments: it assembles the two
words a `&mut [T]` is made of.

**The canonical use — `split_at_mut`.** Two `&mut` into one slice: the borrow checker must reject it
(it can't reason about the *value* of `middle`), and it is provably fine:

```rust
fn split_at_mut(values: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
    let length = values.len();
    let start = values.as_mut_ptr();     // the address, BEFORE either half exists
    assert!(middle <= length);           // the entire safety argument
    unsafe {
        (
            slice::from_raw_parts_mut(start, middle),
            slice::from_raw_parts_mut(start.add(middle), length - middle),
        )
    }
}
```

Soundness rests on the `assert!` *and* on the signature: elision ties both outputs to `values`, so
neither half can outlive the array. Drop the assert and `length - middle` underflows in release to
~18 quintillion.

**`&mut` uniqueness is still assumed.** While a `&mut T` exists nothing else may touch that memory —
including a raw pointer made earlier. The code generator was told `&mut` is unique (`noalias`) and
optimizes on it; going raw removed the *checking*, not the assumption.

**Alignment is UB, not slowness.** A `*const u32` at an odd address is undefined behaviour even
though the bytes are there — some machines fault, and the compiler picks instructions assuming
alignment. Use `ptr::read_unaligned` when you really need it.

**Provenance.** A pointer is an address *plus* which allocation it may reach. `pointer as usize as
*const T` compiles but discards that permission. Derive pointers from pointers (`.add`, `.offset`,
`.wrapping_add`); if an address must live in an integer, use `.addr()` / `.with_addr()`. Check it
all with Miri.

**`NonNull<T>` buys the niche back:** `Option<&T>` is 8 bytes (null *is* `None`), `Option<*const T>`
is 16, `Option<NonNull<T>>` is 8 again — which is why `Box`, `Vec` and `Rc` use `NonNull` internally
and `Option<Box<T>>` costs nothing extra.

First seen in: [From-Zero concept 41 — raw pointers](../from-zero/rust/41-raw-pointers/use-it.md)

## `fn main` — the program's entry point {#main}

**In one line:** the function the operating system calls when your program starts;
without it, `rustc` refuses to build a runnable file.

**What it is.** A `.rs` file full of functions is just a *library* — nothing in it
runs on its own, because nothing says where to begin. `fn main() { ... }` is that
starting line. When you run `rustc solution.rs && ./solution`, the binary starts at
the first statement inside `main`, runs to the closing brace, and exits.

```rust
fn main() {
    println!("this runs first");
}
```

**Why LeetCode files have none.** LeetCode's editor shows you only an `impl
Solution` block. Their judge pastes it into a bigger file that already has a `main`,
reads the test input, calls your method, and compares the output — you just never
see that half. Copy their half-file to your machine and `rustc` says
`` error[E0601]: `main` function not found ``, because the part that starts the
program was never yours.

So every solution in this repo writes the missing half itself: the
[unit struct](#unit-struct) they hide, plus a `main` that feeds the examples in and
checks what comes out. The `impl Solution` block in between stays exactly what you
paste back into LeetCode.

**No arguments, no return.** `main` takes nothing and normally returns nothing;
command-line arguments come from `std::env::args()` instead. It *may* return
`Result<(), E>`, in which case an `Err` exits with a failure code — handy when the
body uses [`?`](#question-mark).

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## unit structs — `struct Solution;` {#unit-struct}

**In one line:** a [struct](#struct) with no fields — a type that holds nothing and
exists only to hang functions off.

`struct Solution;` (note the semicolon, not `{}`) declares a type with zero data. On
its own it's useless. Paired with an [`impl` block](#impl) it becomes a **namespace**:

```rust
struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> { /* ... */ }
}

Solution::two_sum(vec![2, 7], 9);
```

Nothing here needs an *instance* — you never write `let s = Solution;`. The functions
take all their input as parameters and are called through the type name with `::`,
the way `HashMap::new()` is. That's why the value carries no fields: there's no state
to carry.

**Could you do without it?** Yes — a plain `fn two_sum(...)` at the top level works
identically and is what you'd write in real Rust. LeetCode's harness insists on the
`Solution::` spelling because its judge is written once for languages where every
function must live inside a class (Java, C#), so the Rust track copies that shape.
It's a house rule, not a Rust one.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `println!` — print a line {#println}

**In one line:** writes text plus a newline to the terminal, filling each `{}` hole
with a value.

```rust
let answer = vec![0, 1];
println!("two_sum = {:?}", answer);        // two_sum = [0, 1]

let name = "ana";
println!("hello {name}");                  // hello ana
```

**Three things to know:**

1. **`{}` versus `{:?}`.** `{}` asks for the *Display* form — the tidy, human
   version a type chooses to show (`77`, `ana`). `{:?}` asks for the *Debug* form —
   the programmer's version, which shows structure (`[0, 1]`, `Some("ana")`).
   Collections like `Vec` and wrappers like `Option` have **only** `Debug`, so
   printing one with `{}` fails to compile; that's why the harnesses use `{:?}`.
2. **Inline names.** If the value is a plain variable, you can put its name inside
   the braces — `println!("{name}")` instead of `println!("{}", name)`. Only a bare
   variable name works there; an expression like `stored.value` still goes in the
   argument list.
3. **It's a macro.** The `!` means `println!` is expanded at compile time, which is
   how it can check your holes against your arguments and reject a mismatch before
   the program ever runs. A normal function couldn't take a varying number of
   differently-typed arguments like that.

The same holes-and-values syntax powers [`format!`](#format) (returns a `String`
instead of printing) and the [format specifiers](#format-spec) like `{:.1}`.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `assert_eq!` — a check that stops the program {#assert-eq}

**In one line:** compares two values and, if they differ, crashes the program on the
spot with both values printed.

```rust
assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
```

If the two sides match, the line does nothing and execution moves on. If they don't,
the program **panics** — stops immediately, prints the file and line, and shows what
it got versus what it expected:

```
thread 'main' panicked at solution.rs:24:5:
assertion `left == right` failed
  left: [1, 2]
 right: [0, 1]
```

**Why the harness asserts instead of just printing.** A `println!` of a wrong answer
looks exactly like a `println!` of a right one — you have to read every line and
compare by eye. An `assert_eq!` makes the *expected* value part of the program: a
passing run prints its results calmly, and a broken solution can't slip past quietly,
because the run dies at the first mismatch with the failing case named.

That's also why the solutions carry no comments claiming what they return. A comment
saying "returns [0, 1]" is checked by nobody; `assert_eq!` is checked on every run.

**Relatives:** `assert!(condition)` for a plain true/false check, and `assert_ne!`
for "these must differ". Both sides of `assert_eq!` need
[`Debug`](#println) so the failure can be printed, and the two types must be
comparable with `==`.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `while let` — loop while a pattern still matches {#while-let}

**In one line:** keeps looping as long as a value keeps matching a pattern, and hands
you the contents each time round.

It's [`if let`](#if-let) with a loop instead of a branch. Walking a linked list is the
classic use — keep going while there's still a node, stop at the `None` end:

```rust
let mut node = head;
while let Some(current) = node {
    digits.push(current.val);
    node = current.next;
}
```

**Trace what happens.** `node` is an `Option<Box<ListNode>>` — a node, or the end of
the list. Each pass, `while let Some(current) = node` tries the pattern: if `node` is
a `Some`, `current` is bound to the node inside and the body runs; if it's `None`,
the pattern fails and the loop ends. The last line moves the loop on by making `node`
the next link.

**The `while` version is clumsier.** With a plain [`while`](#while) you'd write
`while node.is_some()` and then dig the value out by hand with
[`.unwrap()`](#unwrap) — a second step that can panic, guarding a condition you
already tested. `while let` does the test and the extraction in one move, and there's
no `unwrap` to get wrong.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `.fold()` — collapse an iterator into one value {#fold}

**In one line:** walks an iterator carrying a running value, and hands you that value
at the end.

```rust
let total = [1, 2, 3].iter().fold(0, |running_total, n| running_total + n); // 6
```

`.fold(start, |accumulator, item| ...)` takes two things: the value to start with,
and a [closure](#closures) that combines the running value with the next item. What
the closure returns becomes the running value for the next item. `.sum()` is just a
`.fold(0, |a, b| a + b)` with a name.

**Building a linked list backwards is the interesting case:**

```rust
digits
    .iter()
    .rev()
    .fold(None, |next, &digit| Some(Box::new(ListNode { val: digit, next })))
```

The running value here isn't a number — it's *the list built so far*. Start with
`None` (the empty tail), walk the digits [in reverse](#rev), and each step wraps the
list-so-far as the `next` of a fresh node. After the last (leftmost) digit, the
running value is the whole chain, head first.

**Why not a loop?** You can write the same thing with `let mut head = None;` and a
`for`, and it reads fine. `.fold` earns its place when you want the result as a
single expression with no mutable variable to accidentally use half-built — the whole
construction is one value that only exists finished.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `where` clauses — bounds moved below the signature {#where}

**In one line:** the same [generic](#generics) trait bounds, written under the
signature instead of inline, so the parameter list stays readable.

These two are identical to the compiler:

```rust
fn check<T: std::fmt::Debug + PartialEq<U>, U: std::fmt::Debug>(label: &str, actual: T, expected: U) {}

fn check<T, U>(label: &str, actual: T, expected: U)
where
    T: std::fmt::Debug + PartialEq<U>,
    U: std::fmt::Debug,
{
}
```

The first crams the requirements between the angle brackets, pushing the actual
parameters off the edge. The `where` form names the type parameters up top and lists
what each must be able to do below — one bound per line. Reach for it as soon as a
signature has more than one bound, or a bound with more than one trait in it.

**Reading the bounds above:** `T: PartialEq<U>` says the two sides must be comparable
even though they're *different* types — which is what lets a harness compare a
`Vec<String>` the code produced against the `vec!["a", "b"]` of `&str` literals you
typed. `Debug` on both is what [`assert_eq!`](#assert-eq) needs to print them if the
check fails.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `mod` — modules, paths and privacy {#modules}

**In one line:** a named box for items with a wall around it that is closed by
default — the thing `::` in every path has been separating all along.

```rust
mod weather {
    pub mod sensors {
        pub fn read_celsius() -> f64 { 21.5 }
        fn calibration_offset() -> f64 { 0.4 }      // private
    }

    fn label() -> &'static str { "hourly reading" }  // private

    pub fn report() -> String {
        format!("{}: {:.1}", label(), sensors::read_celsius())
    }
}

use weather::sensors::read_celsius;      // shortens a path; grants nothing
use weather::sensors as probe;           // renames it
pub use weather::sensors::read_celsius as reading;   // re-export
```

**The one privacy rule.** An item is visible to the module it is in *and every
module inside that one*, however deep. To anything else it exists only if marked
`pub`. So looking **up** the tree is always allowed, private items included;
looking **down** reaches only what said `pub`. Reaching a private item from
outside is `error[E0603]`, which is a different complaint from `E0425 cannot find
value` — E0603 means the compiler found it and won't let you use it.

**Three path roots:** `crate::a::b` from the crate root, `super::b` one module up,
`b` relative to here. Prefer relative paths for close neighbours so a subtree
survives being moved.

**`use` only nicknames.** It does not open anything (the item must already be
`pub`) and does not copy anything. `pub use` re-exports: the item gains a second,
shorter public name — how `std::collections::HashMap` is reachable at all.

**Visibility markers:**

| marker | reachable from |
|---|---|
| *(none)* | this module and its descendants |
| `pub` | anywhere, including other crates |
| `pub(crate)` | anywhere in this crate, invisible to dependents |
| `pub(super)` | the parent module and below |

**Fields are separate.** `pub struct` opens the type, not the fields — each field
needs its own `pub`. Leaving them off is how an invariant is enforced: writing to
a private field from outside is `E0616`, and building one with a struct literal is
`E0451`. `pub enum` is the exception: it makes every variant and their fields
public, since callers must be able to `match`.

**Files.** `mod weather;` (semicolon, no body) says the body is in `weather.rs`
beside the declaring file, or `weather/mod.rs`. The **`mod` line creates the
module; the file is only where its body is kept** — a `.rs` file no `mod` line
mentions is never compiled, checked, or warned about.

**It costs nothing.** Modules are resolved entirely at compile time: no lookup, no
allocation, no size. The only trace left is name mangling, where the path becomes
part of each symbol (`_RNvNtNt..6mangle7weather7sensors12read_celsius`) and of
`std::any::type_name`.

See also [`use` declarations](#use), [`pub fn`](#pub-fn).

First seen in: [From-Zero concept 42 — modules](../from-zero/rust/42-modules/use-it.md)
