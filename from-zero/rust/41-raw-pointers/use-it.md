# Concept 41 · Raw pointers (`*const T` · `*mut T`) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 40](../40-unsafe/use-it.md)

## The idea
[Concept 40](../40-unsafe/use-it.md) listed five superpowers and said the first one — dereferencing a
raw pointer — was the big one. Here it is.

Go back to [Concept 10](../10-borrowing-with-ref/use-it.md). A reference, you learned, is *an
address*: eight bytes holding the location of something else. That was true, and it was half the
story. A `&i32` is an address **plus four things the compiler proved on your behalf**:

1. it is **not null**
2. it is **correctly aligned** for an `i32`
3. it points at a **live `i32`**, right now, for as long as the reference exists
4. **no `&mut`** to that same `i32` exists at the same time

Those four proofs are what a lifetime and the borrow checker are *for*. And they are the reason `*r`
needs no keyword: there is nothing left to go wrong.

A **raw pointer** is the same eight bytes with all four thrown away.

```rust
let mut reading = 42;

let writable: *mut i32 = &raw mut reading;      // an address. that is all it is.
let readable: *const i32 = writable;

println!("{writable:p}");                        // safe — it's just a number
let value = unsafe { *readable };                // unsafe — now you're believing it
```

Note where the keyword falls, because it is the whole design of the feature:

> **Making a raw pointer is safe. Dereferencing one is unsafe.** Writing down a number can't hurt
> anybody. *Believing* a number — reaching into memory at that address and treating what's there as
> an `i32` — is where every promise above gets cashed in.

You can build a null pointer, a dangling pointer, a pointer into the middle of nothing, all in
ordinary safe code. Nothing happens. The moment of danger is the `*`.

![Three panels: a reference's four compiler-proved promises beside a raw pointer with all four stripped off; split_at_mut taking one address before either half exists and building two non-overlapping mutable windows from it; and the four ways a dereference goes wrong — null, unaligned, dangling, aliasing a &mut](diagrams/raw-pointers.svg)

## The two types, and how to make one
There are exactly two, and the difference is the same one you already know from `&` and `&mut`:

| type | means |
|---|---|
| `*const T` | an address you intend to read from |
| `*mut T` | an address you intend to write through |

Neither is enforced — you can cast between them freely — so they're documentation with teeth rather
than a guarantee. Four ways to get one:

```rust
let mut reading = 42;

let a: *mut i32   = &raw mut reading;          // address of a local, no reference made
let b: *const i32 = &raw const reading;        // the read-only version
let c: *const i32 = &reading;                  // a reference coerces into a pointer
let d: *const i32 = std::ptr::null();          // an address of nothing at all

let numbers = vec![10, 20, 30];
let start: *const i32 = numbers.as_ptr();      // the first element of a Vec's buffer
```

Prefer `&raw const` / `&raw mut`. They take an address **without ever creating a reference**, so the
borrow checker never gets involved and you never briefly hold a `&mut` you weren't entitled to. (The
old spelling, `&mut x as *mut i32`, makes a real reference for an instant — usually fine, occasionally
the bug.)

## What they let you do that references can't
This is the point. All four of these are ordinary, and all four are things the borrow checker exists
to forbid:

```rust
let mut reading = 42;
let first: *mut i32 = &raw mut reading;
let second: *mut i32 = first;                  // TWO writers to one place. try that with &mut.

let nothing: *const i32 = std::ptr::null();    // null. a reference can never be null.

let dangling: *const i32 = {
    let temporary = 7;
    &raw const temporary                       // outlives what it points at. no lifetime error.
};

let readings = [10, 20, 30, 40];
let start = readings.as_ptr();
let third = unsafe { *start.add(2) };          // arithmetic. `&` has no `+`.
```

`.add(n)` moves by **`n` elements, not `n` bytes** — `start.add(2)` on a `*const i32` steps 8 bytes,
because that's two `i32`s. This is how every collection in the standard library walks its buffer, and
getting the unit wrong is a classic C bug that Rust's API quietly removes.

## The payoff: `split_at_mut`
Here is a function that *must* be rejected by the borrow checker and is *obviously* fine:

```rust
fn split_at_mut(values: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
    (&mut values[..middle], &mut values[middle..])
}
```

```
error[E0499]: cannot borrow `*values` as mutable more than once at a time
```

The compiler is not being fussy. To see that `..middle` and `middle..` never overlap it would have to
reason about the *value* of `middle` at runtime, and that is not something a type system does. It
sees one slice, borrowed mutably, twice. Rejected.

But *you* can see the proof. So write it down, take the blame, and hand back the two halves:

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
    let length = values.len();
    let start = values.as_mut_ptr();
    assert!(middle <= length);

    unsafe {
        (
            slice::from_raw_parts_mut(start, middle),
            slice::from_raw_parts_mut(start.add(middle), length - middle),
        )
    }
}
```

Read it in the order it happens:

1. **`values.as_mut_ptr()`** takes the address *before either half exists*. Both halves descend from
   this one pointer, not from each other — so at no moment do two `&mut` overlap.
2. **`assert!(middle <= length)`** is not tidiness. It is the **entire safety argument**, written as
   a runtime check because the compiler couldn't do it at compile time. Delete it and this safe
   function becomes able to cause undefined behaviour.
3. **`from_raw_parts_mut(pointer, count)`** builds a real `&mut [i32]` out of an address and a
   length. It is an `unsafe fn` because it invents a slice — and a lifetime — from a number you
   handed it, and it will believe whatever you say.
4. The windows are `0..middle` and `middle..length`. **Disjoint**, so no element is ever reachable
   through both. That sentence is the proof the compiler wanted and couldn't construct.

And now the whole shape from Concept 40 is in one function: a **safe** signature, a **written**
invariant, an **assert** enforcing it, and three lines of audited unsafe underneath. This is
`slice::split_at_mut` in the standard library, near enough line for line.

## The promises are yours now
Every time you write `*pointer`, you are asserting all four:

- **not null** — `ptr::null()` derefs are the classic segfault, and the friendly outcome.
- **aligned** — a `*const u32` must sit at a multiple of 4. Cast a `*const u8` at an odd address to
  `*const u32` and reading it is undefined behaviour *even though the bytes are there*.
- **live, and the right type** — the value must still exist, and actually be a `T`.
- **not aliasing someone's `&mut`** — if a `&mut` exists, the compiler assumes nothing else can see
  that memory and optimizes accordingly. Writing through a raw pointer behind its back is undefined
  behaviour, even if the write "works".

The first one usually crashes, which is the lucky case. The other three tend to produce a plausible
wrong number, in release builds only, months later.

> Quick reference: [raw pointers](../../../languages/rust.md#raw-pointers) in the handbook.

## Exercises
```bash
rustc --edition 2024 1-solution.rs && ./1-solution
```

1. **Two names for one address** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Prove a reference and a raw pointer are the same size, hold three `*mut` to one variable at once,
   walk an array with `.add()`, then build a null pointer and a dangling one and notice how much
   `unsafe` you needed to *create* them.
2. **`split_at_mut` by hand** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Watch the honest version fail with `E0499`, write the raw-pointer version, mutate both halves at
   once, then delete the `assert!` and work out what `length - middle` becomes in a release build.

## Next
- Why a `*const [i32]` is 16 bytes and a `*const i32` is 8, what the compiler actually loses when you
  go raw, why alignment is undefined behaviour rather than a slow read, and the one idea — provenance
  — that explains why casting a `usize` back into a pointer is not the same pointer:
  [Under the hood](under-the-hood.md).
