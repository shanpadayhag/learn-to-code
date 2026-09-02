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
- `&mut total_list_head` — [`&mut` reference](../../languages/rust.md#mut-ref): a borrow we
  can *write through*, used here so we can grow the list without owning it; new versus
  Two Sum's read-only [`&`](../../languages/rust.md#ref-pattern).
- `while ... { }` — [`while` loop](../../languages/rust.md#while): loop on a *condition*
  (lists not empty, or carry pending) rather than a fixed count.
- `remaining_first_digits.is_some()` —
  [`Option::is_some`](../../languages/rust.md#is-some): peek at whether a digit
  remains, without consuming it.
- `remaining_first_digits.take()` —
  [`Option::take`](../../languages/rust.md#option-take): pull the node out and leave
  `None` behind, so the variable stays valid for the next pass.
- `total_list_tail.next.as_mut()` — [`Option::as_mut`](../../languages/rust.md#option-as-mut):
  borrow the value *inside* an `Option` without removing it — the keep-it twin of `take`.
- `.unwrap()` — [`.unwrap()`](../../languages/rust.md#unwrap): pull the value out of a
  `Some`, safe here only because we just put one there.

## Already covered
- `impl Solution { ... }` — see [handbook](../../languages/rust.md#impl)
- `pub fn add_two_numbers(...) -> ...` — see [handbook](../../languages/rust.md#pub-fn)
- `let mut ...` — see [handbook](../../languages/rust.md#let-mut)
- `if let Some(first_digit_node) = ...` — see
  [handbook](../../languages/rust.md#if-let); note that here it matches an **owned**
  `Box` moved out by `.take()`, not a borrowed `&` value peeled like in Two Sum, so
  there's no `&` in the pattern.

## Line by line

**The signature.**
```rust
pub fn add_two_numbers(
    first_number_head: Option<Box<ListNode>>,
    second_number_head: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
```
Each argument is an `Option<Box<ListNode>>` — Rust's way of saying "a node on the heap,
or nothing." The [`Box`](../../languages/rust.md#box) is the pointer that holds the
next node; the [`Option`](../../languages/rust.md#if-let) is how Rust expresses "or the
list is empty" *without* a null pointer. We hand the same shape back as the answer.

**The result list, head and tail.**
```rust
let mut total_list_head = Box::new(ListNode::new(0));
let mut total_list_tail = &mut total_list_head;
```
`ListNode::new(0)` builds a node (the judge provides this constructor), and
`Box::new(...)` puts it on the heap, giving a `Box<ListNode>`. This first node is a
throwaway **dummy**: starting with a real node means we never have to special-case
"the list is still empty" when appending — we always have a tail to attach to. We skip
past it at the very end.

`total_list_tail` is a [`&mut`](../../languages/rust.md#mut-ref) borrow of that head — a
*write-through* pointer to "where the next node should go." Its type is
`&mut Box<ListNode>`. Note the two `mut`s doing different jobs: `let mut total_list_tail`
makes the binding *reassignable* (we re-point it every loop), and `&mut` makes it a
reference we can *mutate through*.

**The carry and the two cursors.**
```rust
let mut carried_digit_value = 0;

let mut remaining_first_digits = first_number_head;
let mut remaining_second_digits = second_number_head;
```
`carried_digit_value` is the single `0`-or-`1` we pass between columns.
`remaining_first_digits` and `remaining_second_digits` are our walking positions
through the two input lists; renaming the parameters into `mut` bindings lets us
advance them (`remaining_first_digits = first_digit_node.next`) as we go.

**The loop condition.**
```rust
while remaining_first_digits.is_some()
    || remaining_second_digits.is_some()
    || carried_digit_value != 0
{
```
Keep going while *either* list still has a digit, **or** a carry is still waiting to be
placed. [`.is_some()`](../../languages/rust.md#is-some) just peeks — it asks "is there a
node here?" without taking it — so checking the condition doesn't disturb the lists.
The `carried_digit_value != 0` part is what handles `99 + 1 = 100` growing an extra digit.

**Consuming one digit from each list.**
```rust
    let mut current_digit_sum = carried_digit_value;

    if let Some(first_digit_node) = remaining_first_digits.take() {
        current_digit_sum += first_digit_node.val;
        remaining_first_digits = first_digit_node.next;
    }
```
Start the column's total at the incoming carry. Then
[`.take()`](../../languages/rust.md#option-take) does something subtle: it lifts the
node *out* of `remaining_first_digits` and leaves `None` sitting there. Why not just
`if let Some(first_digit_node) = remaining_first_digits`? Because that would *move*
`remaining_first_digits` away, and Rust would then forbid using it on the next loop.
`.take()` hands us the node to work with while keeping `remaining_first_digits` a
valid variable we immediately reassign:
- `first_digit_node` is the owned `Box<ListNode>` that was inside.
- `first_digit_node.val` reads the digit (auto-dereferencing through the box).
- `first_digit_node.next` is the *rest* of the list, which we store back into
  `remaining_first_digits` to step forward.

If that list was already empty, `.take()` returns `None`, the block is skipped, and the
missing digit simply contributes nothing — exactly the "treat it as 0" rule. The same
five lines repeat for `remaining_second_digits`.

**Writing the digit and advancing the tail.**
```rust
    carried_digit_value = current_digit_sum / 10;
    total_list_tail.next = Some(Box::new(ListNode::new(current_digit_sum % 10)));
    total_list_tail = total_list_tail.next.as_mut().unwrap();
```
`current_digit_sum` is at most `9 + 9 + 1 = 19`, so integer division `/ 10` gives the new carry
(`0` or `1`) and `% 10` (remainder) gives the digit to write. We hang a fresh node off
`total_list_tail.next` — this writes through the `&mut` borrow straight into the real list.

The last line walks the tail to that new node. Read it inside-out:
- `total_list_tail.next` is the `Option<Box<ListNode>>` we just set to `Some(...)`.
- [`.as_mut()`](../../languages/rust.md#option-as-mut) borrows *into* it without taking
  it back out, giving `Option<&mut Box<ListNode>>`.
- [`.unwrap()`](../../languages/rust.md#unwrap) pulls out the `&mut Box<ListNode>` —
  safe because we set it to `Some` one line above.

So `total_list_tail` now points at the node we just appended, ready for the next column.

**Returning the answer.**
```rust
    total_list_head.next
}
```
`total_list_head` is still the dummy node we started with, so the real answer begins at
`total_list_head.next`. Returning it (last expression, no semicolon) hands back the list
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
- `build_digit_list(&[i32]) -> Option<Box<ListNode>>` — the harness thinks in plain vectors
  like `[2, 4, 3]`; the solution thinks in linked nodes. This converts one to the
  other with [`.fold()`](../../languages/rust.md#fold): walking the digits
  [in reverse](../../languages/rust.md#rev) and wrapping the list-so-far as each new
  node's `next` builds the chain from the tail up.
- `collect_digit_values(Option<Box<ListNode>>) -> Vec<i32>` — the way back, using
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
