# Concept 40 · `unsafe` (the door out of the rules) — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 39](../39-future-poll-and-the-executor/use-it.md)

## The idea
Every concept in this track has been a rule the compiler enforces. Values have one owner. A borrow
can't outlive what it points at. Two `&mut` to the same thing can't exist at once. Thirty-nine
lessons of *no*.

And yet [`Vec`](../17-vec/use-it.md) grows by asking the operating system for a fresh block of memory
and copying bytes into it. [`Rc`](../30-rc/use-it.md) hands out several owners of one value and
mutates a counter that all of them share. [`Mutex`](../35-arc-mutex/use-it.md) hands you a `&mut` to
a value living behind a `&`. Not one of those can be proved correct by the rules above — the rules
would forbid every one of them.

So somebody wrote them by hand and checked them by eye. `unsafe` is the keyword that says so.

Here is the thing almost everyone gets wrong on first meeting, and it is worth being blunt about:

> **`unsafe` does not turn off the borrow checker.** It does not turn off ownership, moves,
> lifetimes, types, or `Drop`. It unlocks exactly **five** extra abilities and changes *nothing* else.

![Three panels: the five abilities unsafe unlocks beside the longer list of rules that stay on; the safe-wrapper sandwich of a safe API over a few audited unsafe lines over raw memory; and an optimizer deleting a bounds check because the programmer promised the index was in range](diagrams/unsafe-door.svg)

## The five superpowers
That's the entire list. Read it once and you know what the keyword is for:

| # | ability | where you've already met it |
|---|---|---|
| 1 | **dereference a raw pointer** — `*ptr` | [Concept 41](../41-raw-pointers/use-it.md), next |
| 2 | **call an `unsafe fn`** | `slice.get_unchecked(i)`, `String::from_utf8_unchecked` |
| 3 | **read or write a `static mut`** | a mutable global — see the warning below |
| 4 | **implement an `unsafe trait`** | `unsafe impl Send for MyType {}` ([Concept 37](../37-send-and-sync/use-it.md)) |
| 5 | **access a `union` field** | C interop, and almost nowhere else |

Superpower 1 is the big one, and it gets the whole of the next lesson. The others are small doors.

## Proving the rules are still on
Don't take it on trust. Try to break an ordinary rule inside an `unsafe` block:

```rust
let mut owner = 5;
let borrowed = &owner;
unsafe {
    owner += 1;
    println!("{borrowed}");
}
```

```
error[E0506]: cannot assign to `owner` because it is borrowed
warning: unnecessary `unsafe` block
```

Two messages, and the *warning* is the better lesson. The borrow error fires exactly as it would
outside the block — and the compiler additionally points out that the block bought you nothing,
because you never used one of the five. `unsafe` is not a mode. It is a permission slip for five
specific operations.

## The two spellings
The keyword appears in two places and means the mirror image of itself in each:

```rust
unsafe { *pointer }                            // I have checked the contract here.

unsafe fn get_unchecked(&self, i: usize) -> &T // Calling me is a promise. Yours to keep.
```

- **`unsafe { … }`** — a *block*. You are asserting: I have verified by hand what the compiler
  cannot. Keep it small; it is an audit boundary, and everything inside it is on you.
- **`unsafe fn`** — a *signature*. You are declaring: this function has a precondition I cannot
  check, so callers must prove it. Since edition 2024 the body of an `unsafe fn` is still ordinary
  safe code, so it needs its own inner `unsafe { }` — the two meanings really are separate.

Every `unsafe fn` in the standard library documents its precondition under a `# Safety` heading, and
that convention is not optional politeness. **An unwritten contract is one nobody can keep.** Write
yours down:

```rust
// SAFETY (the caller must guarantee):
//   `count` is less than or equal to `values.len()`.
unsafe fn sum_first_unchecked(values: &[u32], count: usize) -> u32 {
    let mut total = 0;
    for index in 0..count {
        total += unsafe { *values.get_unchecked(index) };
    }
    total
}
```

## The pattern that makes it bearable: the safe wrapper
`unsafe` is not something you sprinkle through a program. It is something you write **once**, in a
few lines, and then bury under an API nobody can misuse:

```rust
fn sum_first(values: &[u32], count: usize) -> Option<u32> {
    if count > values.len() {
        return None;
    }
    Some(unsafe { sum_first_unchecked(values, count) })
}
```

Four lines. The wrapper checks the one thing the core cannot check, so the contract is kept **by
construction** and no caller can break it. `sum_first` is a completely safe function — call it with
any slice and any number and the worst you get is `None`.

That sandwich is the whole design of Rust's standard library:

```text
        v.push(x)   s.len()   rc.clone()   m.lock()      ← safe API, millions of callers
                          │
             a few lines of audited unsafe                ← written once, reviewed by a human
                          │
              raw memory · the OS · the CPU
```

You have been standing on unsafe code since [Concept 07](../07-the-heap-and-string/use-it.md). You
just never had to look at it.

## The one superpower to avoid
`static mut` — a global you can write to — is the door with a hole in the floor. Any thread can write
it at any moment, so a *reference* to one is unsound the instant it exists, and edition 2024 makes
that a hard error rather than a warning:

```rust
static mut LAUNCH_COUNT: u32 = 0;

unsafe { println!("{LAUNCH_COUNT}") }
```

```
error: creating a shared reference to mutable static
  = note: it's undefined behavior if the static is mutated while the shared reference lives
```

Read the value through its address instead — `unsafe { *(&raw const LAUNCH_COUNT) }` — or, far
better, reach for `AtomicU32` or a [`Mutex`](../35-arc-mutex/use-it.md), which give you a mutable
global with none of this.

## The price: undefined behaviour
Break a promise you made and you do not get a crash. You get **undefined behaviour**, and the phrase
means something much more specific than "something bad happens":

> The compiler was *allowed to assume this never happens*, and it optimized your program on that
> assumption.

Say you call `get_unchecked(i)` and then check the index afterwards. The compiler reasons: calling
that function was a promise that `i` is in bounds, so the later `if i < v.len()` can never be false —
and deletes the check. Your bug is now in a line you never wrote, in a branch that no longer exists,
and it only appears in release builds.

This is why undefined behaviour cannot be debugged the normal way, and why the safe wrapper matters
so much. Rust's bargain is not "no unsafe code." It is **"unsafe code lives in small, audited,
wrapped places, and the other 99% of the program cannot cause it."**

> Quick reference: [`unsafe`](../../../languages/rust.md#unsafe) in the handbook.

## Exercises
```bash
rustc --edition 2024 1-solution.rs && ./1-solution
```

1. **The keyword does less than you think** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs).
   Use three of the five superpowers, then try to break a borrow rule inside an `unsafe` block.
   Predict the messages first: you get one error *and* one warning, and the warning is the point.
2. **Wrap it in a safe API** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs).
   Write an `unsafe fn` with a written contract plus the safe wrapper that keeps it, then delete the
   wrapper's check and watch a *safe* function become unsound. The bug lands in the safe code, not
   the unsafe block — which is the most important thing in this lesson.

## Next
- What the compiler actually does with the keyword (spoiler: it emits identical instructions), the
  difference between *safe*, *unsound* and *unsafe*, why the audit boundary is the module rather than
  the block, and the tool that catches UB for you: [Under the hood](under-the-hood.md).
- Then superpower 1 in full: **raw pointers** — an address with every promise stripped off, and the
  thing `Vec`, `Rc` and `split_at_mut` are all actually made of.
