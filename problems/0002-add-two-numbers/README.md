# 2. Add Two Numbers

| | |
|---|---|
| Difficulty | Medium |
| Languages  | Rust |
| Pattern    | [Linked List](../../glossary/linked-list.md) |
| Time/Space | O(max(m, n)) / O(max(m, n)) |
| Source     | [LeetCode](https://leetcode.com/problems/add-two-numbers/) |

## The Problem
You're given two numbers, but not as plain numbers — each one is handed to you as a
[linked list](../../glossary/linked-list.md), one digit per node, with the digits
stored **backwards** (the ones digit first). Add the two numbers and return the sum
in the same backwards-linked-list form.

Each node holds one digit (0–9) and a pointer to the next node:

```
l1:  2 -> 4 -> 3 -> ∅      reads backwards as 342
l2:  5 -> 6 -> 4 -> ∅      reads backwards as 465
```

What matters:
- The digits are in **reverse** order — the first node is the *ones* place, the next
  is the *tens*, and so on. (This is friendlier than it sounds: it's the exact order
  you add in by hand, right to left.)
- Each list has between 1 and 100 nodes, so the numbers can be up to 100 digits long
  — far bigger than any built-in integer type can hold.
- Neither input has leading zeros (except the number `0` itself, which is a single
  `0` node).

Tiny example:
```
342 + 465 = 807
answer:  7 -> 0 -> 8 -> ∅
```

## Understand It

### In plain words
This is the addition you learned in grade school, with one friendly twist.

Picture adding `342 + 465` on paper. You line them up and work the columns **right to
left**: ones first, then tens, then hundreds, carrying a `1` whenever a column spills
over ten. Now the twist: here the digits arrive already reversed — the ones come
first. So the order you're *given* the digits is exactly the order you *add* them. No
need to flip anything; just walk forward through both lists and add column by column.

### The slow, obvious way
The tempting shortcut: turn each list back into a normal number, add them with `+`,
then chop the answer back into digits. Walk it by hand for `[2,4,3]` and `[5,6,4]`:

| Step | What you do | Result |
|---|---|---|
| Rebuild list 1 | `2`, then `4` in the tens, then `3` in the hundreds | `342` |
| Rebuild list 2 | `5`, then `6` in the tens, then `4` in the hundreds | `465` |
| Add | `342 + 465` | `807` |
| Split back into digits | `7`, `0`, `8` (reversed) | `[7, 0, 8]` |

On this tiny example it works perfectly. But re-read the constraints: each list can be
**100 nodes** long. A 100-digit number doesn't fit in a 64-bit integer (those top out
around 19 digits) — the moment you try to "rebuild" it, the value overflows and the
math comes out wrong. So this approach isn't just slow, it's *broken* for the inputs
the problem actually allows. The fix isn't a bigger number type — it's to never build
the whole number at all.

### The trick
**You never need the whole number — only one column at a time.**

Grade-school addition already works this way: to add a column you only need its two
digits and whatever carry came from the column before. You write down a single digit
and pass along a carry of `0` or `1`. You never have to know the full number to get
the next digit right.

So walk both lists forward together, one node per column:

1. Add the two current digits plus the carry coming in.
2. The digit you write is that total's **ones place** (`sum % 10`).
3. The carry you pass on is whether it spilled over ten (`sum / 10`, always `0` or `1`).
4. Append that one digit to the answer list and step both lists forward.

This leans on what a [linked list](../../glossary/linked-list.md) is good at: stepping
to the next node and tacking a new node onto the end are each a single, cheap move, so
one column costs a fixed amount of work regardless of how long the numbers are. That's
what lets a 100-digit sum cost the same per digit as a 1-digit sum — and why it sails
past the overflow that sinks the rebuild approach. (For what "cost per digit" means,
see [Big-O notation](../../glossary/big-o-notation.md).)

Two loose ends, both handled by *when* you stop:
- **The lists can be different lengths.** When one runs out, treat its missing digits
  as `0` and keep going with the other.
- **The carry can outlive both lists.** `99 + 1 = 100` grows a new digit. So you keep
  looping while *either* list has digits left **or** there's still a carry to place.

### Watch it run
Add `[2,4,3]` (342) and `[5,6,4]` (465). Walk the columns left to right, carrying as
you go:

| Column | digit from l1 | digit from l2 | carry in | sum | write (`sum % 10`) | carry out (`sum / 10`) |
|---|---|---|---|---|---|---|
| 1 (ones) | 2 | 5 | 0 | 7 | **7** | 0 |
| 2 (tens) | 4 | 6 | 0 | 10 | **0** | 1 |
| 3 (hundreds) | 3 | 4 | 1 | 8 | **8** | 0 |

The answer list grows one node per column:

```
after column 1:  7 -> ∅
after column 2:  7 -> 0 -> ∅
after column 3:  7 -> 0 -> 8 -> ∅
```

Now the carry-outlives-the-lists case, `[9,9]` (99) + `[1]` (1):

| Column | l1 | l2 | carry in | sum | write | carry out |
|---|---|---|---|---|---|---|
| 1 | 9 | 1 | 0 | 10 | **0** | 1 |
| 2 | 9 | — (gone, use 0) | 1 | 10 | **0** | 1 |
| 3 | — | — | 1 | 1 | **1** | 0 |

Both lists are empty after column 2, but the carry is still `1`, so the loop runs once
more and places the final `1`. Result: `0 -> 0 -> 1`, which reads backwards as `100`. ✓

### The answer
For the first example the result is `7 -> 0 -> 8`, i.e. `807`, and `342 + 465 = 807`.

Why is this always correct? Each column reproduces exactly one step of hand addition —
the same digit and the same carry you'd write yourself — and the loop only stops once
both lists are exhausted *and* no carry is left waiting. Nothing to add and nothing
carried means there are no more digits to produce, so no digit is ever dropped or
invented.

## The Code
### Rust

> LeetCode hands you a `ListNode` type: a struct with a digit `val: i32` and a
> `next` pointer to the following node (or nothing, at the end of the list). The whole
> list is passed as that "node or nothing" — see the [syntax notes](solution.rs.md)
> for exactly how Rust spells "or nothing" and a heap pointer.

```rust
impl Solution {
    pub fn add_two_numbers(
        first_number_head: Option<Box<ListNode>>,
        second_number_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut total_list_head = Box::new(ListNode::new(0));
        let mut total_list_tail = &mut total_list_head;
        let mut carried_digit_value = 0;

        let mut remaining_first_digits = first_number_head;
        let mut remaining_second_digits = second_number_head;

        while remaining_first_digits.is_some()
            || remaining_second_digits.is_some()
            || carried_digit_value != 0
        {
            let mut current_digit_sum = carried_digit_value;

            if let Some(first_digit_node) = remaining_first_digits.take() {
                current_digit_sum += first_digit_node.val;
                remaining_first_digits = first_digit_node.next;
            }
            if let Some(second_digit_node) = remaining_second_digits.take() {
                current_digit_sum += second_digit_node.val;
                remaining_second_digits = second_digit_node.next;
            }

            carried_digit_value = current_digit_sum / 10;
            total_list_tail.next = Some(Box::new(ListNode::new(current_digit_sum % 10)));
            total_list_tail = total_list_tail.next.as_mut().unwrap();
        }

        total_list_head.next
    }
}
```

**Time:** O(max(m, n)) — one pass, doing a fixed amount of work per column, where `m`
and `n` are the two lengths.   **Space:** O(max(m, n)) — the answer list holds one node
per column, at most `max(m, n) + 1` of them; aside from that the work uses a fixed
amount of extra memory.
**Run:** `rustc solution.rs && ./solution` — [`solution.rs`](solution.rs) wraps the
block above in the scaffolding LeetCode hides (`struct Solution;` and `ListNode`) plus a harness
that asserts every example.
**Syntax notes:** [solution.rs.md](solution.rs.md)

## Remember This
When data arrives as a [linked list](../../glossary/linked-list.md), the natural shape
of a solution is **one forward pass, building the result node by node** — keep a
pointer to the tail so each new node is appended in a single step. And when a problem
is really "do this operation the way you'd do it by hand," reach for **simulation**:
carry exactly the small state each step needs (here, a single carry digit) instead of
trying to compute the whole thing at once. That column-at-a-time, carry-as-you-go idea
shows up well beyond addition — multiplying big numbers, merging sorted lists, and any
running-total walk.
