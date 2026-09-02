# Interlude 10a · Dereferencing with `*` — following a reference back to its value

> A lettered interlude, but a **memory topic**, so it comes as a pair like the main
> concepts: this **Use it** page (how to write `*` and when you need it) and an
> [**Under the hood**](under-the-hood.md) page (what `*` physically does in memory).
> Track: [From-Zero: Rust](../README.md)

In [Concept 10](../10-borrowing-with-ref/use-it.md) you met `&` — you *borrow* a value
and get a **reference** to it. This interlude is its mirror image: `*`, which takes a
reference and gives you back the **value** it points to. You've already been bumping into
it — `*difference_index` in your Two Sum solution — so let's make it obvious.

## First, untangle two words: *reference* vs *borrow*

These are **not two different things** — they're a noun and a verb for one situation:

- **Borrowing** is the *action*: using a value you don't own, without taking it.
- A **reference** is the *thing* you get for doing it: a `&i32`, which (as the next page
  shows) is really just an address — a note saying "the value lives over there."

So `&x` **borrows** `x` (the verb), and the `&i32` it hands back **is a reference** to
`x` (the noun). "I borrowed `x`" and "I hold a reference to `x`" are the *same moment*
described two ways. Like a library book: *borrowing* is what you did; the *borrowed book*
is what you're holding.

That means `&` and `*` are a matched pair of opposites:

- **`&` borrows** → *makes* a reference (wraps a value: `i32` → `&i32`)
- **`*` dereferences** → *follows* a reference back (unwraps it: `&i32` → `i32`)

![On the left, r (a &i32) holds "ptr" and an arrow labelled "let r = &x builds this arrow" points to x (an i32 box holding 5) on the right. Below, a dashed arrow follows the reference back to a chip reading "*r → 5". Caption: & = make a reference (borrow); * = go where it points and use the value.](diagrams/ampersand-star.svg)

`r` doesn't contain `5`. It contains *where `5` is*. `*r` goes there and reads it.

```rust
let x = 5;
let r = &x;          // borrow x → r is a &i32 (a reference)
println!("{}", *r);  // *r follows the reference → 5
```

## When you actually need `*`

You reach for `*` whenever you're holding a reference but the surrounding code wants the
**owned value**. That's exactly your Two Sum line:

```rust
if let Some(difference_index) = number_bank.get(&current_difference) {
    return vec![*difference_index as i32, index as i32];
    //          ^ difference_index is a &usize; * gets the usize out
}
```

`number_bank.get(...)` returns a *reference* into the map (`Option<&usize>` — the map
still owns its data, so it only lends it). After `if let Some(difference_index)`,
`difference_index` is a `&usize`. But `vec![...]` needs owned numbers, not references —
so `*difference_index` follows the reference and pulls out the `usize`. Without the `*`,
the types wouldn't match (you'd be putting a `&usize` where a number is wanted).

## When you *don't* need it — Rust often derefs for you

Here's the part that makes `*` feel inconsistent until it clicks: sometimes you use a
reference with no `*` at all and it just works. That's because Rust auto-dereferences in
two common spots:

- **Method calls** follow references automatically: `r.to_string()` works even though `r`
  is a `&i32` — Rust quietly derefs to call the method on the value.
- **`println!("{}", r)`** prints `5`, not an address — the formatting machinery derefs
  through the reference for you.

So you only write `*` yourself when nothing is auto-dereffing — most often when you need
the **plain owned value** in an expression, like building that `Vec`. Rule of thumb: *if
the compiler complains it got a `&T` where it wanted a `T`, a `*` is what it's asking
for.*

## The other way to do the same peel: `&` in a pattern

You've seen this trick already. Instead of dereferencing later with `*`, you can strip
the reference right in the pattern with `&`:

```rust
// with * (deref where you use it):
if let Some(difference_index) = map.get(&k) {
    use_it(*difference_index);
}

// with & in the pattern (deref where you bind it):
if let Some(&difference_index) = map.get(&k) {
    use_it(difference_index);   // already a usize here
}
```

Both do the identical peel — one at the *binding*, one at the *use*. It's the same
`&`/`*` pairing showing up in a pattern. (This is why your `for (index, &number)` loop
had that `&`: it peels each `&i32` to an `i32` up front, so the body never needs `*`.)

## Next

- The memory picture beneath all of this: [Under the hood — a reference is an address](under-the-hood.md). Read it next; `*` makes complete sense once you see what a
  reference physically is.
- Back to the main line: [Concept 11 — `&mut` and the borrow rules](../11-mut-references-and-borrow-rules/use-it.md).
- Terse reference: [`*` dereference](../../../languages/rust.md#deref) and
  [`&` borrowing](../../../languages/rust.md#borrow) in the handbook.
