# Concept 41 · Raw pointers (`*const T` · `*mut T`) — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## At runtime there is no difference at all
Measure them:

```rust
println!("{}", size_of::<&i32>());          // 8
println!("{}", size_of::<*const i32>());    // 8
println!("{}", size_of::<&[i32]>());        // 16
println!("{}", size_of::<*const [i32]>());  // 16
println!("{}", size_of::<&dyn Trait>());    // 16
println!("{}", size_of::<*const dyn Trait>()); // 16
```

A reference and a raw pointer to the same type are byte-for-byte the same value in memory. Every
difference you read about in the last lesson is **knowledge held by the compiler**, and knowledge
weighs nothing:

```text
   the stack

   reading:   ┌────────────┐
              │     42     │  ← an i32, at 0x7ff7b8a6dd14
              └────────────┘
                    ▲
   writable:  ┌─────┴──────┐
              │0x7ff7b8a6d…│  ← 8 bytes. exactly what a &mut i32 would hold.
              └────────────┘
```

Notice the pairs of sizes above, too — you already met this in
[Concept 12](../12-slices/use-it.md) and [Concept 21](../21-trait-objects/use-it.md). Some pointers
are **thin** (just an address) and some are **fat** (an address plus a second word), and going raw
does not change which is which:

| pointer | contents | size |
|---|---|---|
| `*const i32` | address | 8 |
| `*const [i32]` | address **+ element count** | 16 |
| `*const dyn Trait` | address **+ vtable address** | 16 |

Which is why `slice::from_raw_parts_mut(start, middle)` takes *two* arguments. It isn't inventing
extra bookkeeping; it is literally assembling the two words a `&mut [i32]` is made of.

## What the compiler stops doing for you
Going raw isn't only losing checks. Two of the four promises are things the compiler *acts on*, and
that's where the surprises live.

**It stops tracking lifetime, so nothing notices a dangling pointer.**

```rust
let dangling: *const i32 = {
    let temporary = 7;
    &raw const temporary        // compiles. no error, no warning.
};
```

Write that with `&temporary` and you get `error[E0597]: 'temporary' does not live long enough`,
because a reference carries a lifetime and the compiler checks it. A raw pointer carries no lifetime
at all — there is nothing to check, so nothing is checked. The stack slot gets reused by the next
call, and `unsafe { *dangling }` reads whatever landed there. Often it still prints `7`, because
nothing has overwritten the slot yet. That is the worst possible outcome: a test that passes.

**It keeps assuming `&mut` is unique, and optimizes on it.**

This is the one that catches experienced programmers. Rust tells the code generator that a `&mut T`
is the *only* way to reach that memory (LLVM calls it `noalias`), which licenses it to keep the value
in a register across code it can see doesn't touch it:

```rust
fn double(value: &mut i32, sneaky: *mut i32) {
    let before = *value;          // may be loaded into a register once...
    unsafe { *sneaky = 99; }      // ...and this write is invisible to that assumption
    *value = before * 2;          // so this may use the STALE `before`
}
```

Call it with both arguments pointing at the same `i32` and the program is undefined behaviour — not
because a rule was written down somewhere, but because the machine code was generated on a
promise you broke. The raw pointer didn't remove the assumption; it removed the *checking* of it.

So the rule is sharper than "don't alias":

> While a `&mut T` exists, **nothing else may touch that memory** — including a raw pointer you made
> before the reference existed. `split_at_mut` is careful about this on purpose: it takes the address
> *first*, then never uses it again once the two `&mut` slices are built.

## Alignment: why it's undefined behaviour and not a slow read
An `i32` must live at an address divisible by 4; a `u64` at a multiple of 8. Take a byte buffer,
point four bytes in at an odd offset, and read a `u32`:

```rust
let bytes: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
let odd = unsafe { bytes.as_ptr().add(1) } as *const u32;
let value = unsafe { *odd };        // undefined behaviour
```

The bytes exist. You own them. On x86 it will even give you a number. It's still UB, for two separate
reasons worth keeping apart:

- **Some machines fault.** On many ARM configurations and for most SIMD instructions an unaligned
  load is a hardware trap, not a slow path. "It worked on my laptop" means x86 tolerated it.
- **The compiler assumes alignment before the code runs.** Knowing a `*const u32` is 4-aligned lets
  it pick wider instructions, vectorize a loop, or split a copy into aligned chunks. Break the
  assumption and the *generated code* is wrong, whatever the CPU would have tolerated.

If you genuinely need an unaligned read, there is a safe-by-construction way to ask for one:
`ptr::read_unaligned`. It compiles to a byte-wise load, costs a little, and is correct everywhere.

## Provenance — a pointer is more than its address
The last idea, and the one that explains a rule that otherwise looks arbitrary.

You'd think a pointer is just a number, so this should be a round trip:

```rust
let value = 42;
let pointer = &raw const value;
let address = pointer as usize;             // fine
let back = address as *const i32;           // compiles fine
let read = unsafe { *back };                // NOT guaranteed to be valid
```

It isn't. A pointer in Rust (and in C, and in LLVM) is **an address plus a permission**: which
allocation it was derived from, and what it's allowed to reach. That second half is called
**provenance**, it exists only at compile time, and casting through `usize` throws it away.

Why the compiler cares: provenance is how it knows two pointers *can't* touch the same thing. Two
pointers derived from different allocations can never alias, so their loads and stores can be
reordered freely. If any `usize` could be turned back into a pointer to anything, that reasoning
collapses and a great deal of optimization goes with it.

In practice: **derive pointers from pointers, never from integers.** Use `.add()`, `.offset()`,
`.wrapping_add()` to move within one allocation — they carry provenance with them. If you must stash
an address in an integer (tagged pointers, hash keys), the modern spelling is `.addr()` to get the
number out and `.with_addr()` to put one back into a pointer that still has provenance. And run it
under **Miri**, which models provenance exactly and will tell you the moment you step outside an
allocation's permission.

## Why `split_at_mut` is actually sound
Put the argument in memory terms, because "the halves don't overlap" is the conclusion, not the
proof:

```text
   values: &mut [i32]   →   ┌────┬────┬────┬────┬────┬────┐
                            │  1 │  2 │  3 │  4 │  5 │  6 │
                            └────┴────┴────┴────┴────┴────┘
                              ▲                   ▲
                            start              start.add(3)

   left  = from_raw_parts_mut(start,        3)   covers bytes  0..12
   right = from_raw_parts_mut(start.add(3), 3)   covers bytes 12..24
```

Four things have to hold, and each maps to a line of the function:

1. **`start` is valid for the whole buffer.** It came from `values.as_mut_ptr()`, and `values` is a
   live `&mut [i32]` for the length of the call.
2. **Both ranges stay inside it.** `assert!(middle <= length)` gives `middle + (length - middle) ==
   length`. Remove the assert and `length - middle` underflows in release to about 18 quintillion,
   and `right` claims most of the address space.
3. **The ranges are disjoint.** `0..middle` and `middle..length` share no element, so no `i32` is
   reachable through both `&mut`.
4. **The lifetimes are honest.** `from_raw_parts_mut` invents a lifetime from nothing, but the
   signature ties both outputs back to `values`, so the borrow checker keeps the original slice
   locked for exactly as long as either half lives. The unsafe code creates the halves; the *safe*
   signature is what stops them outliving the array.

Point 4 is the quiet one. The unsafe block would be equally happy returning `&'static mut [i32]`, and
that would be catastrophic. What makes the function sound is as much the type you wrote in the
signature as the code inside the braces.

## The bonus you get back: `NonNull` and the niche
One promise is worth handing back to the compiler voluntarily. `NonNull<T>` is a `*mut T` that
is never null, and because the compiler knows that, it can use address zero as a spare value:

```rust
size_of::<&i32>()                  // 8
size_of::<Option<&i32>>()          // 8   ← None IS the null address. free.
size_of::<*const i32>()            // 8
size_of::<Option<*const i32>>()    // 16  ← a raw pointer might be null, so a tag word is needed
size_of::<NonNull<i32>>()          // 8
size_of::<Option<NonNull<i32>>>()  // 8   ← the promise buys the trick back
```

That's the **niche optimization**, and it's why `Option<Box<T>>` costs the same as `Box<T>` —
something you might have wondered about back at [Concept 29](../29-box/use-it.md). It's also why
`Box`, `Vec` and `Rc` are built on `NonNull` rather than `*mut T` internally: the invariant is true
anyway, so declaring it is free money.

## Predict the memory
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

fn main() {
    let dangling: *const i32 = {
        let temporary = 7;
        &raw const temporary
    };

    let mut readings = [1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut readings, 3);
    left[0] = 10;
    right[0] = 40;
    println!("{readings:?}");
}
```

1. The `dangling` line compiles with no error and no warning. Which of the four promises is broken,
   and why does the compiler not catch it when `&temporary` in the same position would be a hard
   error?
2. `readings` is `[i32; 6]` on the stack. How many bytes apart are `start` and `start.add(3)`, and
   how many *elements*?
3. `left` and `right` are two live `&mut` into one array. What stops them from outliving `readings`,
   given that `from_raw_parts_mut` invents a lifetime out of nothing?
4. Delete the `assert!` and call `split_at_mut(&mut readings, 99)`. What happens in a debug build,
   what happens in a release build, and which of those two is the more dangerous outcome?

<details>
<summary>Show the answer</summary>
<ol>
<li><strong>Promise 3 — "points at a live value."</strong> <code>temporary</code> dies at the closing brace, so the pointer outlives its target. The compiler doesn't catch it because <strong>a raw pointer carries no lifetime</strong>; there is no <code>'a</code> in <code>*const i32</code> for the borrow checker to relate to the block. With <code>&amp;temporary</code> the type would be <code>&amp;'a i32</code>, the region would be the inner block, and you'd get <code>error[E0597]: 'temporary' does not live long enough</code>. Same address, same bug, entirely different amount of compiler knowledge — and <code>unsafe { *dangling }</code> will very often still print <code>7</code>, because nothing has reused the slot yet.</li>
<li><strong>12 bytes, 3 elements.</strong> <code>.add(n)</code> counts in elements, so <code>.add(3)</code> on a <code>*mut i32</code> moves <code>3 * size_of::&lt;i32&gt;() = 12</code> bytes. This is the unit that C gets wrong constantly; Rust's API makes it impossible to mean bytes by accident (that's <code>.byte_add()</code>, spelled differently on purpose).</li>
<li><strong>The signature.</strong> <code>fn split_at_mut(values: &amp;mut [i32], …) -&gt; (&amp;mut [i32], &amp;mut [i32])</code> elides to one lifetime shared by the input and both outputs, so the borrow checker keeps <code>readings</code> mutably borrowed for as long as either half is alive. The unsafe block <em>creates</em> the halves; the safe signature is what makes them sound. Had it returned <code>&amp;'static mut [i32]</code>, the code inside would be unchanged and the function would be catastrophically wrong.</li>
<li><strong>Debug: a panic</strong> — <code>attempt to subtract with overflow</code> on <code>length - middle</code>, an unrelated check that happens to fire first. <strong>Release: silence</strong> — overflow checks are off, <code>6 - 99</code> wraps to about 18 quintillion, and you get a <code>&amp;mut [i32]</code> claiming most of the address space; the first write through it is undefined behaviour. <strong>The release build is far more dangerous</strong>, and the debug panic is actively misleading: it looks like the language caught the bug, when it caught a different symptom by luck. This is why the <code>assert!</code> is written as a real runtime check rather than left to arithmetic.</li>
</ol>
</details>

## Next
- That's the floor. From [Concept 01](../01-a-number-in-a-variable/use-it.md) — a number in a stack
  slot — to a `*mut i32` that is the same eight bytes with every promise stripped off, and the
  argument for why handing those promises to a compiler was worth it.
  [`Vec`](../17-vec/use-it.md), [`Box`](../29-box/use-it.md), [`Rc`](../30-rc/use-it.md),
  [`RefCell`](../31-refcell/use-it.md), [`Mutex`](../35-arc-mutex/use-it.md) and the
  [executor](../39-future-poll-and-the-executor/use-it.md) are all raw pointers plus an invariant plus
  a safe wrapper. You can now read every one of them.
- Phases 1–11 answered *where does this value live, and who owns it?* What's left is the
  **macro-phases** — how a program is organized rather than how a value is stored: modules and
  crates, error types beyond [`Result`](../23-result/use-it.md), and testing. Firmed up as we reach
  them.
