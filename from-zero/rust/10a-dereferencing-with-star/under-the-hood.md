# Interlude 10a · Dereferencing with `*` — Under the hood

> The memory picture beneath [Use it](use-it.md). Once you see what a reference physically
> *is*, `*` stops being punctuation and becomes obvious.
> Track: [From-Zero: Rust](../README.md)

## A reference is an address

Every value your program makes lives in a **slot** in memory, and every slot has a
number — its **address** (think of houses on a street, each with a number). When you
write `let x = 5;`, the `5` goes into some slot, say number `0x100`.

A reference is nothing mysterious: **it's just that address, stored as a value.**

```rust
let x = 5;       // 5 lives in a slot, say at address 0x100
let r = &x;      // r stores the number 0x100 — "x is over there"
```

`r` does **not** hold `5`. It holds `0x100` — the *location* of `5`. That's the whole
secret of what `&` produces.

![Two memory slots shown with addresses. Slot 0x100 holds "x = 5". Slot 0x108 holds "r = 0x100", annotated "the address of x — not the 5", with an arrow from r's stored address up to the x slot. Below: &x is the address 0x100 ("where x lives"); *r = go to the address in r (0x100), read what's there → 5. Footer: a reference is one address (8 bytes), same size whatever it points to — Concept 03 again.](diagrams/deref-address.svg)

Now the two operators are literally physical actions:

- **`&x`** = "**write down** the address of `x`." (Here: the number `0x100`.)
- **`*r`** = "**go to** the address stored in `r`, and use whatever value is sitting in
  that slot." (Go to `0x100`, find `5`.)

That "go to the address and read it" step has a name: a **load**. It's a single hop —
one jump to a known location — so dereferencing is cheap, O(1). It never searches.

## Why this makes references cheap and fixed-size

Back in [Concept 03](../03-types-have-sizes/use-it.md), every type had a known, fixed
size. A reference's size is **one address** — 8 bytes on a 64-bit machine — *no matter
how big the thing it points at is.*

That's the superpower of borrowing you felt in Concept 10 without seeing why: passing a
100-megabyte value **by reference** copies only its 8-byte address, not the 100 MB. The
function receives a little note saying "it's over there" and reaches through with `*`.
Moving or cloning would haul the whole thing; a reference hands over an address.

> A footnote you'll meet again: a plain `&T` is *one* address (one word). A **slice**
> `&str` / `&[T]` is *two* words — an address **plus** a length ([Concept 12](../12-slices/use-it.md)),
> because "a window into a buffer" needs both where it starts and how long it is. Same
> idea, one extra number. (Trait objects in Concept 21 are the other two-word reference.)

## Predict the memory

```rust
fn main() {
    let x = 10;
    let r = &x;
    let value = *r;
    println!("{}", value + *r);
}
```

Before running it, answer:
1. Does `let r = &x;` move or copy `x`? Can you still use `x` afterward?
2. What does `r` physically hold — `10`, or something else?
3. What does `*r` do at run time?
4. What does it print?

<details>
<summary>Reveal</summary>
<ol>
<li><strong>Neither — it borrows.</strong> <code>&amp;x</code> just reads x's address; <code>x</code> is untouched and still fully usable. (Borrowing never disturbs the owner.)</li>
<li><strong>The address of <code>x</code></strong>, not <code>10</code>. <code>r</code> is a <code>&amp;i32</code> — one machine word holding the location of the slot where <code>10</code> lives.</li>
<li><strong>A load:</strong> it goes to the address in <code>r</code> and reads the value there → <code>10</code>. Since <code>i32</code> is <a href="../06-copy-types/use-it.md"><code>Copy</code></a>, that <code>10</code> is copied out cheaply.</li>
<li><strong><code>20</code>.</strong> <code>value</code> is <code>10</code> (from the first <code>*r</code>), and <code>value + *r</code> is <code>10 + 10</code>.</li>
</ol>
</details>

## The takeaway

`&` and `*` are a round trip through an address: `&` writes the address down, `*` follows
it back to the value. A reference is just "where," and `*` is "go there." Every borrow
you write from here on is this same little note-and-follow dance.

- Back to [Use it](use-it.md) for the when-and-how.
- Next on the main line: [Concept 11 — `&mut` and the borrow rules](../11-mut-references-and-borrow-rules/use-it.md).
