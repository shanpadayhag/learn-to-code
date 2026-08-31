# 2. Add Two Numbers — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). New features are explained in
the [Rust handbook](../../languages/rust.md); already-known ones are linked there.

This is the repo's first **linked-list** solution, so the new syntax is all about
*pointers that might be empty*: `Box` (a heap pointer), `Option` (the "or nothing"
wrapper), and the handful of `Option` methods for getting at — or past — what's inside.

## New here
- `Box<ListNode>` / `Box::new(...)` — [`Box<T>`](../../languages/rust.md#box): a
  heap pointer; it's what lets a node point at the next node without the type becoming
  infinitely large. The list type `Option<Box<ListNode>>` reads as "a pointer to the
  next node, or nothing."
- `&mut result_head` — [`&mut` reference](../../languages/rust.md#mut-ref): a borrow we
  can *write through*, used here so we can grow the list without owning it; new versus
  Two Sum's read-only [`&`](../../languages/rust.md#ref-pattern).
- `while ... { }` — [`while` loop](../../languages/rust.md#while): loop on a *condition*
  (lists not empty, or carry pending) rather than a fixed count.
- `first_digit.is_some()` — [`Option::is_some`](../../languages/rust.md#is-some): peek
  at whether a digit remains, without consuming it.
- `first_digit.take()` — [`Option::take`](../../languages/rust.md#option-take): pull the
  node out and leave `None` behind, so the variable stays valid for the next pass.
- `result_tail.next.as_mut()` — [`Option::as_mut`](../../languages/rust.md#option-as-mut):
  borrow the value *inside* an `Option` without removing it — the keep-it twin of `take`.
- `.unwrap()` — [`.unwrap()`](../../languages/rust.md#unwrap): pull the value out of a
  `Some`, safe here only because we just put one there.

## Already covered
- `impl Solution { ... }` — see [handbook](../../languages/rust.md#impl)
- `pub fn add_two_numbers(...) -> ...` — see [handbook](../../languages/rust.md#pub-fn)
- `let mut ...` — see [handbook](../../languages/rust.md#let-mut)
- `if let Some(node) = ...` — see [handbook](../../languages/rust.md#if-let); note that
  here it matches an **owned** `Box` moved out by `.take()`, not a borrowed `&` value
  peeled like in Two Sum, so there's no `&` in the pattern.

## Line by line

**The signature.**
```rust
pub fn add_two_numbers(
    first_number: Option<Box<ListNode>>,
    second_number: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
```
Each argument is an `Option<Box<ListNode>>` — Rust's way of saying "a node on the heap,
or nothing." The [`Box`](../../languages/rust.md#box) is the pointer that holds the
next node; the [`Option`](../../languages/rust.md#if-let) is how Rust expresses "or the
list is empty" *without* a null pointer. We hand the same shape back as the answer.

**The result list, head and tail.**
```rust
let mut result_head = Box::new(ListNode::new(0));
let mut result_tail = &mut result_head;
```
`ListNode::new(0)` builds a node (the judge provides this constructor), and
`Box::new(...)` puts it on the heap, giving a `Box<ListNode>`. This first node is a
throwaway **dummy**: starting with a real node means we never have to special-case
"the list is still empty" when appending — we always have a tail to attach to. We skip
past it at the very end.

`result_tail` is a [`&mut`](../../languages/rust.md#mut-ref) borrow of that head — a
*write-through* pointer to "where the next node should go." Its type is
`&mut Box<ListNode>`. Note the two `mut`s doing different jobs: `let mut result_tail`
makes the binding *reassignable* (we re-point it every loop), and `&mut` makes it a
reference we can *mutate through*.

**The carry and the two cursors.**
```rust
let mut carry = 0;

let mut first_digit = first_number;
let mut second_digit = second_number;
```
`carry` is the single `0`-or-`1` we pass between columns. `first_digit` and
`second_digit` are our walking positions through the two input lists; renaming the
parameters into `mut` bindings lets us advance them (`first_digit = node.next`) as we
go.

**The loop condition.**
```rust
while first_digit.is_some() || second_digit.is_some() || carry != 0 {
```
Keep going while *either* list still has a digit, **or** a carry is still waiting to be
placed. [`.is_some()`](../../languages/rust.md#is-some) just peeks — it asks "is there a
node here?" without taking it — so checking the condition doesn't disturb the lists.
The `carry != 0` part is what handles `99 + 1 = 100` growing an extra digit.

**Consuming one digit from each list.**
```rust
    let mut digit_sum = carry;

    if let Some(node) = first_digit.take() {
        digit_sum += node.val;
        first_digit = node.next;
    }
```
Start the column's total at the incoming carry. Then [`.take()`](../../languages/rust.md#option-take)
does something subtle: it lifts the node *out* of `first_digit` and leaves `None`
sitting there. Why not just `if let Some(node) = first_digit`? Because that would
*move* `first_digit` away, and Rust would then forbid using it on the next loop. `.take()`
hands us the node to work with while keeping `first_digit` a valid variable we
immediately reassign:
- `node` is the owned `Box<ListNode>` that was inside.
- `node.val` reads the digit (auto-dereferencing through the box).
- `node.next` is the *rest* of the list, which we store back into `first_digit` to step
  forward.

If that list was already empty, `.take()` returns `None`, the block is skipped, and the
missing digit simply contributes nothing — exactly the "treat it as 0" rule. The same
five lines repeat for `second_digit`.

**Writing the digit and advancing the tail.**
```rust
    carry = digit_sum / 10;
    result_tail.next = Some(Box::new(ListNode::new(digit_sum % 10)));
    result_tail = result_tail.next.as_mut().unwrap();
```
`digit_sum` is at most `9 + 9 + 1 = 19`, so integer division `/ 10` gives the new carry
(`0` or `1`) and `% 10` (remainder) gives the digit to write. We hang a fresh node off
`result_tail.next` — this writes through the `&mut` borrow straight into the real list.

The last line walks the tail to that new node. Read it inside-out:
- `result_tail.next` is the `Option<Box<ListNode>>` we just set to `Some(...)`.
- [`.as_mut()`](../../languages/rust.md#option-as-mut) borrows *into* it without taking
  it back out, giving `Option<&mut Box<ListNode>>`.
- [`.unwrap()`](../../languages/rust.md#unwrap) pulls out the `&mut Box<ListNode>` —
  safe because we set it to `Some` one line above.

So `result_tail` now points at the node we just appended, ready for the next column.

**Returning the answer.**
```rust
    result_head.next
}
```
`result_head` is still the dummy node we started with, so the real answer begins at
`result_head.next`. Returning it (last expression, no semicolon) hands back the list
from the first *real* digit onward and quietly drops the dummy.

## Running it
```
rustc solution.rs && ./solution
```

This problem hides more than most behind LeetCode's editor: not just the
[`Solution`](../../languages/rust.md#unit-struct) type and
[`main`](../../languages/rust.md#main), but `ListNode` itself. The file writes all of
it out, with `ListNode` copied field-for-field from LeetCode's own definition — same
`val`, same `next`, same `new` constructor — so the `impl Solution` block still
compiles unchanged on their side.

- `#[derive(PartialEq, Eq, Clone, Debug)]` — the derives LeetCode puts on `ListNode`.
  They ask the compiler to write the "compare two nodes" and "print a node" code for
  us instead of implementing it by hand.
- `build_list(&[i32]) -> Option<Box<ListNode>>` — the harness thinks in plain vectors
  like `[2, 4, 3]`; the solution thinks in linked nodes. This converts one to the
  other with [`.fold()`](../../languages/rust.md#fold): walking the digits
  [in reverse](../../languages/rust.md#rev) and wrapping the list-so-far as each new
  node's `next` builds the chain from the tail up.
- `collect_digits(Option<Box<ListNode>>) -> Vec<i32>` — the way back, using
  [`while let`](../../languages/rust.md#while-let): keep pulling nodes while there's
  still a `Some`, push each `val`, and step to `next`. The loop ends by itself at the
  `None` that marks the end of the list.
- Both converters live *outside* the submit block on purpose — they're harness
  plumbing, and pasting them into LeetCode would be pointless (their judge does the
  same conversion for you).
- `check(...)` asserts with [`assert_eq!`](../../languages/rust.md#assert-eq) on the
  digit vectors, then prints the sum as `[2, 4, 3] + [5, 6, 4] = [7, 0, 8]`. The cases
  are the LeetCode examples plus the two carry traps: `5 + 5` growing a new digit, and
  `1 + 999` where the carry ripples the whole way and past the shorter list's end.
