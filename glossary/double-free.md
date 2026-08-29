# Double Free

**In one line:** telling the system "I'm done with this piece of memory" **twice**,
which quietly corrupts the memory system and is one of the nastiest, most exploitable
bugs in programming.

## Plain explanation
When a program needs memory on the [heap](../from-zero/rust/07-the-heap-and-string/under-the-hood.md)
— say, to hold a piece of text — it doesn't just grab it. It **asks** a part of the
system called the **allocator**. Think of the allocator as a storage-locker front desk:

- You ask for room; the desk finds a free locker — say **#42** — hands it to you, and
  writes in its ledger *"locker 42 is taken."*
- The pointer your program holds is just a **ticket** with `42` on it.
- **Freeing** the memory means handing the ticket back: *"I'm done with 42."* The desk
  crosses it out — *"42 is free again"* — so it can give that locker to the next
  customer who asks.

That whole system only works if each locker is returned **exactly once**. A **double
free** is returning the ticket for locker 42 a *second* time — after the desk has
already marked it free and possibly handed it to someone else.

Here's the trouble, step by step, when two variables wrongly hold a ticket to the same
locker:

| step | what happens | the ledger |
|---|---|---|
| 1 | the first variable is done → returns ticket 42 | 42 → **free** |
| 2 | some *other* value asks for a locker; the desk hands out 42 and stores their data | 42 → taken (by someone new) |
| 3 | the second variable is done → returns ticket 42 **again** | 42 → "free"… but it's in use! |

After step 3 the desk's records are **lying** — and everything downstream trusts those
records.

## Why you care (why it's dangerous)
- **It corrupts the allocator's own bookkeeping.** The desk tracks free lockers in an
  internal list. Freeing the same locker twice can put it in that list *twice*, so the
  list loops or points into live data. The *next* innocent memory request your program
  makes can then be handed a garbage or overlapping locker.
- **Two live values silently share one locker.** After step 2, both the new value and the
  old variable think they own locker 42. Writing through one changes the other — a "ghost"
  bug where one variable's data mutates because you touched a completely different one.
- **It's a favorite of attackers.** *Double free* is a classic security **exploitation
  primitive**. By carefully controlling what gets stored in the reused locker, an attacker
  can steer the allocator into handing them a pointer to memory they shouldn't control —
  sensitive data, or the program's own control structures — and from there run their own
  code. Many real-world security holes (CVEs) are built on exactly this.
- **The crash comes late, somewhere else.** Step 3 usually doesn't crash on the spot. The
  corruption sits like a landmine and detonates minutes later in unrelated code, so the
  crash points at innocent bystanders and the real cause is nowhere nearby. That delay is
  what makes it one of the hardest bugs to track down.

Its close sibling is **use-after-free**: even before the second free, *reading or writing*
through a ticket whose locker was already handed to someone else touches the wrong data —
same family of danger.

## How Rust makes it impossible
Rust's [ownership](../from-zero/rust/08-ownership-and-moves/use-it.md) rule is designed to
kill this bug at the root: **only one ticket to a locker is ever allowed to exist.** When a
value is *moved* from one variable to another, Rust copies the ticket and **tears up the
original** — the old variable is marked invalid, and using it won't compile. So exactly one
variable can ever return the ticket, and the locker is freed exactly once.

The compiler proves this while it *compiles* your code — no checks run while the program is
executing — so the protection costs nothing at runtime. No garbage collector, no reference
counting. The bug is simply made impossible to write. (Languages like C and C++ leave it up
to the programmer, which is why double-free and use-after-free bugs are so common there.)

## Related
- [The heap](../from-zero/rust/07-the-heap-and-string/under-the-hood.md) — where the
  memory being freed lives.
- [Big-O notation](big-o-notation.md) — why the alternative (always copying data to avoid
  sharing) can be too costly.

## Shows up in
- [From-Zero Concept 08 — Ownership and moves](../from-zero/rust/08-ownership-and-moves/under-the-hood.md)
- [From-Zero Concept 35 — `Arc<Mutex<T>>`](../from-zero/rust/35-arc-mutex/under-the-hood.md)
