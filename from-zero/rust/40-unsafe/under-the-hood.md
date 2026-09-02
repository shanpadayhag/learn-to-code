# Concept 40 · `unsafe` (the door out of the rules) — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## The keyword generates no code at all
Start with the surprise. Compile these two functions and compare the machine code:

```rust
fn through_a_reference(value: &u32) -> u32 {
    *value
}

fn through_a_pointer(value: *const u32) -> u32 {
    unsafe { *value }
}
```

They are **identical** — one load instruction each. `unsafe` is not a runtime mode, not a flag on a
value, not a slower path, not a checked path. Nothing is added and nothing is removed.

So what is it? A **compile-time permission**. The compiler keeps a list of five operations it refuses
to emit, and `unsafe` is how you take one off the list for a few lines. That is the entire mechanism.

Which explains the shape of the whole feature:

- There is no cost to `unsafe`, so "is this fast enough" is never a reason to reach for it — the safe
  and unsafe versions of a correct program usually compile to the same thing.
- There is no protection in `unsafe` either. The block does not watch you. It is a note in the source
  saying *a human checked this*, and the only thing enforcing it is the human.
- The check that was skipped is skipped at runtime forever. A wrong promise is not caught later; it
  is simply believed.

## Three words you now need
Programmers use *safe* and *unsafe* loosely and then can't describe the actual bug. The vocabulary is
worth ten minutes:

| word | means |
|---|---|
| **safe** | callable without the `unsafe` keyword |
| **unsafe** | requires the keyword — it has a precondition the compiler can't check |
| **sound** | *cannot* cause undefined behaviour, no matter how it is called |
| **unsound** | a **safe** thing that *can* cause undefined behaviour when called normally |

The interesting cell is the last one, because "unsafe" and "unsound" sound like synonyms and are
nearly opposites in practice:

- `slice.get_unchecked(i)` is **unsafe and sound**. It is marked, its contract is documented, and
  keeping the contract is the caller's declared job.
- `sum_first` with its bounds check deleted is **safe and unsound**. Nothing in its signature warns
  you, no caller writes the keyword, and calling it with a big number is undefined behaviour.

The second is the real bug class. Unsafe code is not dangerous because it exists; it is dangerous
when a **safe** wrapper around it fails to enforce the invariant it depends on.

## The audit boundary is the module, not the block
This follows straight from the above, and it is the piece that catches people out.

You'd think that after writing `unsafe`, only the lines between the braces need auditing. But an
unsafe block's correctness usually depends on an *invariant* — some fact about the data — and that
fact is maintained by ordinary safe code elsewhere:

```rust
pub struct Readings {
    values: Vec<u32>,
    live: usize,
}

impl Readings {
    pub fn set_live(&mut self, live: usize) {      // safe. no keyword. no warning.
        self.live = live;
    }

    pub fn get(&self, index: usize) -> u32 {
        if index < self.live {
            unsafe { *self.values.get_unchecked(index) }   // correct — IF live <= values.len()
        } else {
            0
        }
    }
}
```

The `unsafe` block is fine. It is guarded by a bounds check, and you could stare at it all day
without finding a flaw. The bug is `set_live`, which is safe, unmarked, and lets any caller write
`readings.set_live(999)` and turn the guarded read into an out-of-bounds one.

So the unit you have to audit is everything that can touch the invariant — in Rust, that means
everything with access to the private fields, which is the **module**. Hence the practical rules:

- Keep the fields an unsafe block relies on **private**.
- Keep the module **small**, so "everything that could break this" is a page you can actually read.
- Write the invariant down at the top of the struct, not just the contract at the `unsafe fn`.

`Vec` is exactly this shape: `ptr`, `len`, `cap` are private, and the invariant *"the first `len`
elements are initialized and `len <= cap`"* is what every one of its unsafe blocks leans on. A public
`set_len` would be an unsound API — which is why the real one, `Vec::set_len`, is an `unsafe fn`.

## Why undefined behaviour is not "it might crash"
The most expensive misunderstanding in systems programming. UB is not a category of *crash*; it is a
category of *assumption*.

The compiler optimizes by reasoning about what your program can do. Every unsafe contract you sign
becomes a fact it is entitled to use:

```rust
let value = unsafe { *values.get_unchecked(index) };   // you promised index < len
if index < values.len() {                              // ...so this is provably true
    slow_path();
}
```

Step by step, the way the optimizer reads it:

1. `get_unchecked(index)` was called, so `index < values.len()` — you said so.
2. Therefore `if index < values.len()` is always true.
3. Therefore the branch is dead. Delete it, keep only `slow_path()`.

Now break the promise. The load reads a neighbouring value — no crash, the memory is mapped — and
your guard is *gone*, because it was deleted at compile time on the strength of a promise you broke.
The symptom appears somewhere else entirely, only under `--release`, and possibly only on one
machine. This is why UB bugs have a reputation: the effect isn't near the cause, and the code you're
staring at in the debugger is not the code that ran.

Three sharp consequences worth memorizing:

- **UB is retroactive.** The optimizer may have used your promise *before* the line that breaks it,
  so misbehaviour can appear earlier in the program than the bug.
- **"It worked when I tested it" proves nothing.** A wrong program with no check is often a *working*
  program until the day the optimizer, the allocator, or the input changes.
- **There is no partial UB.** Once a program has any, the whole run is meaningless — you cannot trust
  the output before it either.

## You have been standing on this the whole track
Every abstraction in Phases 2–10 is a safe wrapper over a small unsafe core:

| what you used | the unsafe thing inside it |
|---|---|
| [`String` / `Vec`](../17-vec/use-it.md) | asks the allocator for a block, writes into uninitialized memory, copies bytes when it grows |
| [`Box`](../29-box/use-it.md) | allocates, then frees exactly once through a raw pointer |
| [`Rc`](../30-rc/use-it.md) | several owners of one value; the shared count is mutated through a raw pointer |
| [`RefCell`](../31-refcell/use-it.md) | hands out `&mut` from behind a `&`, guarded by a runtime flag instead of the compiler |
| [`Mutex`](../35-arc-mutex/use-it.md) | the same trick, guarded by an OS lock; the guard's `Drop` is what unlocks it |
| [`mpsc::channel`](../36-channels/use-it.md) | a queue written by one thread and read by another |
| [`Send` / `Sync`](../37-send-and-sync/use-it.md) | `unsafe impl` is superpower 4 — the auto-traits are *claims*, not proofs |
| [`Waker`](../39-future-poll-and-the-executor/use-it.md) | a hand-built vtable of function pointers, which is why `Waker::noop()` exists as the safe stand-in |

Read that list and Rust's actual claim comes into focus. It was never "no unsafe code" — a language
that cannot allocate memory cannot do anything. It is: **the unsafe code is finite, named, wrapped
and audited, and the millions of lines built on top of it cannot cause undefined behaviour at all.**

## The tool that checks your homework
Ordinary tests will not find UB, because a wrong program usually produces right answers on the run
where nothing has moved. **Miri** is an interpreter that executes your code while tracking what each
pointer is actually allowed to reach, and it reports the violation at the instruction that commits
it:

```bash
rustup +nightly component add miri
cargo +nightly miri test
```

```
error: Undefined Behavior: attempting a read access
       but the borrow stack does not contain a matching tag
```

It is slow — tens of times slower than native — and it only sees the paths your tests actually take.
That's still the difference between a bug found in a test run and a bug found in production two
months later. **If you write unsafe code, run it under Miri.**

## Predict the memory
```rust
static mut TOTAL: u32 = 0;

unsafe fn add(amount: u32) {
    unsafe { TOTAL += amount; }
}

pub struct Readings {
    values: Vec<u32>,
    live: usize,
}

impl Readings {
    pub fn set_live(&mut self, live: usize) { self.live = live; }
    pub fn get(&self, index: usize) -> u32 {
        if index < self.live { unsafe { *self.values.get_unchecked(index) } } else { 0 }
    }
}

fn main() {
    let mut owner = 5;
    let borrowed = &owner;
    unsafe { owner += 1; println!("{borrowed}"); }
}
```

1. Does the `unsafe` block in `main` compile? What two messages does the compiler print, and what
   does the *warning* tell you the keyword is for?
2. `add` is an `unsafe fn`. Where does `TOTAL` live — stack, heap, or somewhere else — and how many
   copies of it exist if three threads call `add`?
3. `Readings::get` is a safe function containing an `unsafe` block, and `set_live` is a safe function
   containing none. Which one is the bug, and what is the smallest region of code you must audit to
   be sure `get` is sound?
4. `through_a_reference(&x)` and `through_a_pointer(&x as *const u32)` both return `x`. How do their
   compiled instructions differ, and how do their *guarantees* differ?

<details>
<summary>Show the answer</summary>
<ol>
<li><strong>No — <code>error[E0506]: cannot assign to </code>owner<code> because it is borrowed</code>, plus <code>warning: unnecessary unsafe block</code>.</strong> The borrow checker never stopped running; <code>unsafe</code> unlocks five specific operations and mutating a borrowed local is not one of them. The warning is the compiler saying the block gave you nothing, because none of the five were used inside it.</li>
<li><strong>In the program's static data, not on any stack or heap.</strong> A <code>static</code> is baked into the executable and lives at one fixed address for the entire run, so there is exactly <strong>one</strong> copy no matter how many threads touch it. That is precisely the problem: three threads calling <code>add</code> concurrently is a data race — undefined behaviour, not merely a wrong total. The keyword is what makes you notice. Use an <code>AtomicU32</code> or a <code>Mutex</code> instead.</li>
<li><strong><code>set_live</code> is the bug</strong>, even though it contains no <code>unsafe</code>. It can set <code>live</code> past <code>values.len()</code>, turning <code>get</code>'s bounds check into a check against the wrong number and its <code>get_unchecked</code> into an out-of-bounds read. And the region you must audit is <strong>the whole module</strong>, because everything with access to the private fields can break the invariant the unsafe block depends on. Fix it by making <code>set_live</code> clamp to <code>values.len()</code>, or by marking it <code>unsafe fn</code> and documenting the contract.</li>
<li><strong>The instructions are identical</strong> — one load each; <code>unsafe</code> emits nothing. The guarantees are not: for <code>&amp;u32</code> the compiler <em>proved</em> the address is non-null, aligned and pointing at a live <code>u32</code> for the whole of the reference's lifetime, whereas for <code>*const u32</code> it proved nothing and you promised all four. Same instruction, entirely different amount of knowledge behind it — which is exactly what the next lesson is about.</li>
</ol>
</details>

## Next
- The keyword's biggest door, opened properly: **raw pointers** (`*const T` / `*mut T`) — the address
  with all four promises stripped off. You'll see why making one is safe while reading one is not,
  and write `split_at_mut`: the function the borrow checker must reject and the standard library
  provides safely anyway.
